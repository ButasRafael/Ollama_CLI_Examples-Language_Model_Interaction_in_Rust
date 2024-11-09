
use ollama::consts::{DEFAULT_SYSTEM_MOCK,MODEL};
use ollama::Result;

use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::Ollama;
use ollama::gen::gen_stream_print;


#[tokio::main]
async fn main() -> Result<()> {
    let ollama = Ollama::default();

    let model = MODEL.to_string();
    let prompt = "What is the best programming language? (Be concise)".to_string();

    let gen_req = GenerationRequest::new(model, prompt)
        .system(DEFAULT_SYSTEM_MOCK.to_string());

    gen_stream_print(&ollama, gen_req).await?;

    Ok(())
}