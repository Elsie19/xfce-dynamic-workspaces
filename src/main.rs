mod wnck;

struct DynamicWorkspaces {
    debug: bool,
    notify: bool,
    window_blacklist: Vec<String>,
    window_classrole_blacklist: Vec<String>,
    last: usize,
    screen: (),
}

fn main() {
    println!("Hello, world!");
}
