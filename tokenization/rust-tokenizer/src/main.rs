use crate::byte_level_bpe_tokenizer::tokenizer::ByteLevelBPETokenizer;
use std::fs;

mod byte_level_bpe_tokenizer;
mod models;

fn main() {
    let corpus_path = "/Users/micheleverriello/slm/dataset/data/abs-guide.txt";
    let bytes = fs::read(corpus_path);

    if bytes.is_ok() {
        let corpus = String::from_utf8_lossy(&bytes.unwrap()).to_string();
        let mut tokenizer: ByteLevelBPETokenizer = ByteLevelBPETokenizer::new(10000);
        tokenizer.train(&corpus);
        tokenizer.export_to_json("tokenizer.json").expect("Error exporting tokenizer");

        println!("Tokenization end:");
        println!("Vocabulary size: {}", tokenizer.get_vocabulary().len());

        // printing all valid tokens
        for item in tokenizer.get_vocabulary() {
            let word_result = String::from_utf8(item.0.clone());
            if word_result.is_ok() {
                println!("{}", word_result.unwrap());
            }
        }
    } else {
        eprintln!("Unable to get the corpus, check file path.");
    }
}
