use std::{
    ffi::{CString, c_char, c_int, c_void},
    time::Duration,
};

use wmctrl::desktop;
use wnck::{Screen, Window};

mod wnck;

struct DynamicWorkspaces {
    debug: bool,
    notify: bool,
    window_blacklist: Vec<String>,
    window_classrole_blacklist: Vec<String>,
    last: usize,
    screen: Screen,
}

impl DynamicWorkspaces {
    pub fn new(debug: bool, notify: bool) -> Self {
        Self {
            debug,
            notify,
            window_blacklist: vec![
                String::from("Skriveboard"),
                String::from("Desktop"),
                String::from("xfdashboard"),
                String::from("xfce4-panel"),
                String::from("plank"),
                String::from("xfce4-notifyd"),
                String::from("Whisker Menu"),
            ],
            window_classrole_blacklist: vec![String::from("tilix.quake")],
            last: 0,
            screen: Screen::get_default(),
        }
    }

    // TODO: Add libnotify.
    pub fn update_notification(&self) {}

    /// Main logic for handling of dynamic workspaces
    pub fn handle_dynamic_workspaces(&mut self) {
        // Gets the current workspaces
        let workspaces = self.screen.get_workspaces();
        let workspaces_len = workspaces.len();

        eprintln!("Workspaces: {workspaces_len}");

        // Initiates necessary scope variables and counts the windows on the relevant workspaces
        if !workspaces.is_empty() {
            let mut last = 0;
            let mut next_last = 0;
            // Removes blacklisted windows from the list of visible windows
            let windows = self.remove_blacklist(&mut self.screen.get_windows());

            // Counts windows
            for window in windows {
                // Checks if the window is on the last workspace
                if window.is_on_workspace(&workspaces[workspaces.len() - 1]) {
                    last += 1;
                }
                if workspaces_len > 1 {
                    // Checks if the window is on the workspace before the last
                    if window.is_on_workspace(&workspaces[workspaces.len() - 2]) {
                        next_last += 1;
                    }
                }
            }

            // Main logical operations for removing last/last two workspaces
            if last > 0 {
                self.add_workspace(workspaces_len);
            }
            if workspaces_len > 1 && last == 0 && next_last == 0 {
                self.pop_workspace(workspaces_len);
            }
        }

        let workspaces = self.screen.get_workspaces();
        let workspaces_len = workspaces.len();

        if workspaces_len > 2 {
            let windows = self.remove_blacklist(&mut self.screen.get_windows());
            for (idx, workspace) in workspaces
                .iter()
                .take(workspaces.len().saturating_sub(1))
                .enumerate()
            {
                if self.screen.get_active_workspace().as_ref() != Some(workspace)
                    && self.screen.get_workspaces().last() != Some(workspace)
                {
                    let mut workspace_empty = true;
                    for window in &windows {
                        if window.is_on_workspace(workspace) {
                            workspace_empty = false;
                            break;
                        }
                    }
                    if workspace_empty {
                        let workspaces = self.screen.get_workspaces();
                        if let Some(last_workspace) = workspaces.last() {
                            if workspace != last_workspace {
                                self.remove_workspace_by_index(idx);
                            }
                        }
                    }
                }
            }
        }

        if let Some(workspace) = self.screen.get_active_workspace() {
            self.last = workspace.get_number() as usize;
        }
    }

    pub fn remove_blacklist(&self, windows: &mut Vec<Window>) -> Vec<Window> {
        let mut i = 0;
        while windows.len() > i {
            if windows[i].is_sticky() {
                windows.remove(i);
                i -= 1;
            } else if self.window_blacklist.contains(&windows[i].get_name()) {
                windows.remove(i);
                i -= 1;
            } else if !windows[i].get_role().is_empty() {
                if self.window_classrole_blacklist.contains(&format!(
                    "{}.{}",
                    windows[i].get_class_instance_name(),
                    windows[i].get_role()
                )) {
                    windows.remove(i);
                    i -= 1;
                }
            }
            i += 1;
        }

        windows.to_vec()
    }

    pub fn add_workspace(&self, workspaces_len: usize) {
        let _ = desktop::set_desktop_count((workspaces_len + 1).try_into().unwrap());
    }

    pub fn pop_workspace(&self, workspaces_len: usize) {
        if self.screen.get_workspaces().len() > 2 {
            let _ = desktop::set_desktop_count((workspaces_len - 1).try_into().unwrap());
        }
    }

    pub fn remove_workspace_by_index(&self, index: usize) {
        let workspace_num = self.screen.get_active_workspace().map(|ws| ws.get_number());
        let workspaces = String::from_utf8(desktop::list_desktops().stdout)
            .expect("not valid UTF-8")
            .lines()
            .collect::<Vec<_>>()
            .len();

        let windows: Vec<Window> = self
            .screen
            .get_windows()
            .into_iter()
            .filter(|window| {
                if let Some(ws) = window.get_workspace() {
                    ws.get_number() > index as i32
                } else {
                    false
                }
            })
            .collect();

        for window in windows {
            if let Some(workspace) = window.get_workspace() {
                window.move_to_workspace(
                    &self.screen.get_workspaces()[workspace.get_number() as usize - 1],
                );
            }
        }
        self.pop_workspace(workspaces);

        // workspace_num should be Option<usize>
        if let Some(workspace_num) = workspace_num {
            if self.last < workspace_num as usize {
                let _ = desktop::switch_desktop(index.to_string().as_str());
            }
        }
    }

    pub fn connect_signals(&mut self) {
        let _ = desktop::set_desktop_count(1);

        let signals = [
            "active-workspace-changed",
            "workspace-created",
            "workspace-destroyed",
            "window-opened",
            "window-closed",
        ];

        loop {
            self.handle_dynamic_workspaces();
            std::thread::sleep(Duration::from_secs(5));
        }
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let mut debug = false;
    let mut notify = true;

    for arg in &args {
        if arg == "--debug" {
            println!("Debug mode enabled");
            debug = true;
        } else if arg == "--no-notify" {
            println!("Notifications disabled");
            notify = false;
        }
    }

    let cstrings: Vec<CString> = std::env::args()
        .map(|arg| CString::new(arg).unwrap())
        .collect();

    let mut c_args: Vec<*mut c_char> = cstrings
        .iter()
        .map(|cstr| cstr.as_ptr().cast_mut())
        .collect();

    let mut argc: c_int = c_args.len() as c_int;
    let mut argv_ptr: *mut *mut c_char = c_args.as_mut_ptr();

    unsafe {
        if gdk_sys::gdk_init_check(&raw mut argc, &raw mut argv_ptr) == 0 {
            eprintln!("`gdk_init_check` failed to start");
            std::process::exit(1);
        }
    }

    println!("Started workspace indicator");
    let mut workspaces = DynamicWorkspaces::new(debug, notify);
    workspaces.connect_signals();
}
