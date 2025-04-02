PROMPT="Give me a forth program"
BIN=./llm-native/llama.cpp/build/bin/llama-server
# llama-server -m model.gguf --port 8080

# $BIN -m models/openhermes-2.5-mistral-7b.Q5_K_S.gguf --prompt "$PROMPT" --no-display-prompt
$BIN -m models/openhermes-2.5-mistral-7b.Q5_K_S.gguf --port 8080

# ./llm-native/llama.cpp/build/bin/llama-cli -m models/openhermes-2.5-mistral-7b.Q5_K_S.gguf -cnv --chat-template gemma