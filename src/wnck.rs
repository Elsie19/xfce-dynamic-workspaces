use std::ffi::CStr;

use wnck_sys::{
    wnck_screen_force_update, wnck_screen_get_active_workspace, wnck_screen_get_default, wnck_screen_get_windows, wnck_screen_get_workspaces, wnck_window_get_class_instance_name, wnck_window_get_name, wnck_window_get_role, wnck_window_get_workspace, wnck_window_is_on_workspace, wnck_window_is_sticky, wnck_window_move_to_workspace, wnck_workspace_get_number, WnckScreen, WnckWindow, WnckWorkspace
};

pub struct Screen {
    pub screen: *mut WnckScreen,
}

impl Screen {
    /// Get default screen handle.
    pub fn get_default() -> Self {
        Self {
            screen: unsafe { wnck_screen_get_default() },
        }
    }

    pub fn force_update(&self) {
        unsafe { wnck_screen_force_update(self.screen); }
    }

    pub fn get_active_workspace(&self) -> Option<Workspace> {
        let ptr = unsafe { wnck_screen_get_active_workspace(self.screen) };
        if ptr.is_null() {
            None
        } else {
            Some(Workspace { workspace: ptr })
        }
    }

    pub fn get_workspaces(&self) -> Vec<Workspace> {
        let mut out = vec![];

        unsafe {
            let mut list = wnck_screen_get_workspaces(self.screen);
            while !list.is_null() {
                let node = &*list;
                let workspace_ptr = node.data.cast::<WnckWorkspace>();
                out.push(Workspace {
                    workspace: workspace_ptr,
                });
                list = node.next;
            }
        }

        out
    }

    pub fn get_windows(&self) -> Vec<Window> {
        let mut out = vec![];
        unsafe {
            let mut list = wnck_screen_get_windows(self.screen);
            while !list.is_null() {
                let node = &*list;
                let workspace_str = node.data.cast::<WnckWindow>();
                out.push(Window {
                    window: workspace_str,
                });
                list = node.next;
            }
        }

        out
    }
}

#[derive(PartialEq)]
pub struct Workspace {
    workspace: *mut WnckWorkspace,
}

impl PartialEq<&Workspace> for Workspace {
    fn eq(&self, other: &&Workspace) -> bool {
        self.workspace == other.workspace
    }
}

impl Workspace {
    pub fn get_number(&self) -> i32 {
        unsafe { wnck_workspace_get_number(self.workspace) }
    }
}

#[derive(PartialEq, Eq)]
pub struct Window {
    window: *mut WnckWindow,
}

// TODO: Check that this works and doesn't crash.
impl Clone for Window {
    fn clone(&self) -> Self {
        Self {
            window: self.window,
        }
    }
}

impl Window {
    pub fn is_on_workspace(&self, workspace: &Workspace) -> bool {
        unsafe { wnck_window_is_on_workspace(self.window, workspace.workspace) != 0 }
    }

    pub fn is_sticky(&self) -> bool {
        unsafe { wnck_window_is_sticky(self.window) != 0 }
    }

    pub fn get_name(&self) -> String {
        let c_str = unsafe { wnck_window_get_name(self.window) };
        unsafe { CStr::from_ptr(c_str).to_string_lossy().to_string() }
    }

    pub fn get_class_instance_name(&self) -> String {
        let c_str = unsafe { wnck_window_get_class_instance_name(self.window) };
        unsafe { CStr::from_ptr(c_str).to_string_lossy().to_string() }
    }

    pub fn get_role(&self) -> String {
        let c_str = unsafe { wnck_window_get_role(self.window) };
        unsafe { CStr::from_ptr(c_str).to_string_lossy().to_string() }
    }

    pub fn move_to_workspace(&self, workspace: &Workspace) {
        unsafe {
            wnck_window_move_to_workspace(self.window, workspace.workspace);
        }
    }

    pub fn get_workspace(&self) -> Option<Workspace> {
        let ptr = unsafe { wnck_window_get_workspace(self.window) };
        if ptr.is_null() {
            None
        } else {
            Some(Workspace { workspace: ptr })
        }
    }
}
