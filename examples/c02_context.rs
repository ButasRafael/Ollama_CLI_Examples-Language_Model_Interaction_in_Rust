
use ollama::consts::MODEL;
use ollama::gen::gen_stream_print;
use ollama::Result;

use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::generation::completion::GenerationContext;
use ollama_rs::Ollama;
use simple_fs::{ensure_file_dir, save_json};


#[tokio::main]
async fn main() -> Result<()> {
    let ollama = Ollama::default();

    let prompts = &[
        "What do you think about the war between ucraine and russia?",
        "What was my first question?",
    ];

    let mut last_ctx: Option<GenerationContext> = None;

    for prompt in prompts {
        println!("\n->> prompt: {prompt}");
        let mut gen_req =
            GenerationRequest::new(MODEL.to_string(), prompt.to_string());

        if let Some(last_ctx) = last_ctx.take() {
            gen_req = gen_req.context(last_ctx);
        }

        let mut final_data_list = gen_stream_print(&ollama, gen_req).await?;

        if let Some(final_data) = final_data_list.pop() {
            last_ctx = Some(final_data.context);

            let ctx_file_path = ".c02-data/ctx.json";
            ensure_file_dir(ctx_file_path)?;
            save_json(ctx_file_path, &last_ctx)?;
        }
    }

    Ok(())
}