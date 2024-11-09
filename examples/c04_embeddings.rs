use std::fs;
use std::path::Path;
use ollama::Result;
use ollama_rs::Ollama;
use simple_fs::{ensure_dir, read_to_string, save_be_f64, save_json};
use ollama::consts::MODEL;

const MOCK_DATA_DIR:&str="mock_data";
const C04_EMBEDDINGS:&str="c04_embeddings";

#[tokio::main]
async fn main()->Result<()>{
    let ollama=Ollama::default();
    ensure_dir(C04_EMBEDDINGS)?;
    let txt=read_to_string(Path::new(MOCK_DATA_DIR).join("for_embeddings.txt"))?;
    let splits=simple_text_splitter(&txt,500)?;
    println!("splits: {}",splits.len());
    for (i,seg) in splits.iter().enumerate(){
        let file_name=format!("c04_embeddings_{:0>2}.txt",i);
        let file_path=Path::new(C04_EMBEDDINGS).join(file_name);
        fs::write(file_path,&seg)?;
        println!("text length: {}",txt.len());
		let res=ollama.generate_embeddings(MODEL.to_string(),seg.to_string(),None).await?;
		println!("embeddings length: {}",res.embeddings.len());
		let file_name=format!("c04_embeddings_{:0>2}.json",i);
		save_json(Path::new(C04_EMBEDDINGS).join(file_name),&res.embeddings)?;
		let file_name=format!("c04_embeddings_{:0>2}.be-f64.bin",i);
		let file_path=Path::new(C04_EMBEDDINGS).join(file_name);
		save_be_f64(&file_path,&res.embeddings)?;

    }

    Ok(())

}

fn simple_text_splitter(txt: &str, num: u32) -> Result<Vec<String>> {
     	let mut result = Vec::new();
     	let mut last = 0;
     	let mut count = 0;

     	for (idx, _) in txt.char_indices() {
     		count += 1;
     		if count == num {
     			result.push(&txt[last..idx + 1]);
     			last = idx + 1;
     			count = 0;
     		}
     	}

     	if last < txt.len() {
     		result.push(&txt[last..]);
    	}

     	Ok(result.into_iter().map(String::from).collect())
     }