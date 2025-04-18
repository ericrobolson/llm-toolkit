use rusqlite::Connection;
use std::{path::PathBuf, process::Command};

fn main() {
    let dir: PathBuf = ".llama-tk/".into();
    std::fs::create_dir_all(&dir).unwrap();

    setup_db(&dir);
    setup_repo(&dir);
    source_models(&dir);
    start_server(&dir);
}

fn setup_db(path: &PathBuf) {
    let db_path = path.join("llama-db.sqlite");
    let db = Connection::open(db_path).unwrap();

    db.execute(
        "CREATE TABLE IF NOT EXISTS files (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            path TEXT NOT NULL
        )",
        (),
    )
    .unwrap();

    // List out all tables
    let mut stmt = db
        .prepare("SELECT name FROM sqlite_master WHERE type='table'")
        .unwrap();

    let mut rows = stmt.query([]).unwrap();

    let mut names: Vec<String> = Vec::new();
    while let Some(row) = rows.next().unwrap() {
        names.push(row.get(0).unwrap());
    }

    for table in names {
        println!("{}", table);
    }
}

fn setup_repo(path: &PathBuf) {
    let repo_path = path.join("llama.cpp");
    if !repo_path.exists() {
        println!("Cloning repo...");
        let command = "git clone https://github.com/ggerganov/llama.cpp.git";

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

        let command = "cd llama.cpp && cmake -B build && cmake --build build --config Release";

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

fn start_server(path: &PathBuf) {
    println!("Starting server...");
    println!("http://127.0.0.1:8080");
    let repo_path = path.join("llama.cpp");

    let models_path = path.join("models");
    let model = models_path.join("openhermes-2.5-mistral-7b.Q5_K_S.gguf");
    let bin_path = repo_path.join("build/bin/llama-server");
    let command = Command::new(bin_path)
        .args(&["--port", "8080"])
        .args(&["--model", model.to_str().unwrap()])
        .output()
        .unwrap();

    if command.status.success() {
        println!("Server started");
    } else {
        panic!(
            "Server failed to start: {}",
            String::from_utf8(command.stderr).unwrap()
        );
    }
}

fn source_models(path: &PathBuf) {
    let models_path = path.join("models");
    std::fs::create_dir_all(&models_path).unwrap();
    let model = "openhermes-2.5-mistral-7b.Q5_K_S.gguf";
    let url = "https://huggingface.co/TheBloke/OpenHermes-2.5-Mistral-7B-GGUF/resolve/main/openhermes-2.5-mistral-7b.Q5_K_S.gguf";
    let model_path = models_path.join(model);
    if !model_path.exists() {
        /*
                URL=https://huggingface.co/TheBloke/OpenHermes-2.5-Mistral-7B-GGUF/resolve/main/openhermes-2.5-mistral-7b.Q5_K_S.gguf

        mkdir -p models
        curl -L $URL > models/openhermes-2.5-mistral-7b.Q5_K_S.gguf
                 */

        println!("Downloading model...");
        let command = Command::new("curl")
            .args(&["-L", url])
            .args(&["-O"])
            .current_dir(&models_path)
            .output()
            .unwrap();

        if command.status.success() {
            println!("Model downloaded");
        } else {
            panic!(
                "Model failed to download: {}",
                String::from_utf8(command.stderr).unwrap()
            );
        }
    }
}
