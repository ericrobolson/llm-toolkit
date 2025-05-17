use std::{path::PathBuf, process::Command};

pub fn execute(path: &PathBuf) {
    let repo_path = path.join("llama.cpp");
    if !repo_path.exists() {
        println!("Cloning repo...");
        // let command = "git clone https://github.com/ggerganov/llama.cpp.git";

        let command = Command::new("git")
            .args(&["clone", "https://github.com/ggerganov/llama.cpp.git"])
            .current_dir(&path)
            .output()
            .unwrap();

        if command.status.success() {
            println!("Repo cloned successfully");
        } else {
            panic!(
                "Failed to clone repo: {}",
                String::from_utf8(command.stderr).unwrap()
            );
        }

        // let command = "cd llama.cpp && cmake -B build && cmake --build build --config Release";

        let command = Command::new("cmake")
            .args(&["-B", "build"])
            .current_dir(&repo_path)
            .output()
            .unwrap();

        if command.status.success() {
            println!("CMake build successful");
        } else {
            panic!(
                "CMake build failed: {}",
                String::from_utf8(command.stderr).unwrap()
            );
        }

        println!("Building...");
        let command = Command::new("cmake")
            .args(&["--build", "build", "--config", "Release"])
            .current_dir(&repo_path)
            .output()
            .unwrap();

        if command.status.success() {
            println!("CMake build successful");
        } else {
            panic!(
                "CMake build failed: {}",
                String::from_utf8(command.stderr).unwrap()
            );
        }
    } else {
        println!("Repo exists")
    }
}
