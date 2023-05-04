use std::process::Command;

/// initialize weaviate test server
pub fn setup() {
    let mut cwd = std::env::current_dir().expect("cwd error");
    cwd.extend(["tests", "common"].iter());

    let mut command = Command::new("docker");
    command.args(["compose", "up", "-d"]).current_dir(cwd);

    if let Ok(mut child) = command.spawn() {
        child.wait().expect("command wasn't running");
        println!("Child has finished its execution!");
    } else {
        println!("docker command didn't start");
    }
}

/// shut down weaviate test server
pub async fn cleanup() {
    let mut cwd = std::env::current_dir().expect("cwd error");
    cwd.extend(["tests", "common"].iter());

    let mut command = Command::new("docker");
    command.args(["compose", "down"]).current_dir(cwd);

    if let Ok(mut child) = command.spawn() {
        child.wait().expect("command wasn't running");
        println!("Child has finished its execution!");
    } else {
        println!("docker command didn't finish");
    }
}
