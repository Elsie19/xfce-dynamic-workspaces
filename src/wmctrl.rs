//! A lean copy of <https://github.com/Treborium/rust-wmctrl> without regex.

use std::process::{Command, Output};

pub fn set_desktop_count(length: usize) -> Output {
    Command::new("wmctrl")
        .args(["-n", length.to_string().as_str()])
        .output()
        .unwrap_or_else(|_| panic!("Failed to execute `wmctrl`"))
}

pub fn list_desktops() -> Output {
    Command::new("wmctrl")
        .arg("-d")
        .output()
        .unwrap_or_else(|_| panic!("Failed to execute `wmctrl`"))
}

pub fn switch_desktop<S: AsRef<str>>(desktop: S) -> Output {
    Command::new("wmctrl")
        .args(["-s", desktop.as_ref()])
        .output()
        .unwrap_or_else(|_| panic!("Failed to execute `wmctrl`"))
}
