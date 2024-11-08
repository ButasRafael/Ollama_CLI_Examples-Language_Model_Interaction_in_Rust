# Ollama_CLI_Examples-Language_Model_Interaction_in_Rust

## Descripiton
* This Rust project provides an API wrapper around the Ollama language model for generating text and managing chat-based interactions using async functions and the Tokio runtime. It supports streaming responses, managing conversation history, and saving generated data in various formats, such as JSON and binary. This example includes a series of code snippets that interact with the Ollama API for text generation, handling multiple prompts, embedding generation, and context-based chat sessions.

## Features
* Async functions using Tokio for non-blocking operations
* Streamed responses for prompt completion and chat messages
* Context handling between prompts to maintain session continuity
* Embedding generation with custom file management
* Simple text splitting for embedding files
## Usage
* To run this code, ensure you have the necessary dependencies installed and that your API key for Ollama is correctly configured.

## Sample Code
### The code snippets provided demonstrate various functionalities:
* Basic prompt completion with a single query.
* Context-based question-answer session handling multiple prompts.
* Chat-based actor structure with actors like system, user, assistant.
* Embedding generation for a sample text file.

## Getting Started
### Prerequisites
* Rust and Cargo
* ollama-rs, simple-fs, tokio
### Installation
* Clone the repository and build the project:
```
git clone https://github.com/ButasRafael/Ollama_CLI_Examples-Language_Model_Interaction_in_Rust.git
cd Ollama_CLI_Examples-Language_Model_Interaction_in_Rust
cargo build
```
### Running the project
* Cargo runs and watches:
```
cargo run --example c01_simple
cargo watch -q -c -x "run -q --example c01-simple"

cargo run --example c02_context
cargo watch -q -c -x "run -q --example c02-context"

cargo run --example c03_chat
cargo watch -q -c -x "run -q --example c03-chat"

cargo run --example c04_embeddings
cargo watch -q -c -x "run -q --example c04-embeddings"

```
## Files and Directories
* mock_data: Directory containing text files for embedding generation.
* c04_embeddings: Directory for saving embedding files in various formats.
