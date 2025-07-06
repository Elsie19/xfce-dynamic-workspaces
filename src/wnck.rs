use std::ffi::CStr;

use wnck_sys::{
    WnckScreen, WnckWindow, WnckWorkspace, wnck_screen_get_active_workspace,
    wnck_screen_get_default, wnck_screen_get_windows, wnck_screen_get_workspaces,
    wnck_window_get_class_instance_name, wnck_window_get_name, wnck_window_get_role,
    wnck_window_is_on_workspace, wnck_window_is_sticky, wnck_window_move_to_workspace,
    wnck_workspace_get_number,
};

pub struct Screen {
    screen: *mut WnckScreen,
}

impl Screen {
    /// Get default screen handle.
    pub fn get_default() -> Self {
        Self {
            screen: unsafe { wnck_screen_get_default() },
        }
    }

    pub fn get_active_workspace(&self) -> Workspace {
        Workspace {
            workspace: unsafe { wnck_screen_get_active_workspace(self.screen) },
        }
    }

    pub fn get_workspaces(&self) -> Vec<Workspace> {
        let mut out = vec![];
        unsafe {
            let mut list = wnck_screen_get_workspaces(self.screen);
            while !list.is_null() {
                let node = &*list;
                let workspace_ptr = node.data as *mut WnckWorkspace;
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
                let workspace_str = node.data as *mut WnckWindow;
                out.push(Window {
                    window: workspace_str,
                });
                list = node.next;
            }
        }

        out
    }
}

pub struct Workspace {
    workspace: *mut WnckWorkspace,
}

impl Workspace {
    pub fn get_number(&self) -> i32 {
        unsafe { wnck_workspace_get_number(self.workspace) }
    }
}

pub struct Window {
    window: *mut WnckWindow,
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
}
