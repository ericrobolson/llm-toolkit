mod response;
pub use response::*;

use std::{
    net::TcpStream,
    path::PathBuf,
    process::Command,
    sync::mpsc::{Receiver, Sender, channel},
    thread::JoinHandle,
};

pub struct LlamaServer {
    handle: Option<JoinHandle<()>>,
}
impl LlamaServer {
    pub fn new(path: PathBuf, model_name: String) -> Self {
        let handle = std::thread::spawn(move || {
            start_server(&path, &model_name);
        });

        Self {
            handle: Some(handle),
        }
    }

    pub fn address(&self) -> String {
        "http://127.0.0.1:8080".to_string()
    }

    pub fn ask(&self, prompt: &str) -> Response {
        let response = Response::begin(&self.address().replace("http://", ""), prompt);
        response
    }
}

fn start_server(path: &PathBuf, model_name: &str) {
    println!("Starting server...");
    println!("http://127.0.0.1:8080");
    let repo_path = path.join("llama.cpp");

    let models_path = path.join("models");
    let model = models_path.join(model_name);
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
