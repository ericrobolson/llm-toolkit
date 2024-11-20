FROM alpine:3.14
# Set environment variables for non-interactive installations
ENV DEBIAN_FRONTEND=noninteractive

# Install essential build tools and dependencies
RUN apk update && \
    apk add --no-cache \
    git

RUN git clone https://github.com/ggerganov/llama.cpp.git /llama.cpp

WORKDIR /llama.cpp
    
RUN apk add --no-cache \
    g++ \
    make

WORKDIR /llama.cpp

RUN make

RUN apk add --no-cache \
    bash

# # Clone the llama.cpp repository
# RUN git clone https://github.com/ggerganov/llama.cpp.git /llama.cpp

# # Set working directory to llama.cpp
# WORKDIR /llama.cpp

# # Compile llama.cpp
# RUN cmake -S . -B build \
#     && cmake --build build -j $(nproc)

# # Expose any necessary ports (if running a server or app)
# # EXPOSE 8080

# Default command (you can change this to run a script or program)
CMD ["bash"]
