llama-tk: FORCE
	cd llama-tk && \
	cargo run

native-llm-run:
	cd llm-native/llama.cpp/build/bin && \
	./llama-cli

docker-image-cpu:
	cd llm-cpu && \
	sudo docker build -t llm-server .

docker-run:
	cd llm-cpu && \
	sudo docker run --rm -it llm-server

native-llm:
	(mkdir -p llm-native || true) && \
	cd llm-native && \
	(git clone https://github.com/ggerganov/llama.cpp.git || true) && \
	cd llama.cpp && \
	cmake -B build && \
	cmake --build build --config Release

FORCE:
