use crate::byte_level_bpe_tokenizer::tokenizer::ByteLevelBPETokenizer;
use std::collections::HashMap;

mod byte_level_bpe_tokenizer;
mod models;

fn main() {
    let mut tokenizer: ByteLevelBPETokenizer = ByteLevelBPETokenizer::new(15);
    tokenizer.train("Hello, World, test tokenizer, tea, coffee, heating");
}
