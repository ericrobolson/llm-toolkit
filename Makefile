docker-image-cpu:
	cd llm-cpu && \
	sudo docker build -t llm-server .

docker-run:
	cd llm-cpu && \
	sudo docker run --rm -it llm-server

native-llm:
	mkdir -p llm-native && \
	cd llm-native && \
	(git clone https://github.com/ggerganov/llama.cpp.git || true) && \
	cd llama.cpp && \
	make

native-llm-run:
	cd llm-native/llama.cpp && \
	./llama-cli