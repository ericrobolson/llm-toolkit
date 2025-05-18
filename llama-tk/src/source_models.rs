use std::{path::PathBuf, process::Command};

pub fn execute(path: &PathBuf, model: &str, url: &str) {
    let models_path = path.join("models");
    std::fs::create_dir_all(&models_path).unwrap();
    // let model = "openhermes-2.5-mistral-7b.Q5_K_S.gguf";
    // let url = "https://huggingface.co/TheBloke/OpenHermes-2.5-Mistral-7B-GGUF/resolve/main/openhermes-2.5-mistral-7b.Q5_K_S.gguf";
    let model_path = models_path.join(model);
    if !model_path.exists() {
        /*
                URL=https://huggingface.co/TheBloke/OpenHermes-2.5-Mistral-7B-GGUF/resolve/main/openhermes-2.5-mistral-7b.Q5_K_S.gguf

        mkdir -p models
        curl -L $URL > models/openhermes-2.5-mistral-7b.Q5_K_S.gguf
        curl -I $URL
                 */

        println!("Downloading model {}...", model);
        let mut command = Command::new("curl")
            .args(&["-L", url])
            .args(&["-O"])
            .current_dir(&models_path)
            .spawn()
            .unwrap();

        let result = command.wait().unwrap();

        if result.success() {
            println!("Model downloaded");
        } else {
            panic!("Model failed to download");
        }

        // println!("{}", String::from_utf8(command.stdout).unwrap());

        // if command.status.success() {
        //     println!("Model downloaded");
        // } else {
        //     panic!(
        //         "Model failed to download: {}",
        //         String::from_utf8(command.stderr).unwrap()
        //     );
        // }
    }
}
