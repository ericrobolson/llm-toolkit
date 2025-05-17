mod response;
pub use response::*;

use std::{
    path::PathBuf,
    process::{Child, Command},
    sync::mpsc::Sender,
    thread::JoinHandle,
};

enum Msg {
    Poll,
    Kill,
}

pub struct LlamaServer {
    handle: Option<JoinHandle<()>>,
    sender: Sender<Msg>,
}
impl LlamaServer {
    pub fn new(path: PathBuf, model_name: String) -> Self {
        let (tx, rx) = std::sync::mpsc::channel();
        let handle = std::thread::spawn(move || {
            let mut child = start_server(&path, &model_name);
            for msg in rx.iter() {
                match msg {
                    Msg::Poll => {}
                    Msg::Kill => {
                        child.kill().unwrap();
                        return;
                    }
                }
            }
        });

        Self {
            handle: Some(handle),
            sender: tx,
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

impl Drop for LlamaServer {
    fn drop(&mut self) {
        self.sender.send(Msg::Kill).unwrap();
        self.handle.take().unwrap().join().unwrap();
    }
}

fn start_server(path: &PathBuf, model_name: &str) -> Child {
    // println!("Starting server...");
    // println!("http://127.0.0.1:8080");
    let repo_path = path.join("llama.cpp");

    let models_path = path.join("models");
    let model = models_path.join(model_name);
    let bin_path = repo_path.join("build/bin/llama-server");
    let command = Command::new(bin_path)
        .args(&["--port", "8080"])
        .args(&["--model", model.to_str().unwrap()])
        .args(&["--log-disable"])
        .spawn()
        .unwrap();

    command
}
