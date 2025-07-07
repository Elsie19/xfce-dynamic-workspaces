use std::time::Duration;

use notify_rust::Notification;

pub fn default_notification() {
    let _ = Notification::new()
        .summary("Workspace Switch Notifier")
        .timeout(Duration::from_secs(2))
        .show();
}

pub fn update_notification(workspace: i32) {
    let _ = Notification::new()
        .summary("Workspace Switch Notifier")
        .body(&format!("Workspace {workspace}"))
        .timeout(Duration::from_secs(2))
        .show();
}
