PROMPT="Give me a forth program"
BIN=./llm-native/llama.cpp/build/bin/llama-cli

$BIN -m models/openhermes-2.5-mistral-7b.Q5_K_S.gguf --prompt "$PROMPT" --no-display-prompt

# ./llm-native/llama.cpp/build/bin/llama-cli -m models/openhermes-2.5-mistral-7b.Q5_K_S.gguf -cnv --chat-template gemma