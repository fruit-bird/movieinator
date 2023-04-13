use std::process::Command;

fn main() {
    // TODO: Add terminal completions script generation
    let output = Command::new("zsh")
        .arg("setup.sh")
        .output()
        .expect("Failed to execute the setup script");

    // this output is redirected to `target/debug/build/movienator-{random}/output`
    if output.status.success() {
        println!("Script executed successfully");
    } else {
        println!("Script failed with exit code: {}", output.status);
        println!("Script output: {}", String::from_utf8_lossy(&output.stderr));
    }
}
