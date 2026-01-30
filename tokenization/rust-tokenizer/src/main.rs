use crate::byte_level_bpe_tokenizer::tokenizer::ByteLevelBPETokenizer;

mod byte_level_bpe_tokenizer;
mod models;

fn main() {
    let tokenizer: ByteLevelBPETokenizer = ByteLevelBPETokenizer::new(127);
}
