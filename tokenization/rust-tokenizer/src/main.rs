use crate::byte_level_bpe_tokenizer::tokenizer::ByteLevelBPETokenizer;
use std::collections::HashMap;

mod byte_level_bpe_tokenizer;
mod models;

fn main() {
    let mut tokenizer: ByteLevelBPETokenizer = ByteLevelBPETokenizer::new(2);
    tokenizer.train("Hello, World, test tokenizer, tea, coffee, heating");

    let vocabulary: HashMap<Vec<u8>, u32> = tokenizer.get_vocabulary();
    println!("size {:?}", vocabulary.len());
    println!("All tokens as characters");
    for token in vocabulary.keys() {
        println!(tokenizer.decode_word(token))
    }
}
