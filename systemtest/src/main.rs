// This test script starts the processes that will be used to test the distributed network.
// Currently, it only starts the conductor process.

fn main() {
    let binary_path = std::env::current_exe().expect("failed to get current executable path");
    let conductor_path = binary_path
        .parent()
        .expect("failed to get parent directory")
        .join("conductor");
    let _node_path = binary_path
        .parent()
        .expect("failed to get parent directory")
        .join("node");

    let child = std::process::Command::new(conductor_path)
        .spawn()
        .expect("failed to start conductor");

    let output = child.wait_with_output().expect("failed to wait on child");

    if output.status.success() {
        println!("conductor exited successfully");
    } else {
        println!("conductor exited with: {}", output.status);
    }
}
