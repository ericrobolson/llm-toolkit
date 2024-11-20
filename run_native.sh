PROMPT="give me a recipe for exotic noodles"

# ./llm-native/llama.cpp/llama-cli -m models/openhermes-2.5-mistral-7b.Q5_K_S.gguf --prompt "$PROMPT"

./llm-native/llama.cpp/llama-cli -m models/openhermes-2.5-mistral-7b.Q5_K_S.gguf -cnv --chat-template gemma