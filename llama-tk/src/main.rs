mod build_llama_cpp;
mod database;
mod llama_server;
mod source_models;

use llama_server::{LlamaServer, Status};
use std::{io::Write, path::PathBuf};

fn main() {
    let dir: PathBuf = ".llama-tk/".into();
    std::fs::create_dir_all(&dir).unwrap();

    database::setup(&dir);
    build_llama_cpp::execute(&dir);

    let models: Vec<(String, String)> = vec![
        (
            "deepseek-coder-6.7b-instruct.Q4_K_M.gguf",
            "https://huggingface.co/TheBloke/deepseek-coder-6.7B-instruct-GGUF/resolve/main/deepseek-coder-6.7b-instruct.Q4_K_M.gguf",
        ),
        (
            "openhermes-2.5-mistral-7b.Q5_K_S.gguf",
            "https://huggingface.co/TheBloke/OpenHermes-2.5-Mistral-7B-GGUF/resolve/main/openhermes-2.5-mistral-7b.Q5_K_S.gguf",
        ),
        // (
        //     "wizardlm-33b-v1.0-uncensored.Q4_K_M.gguf",
        //     "https://huggingface.co/TheBloke/WizardLM-33B-V1.0-Uncensored-GGUF/resolve/main/wizardlm-33b-v1.0-uncensored.Q4_K_M.gguf",
        // ),
        (
            "orcamaid-v3-13b-32k.Q5_K_S.gguf",
            "https://huggingface.co/TheBloke/OrcaMaid-v3-13B-32k-GGUF/resolve/main/orcamaid-v3-13b-32k.Q5_K_S.gguf",
        ),
    ].iter().map(|(m, u)| (m.to_string(), u.to_string())).collect();

    for (model, url) in models.iter() {
        source_models::execute(&dir, model, url);
    }

    let mut server = initialize_server(&dir, &models);

    // REPL
    loop {
        print!("\n> ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let prompt = input.trim();

        if prompt == "exit" {
            break;
        }

        if prompt == "restart" {
            server = initialize_server(&dir, &models);
            continue;
        }

        let mut response = server.ask(prompt);
        let mut processing = true;

        while processing {
            match response.poll() {
                Status::Poll { msg } => {
                    if let Some(msg) = msg {
                        print!("{}", msg);
                        std::io::stdout().flush().unwrap();
                    }
                }
                Status::Error { msg } => {
                    println!("Error: {}", msg);
                }
                Status::Complete => {
                    processing = false;
                }
            }
        }
    }
}

fn initialize_server(dir: &PathBuf, models: &Vec<(String, String)>) -> LlamaServer {
    let mut model: Option<String> = None;
    while model.is_none() {
        println!("\nAvailable models:");
        for (i, (model, _)) in models.iter().enumerate() {
            println!("{}. {}", i + 1, model);
        }

        println!("\nSelect a model (1-{}):", models.len());

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let index = input.trim().parse::<usize>().unwrap_or(0);
        if index < 1 || index > models.len() {
            eprintln!("Invalid selection");
        } else {
            let (m, _) = &models[index - 1];
            model = Some(m.to_string());
        }
    }
    let model = model.unwrap();

    let server = LlamaServer::new(dir.clone(), model);
    server
}
