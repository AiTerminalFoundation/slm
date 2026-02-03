use crate::byte_level_bpe_tokenizer::tokenizer::ByteLevelBPETokenizer;
use std::collections::HashMap;

mod byte_level_bpe_tokenizer;
mod models;

fn main() {
    let mut tokenizer: ByteLevelBPETokenizer = ByteLevelBPETokenizer::new(15);
    tokenizer.train("Hello, World, test tokenizer, tea, coffee, heating");

    let vocabulary: HashMap<Vec<u8>, u32> = tokenizer.get_vocabulary();
    for item in vocabulary {
        let string_word = tokenizer.decode_word(item.0.clone());
        if !string_word.is_empty() {
            println!("{}: {}", string_word, item.1);
        }
    }
}
