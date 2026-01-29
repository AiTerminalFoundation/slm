use std::collections::HashMap;
use std::collections::HashSet;

struct ByteLevelBPETokenizer {
    num_merges: u16,
    vocabulary: HashMap<Vec<u8>, u32>, // maps byte sequences to ids
    merges: Vec<(Vec<u8>, Vec<u8>)>,   // ordered list of merge operations
    byte_encoder: HashMap<u8, String>, // maps bytes to unicode characters
    byte_decoder: HashMap<String, u8>, // maps unicode characters to bytes
    last_token_id: u32,
}

impl ByteLevelBPETokenizer {
    fn new(num_merges: i8) -> Self {
        return Self {
            num_merges: num_merges as u16,
            vocabulary: HashMap::new(),
            merges: Vec::new(),
            byte_encoder: HashMap::new(),
            byte_decoder: HashMap::new(),
            last_token_id: 0,
        };
    }

    fn train(&mut self, corpus: &str) {
        // initialize vocabulary
        // pre-tokenize corpus
        // convert tokens to byte sequences
        // iteratively find the most frequent pairs
        // merge pairs until we reach num_merges
        // build the final vocabulary
    }

    fn get_word_pairs(&self, word: &[u8]) -> HashSet<[u8; 2]> {
        let mut word_pairs = HashSet::new();

        for i in 0..(word.len() - 1) {
            let pair: [u8; 2] = [word[i], word[i + 1]];

            word_pairs.insert(pair);
        }

        word_pairs
    }

    /// Get the corpus as a string, split it by whitespace, and convert each word to a byte sequence.
    /// This is the first easy implementation of a pre-tokenization just splitting words by spaces
    /// instead of using a regex (that it will be the next step after all the LLM works decently)
    fn pre_tokenize_corpus(&self, corpus: &str) -> Vec<Vec<u8>> {
        let words_by_space: Vec<&str> = corpus.split_whitespace().collect();

        // Str::as_bytes() converts a utf-8 string to a byte vector
        let words_as_bytes: Vec<Vec<u8>> = words_by_space
            .iter()
            .map(|word_as_characters| word_as_characters.as_bytes().to_vec())
            .collect();

        return words_as_bytes;
    }

    /// This function handles the vocabulary initialization.
    /// We are going to put all possible combinations of 8 bit (so 0-255) and give them an ID
    /// For now I will use a backtracking algo to generate all combinations of bits
    fn initialize_vocabulary(&mut self) {
        self.backtrack_initialize_vocabulary(Vec::new());
    }

    fn backtrack_initialize_vocabulary(&mut self, mut actual_combination: Vec<u8>) {
        if actual_combination.len() == 8 {
            // add the combination to the vocabulary
            self.vocabulary
                .insert(actual_combination.clone(), self.last_token_id);
            self.last_token_id = self.last_token_id + 1;

            return;
        }

        for i in 0..2 {
            actual_combination.push(i);
            self.backtrack_initialize_vocabulary(actual_combination.clone());
            actual_combination.remove(actual_combination.len() - 1);
        }
    }
}

fn main() {
    let mut tokenizer = ByteLevelBPETokenizer::new(127);
    tokenizer.train("Hello world!");
    tokenizer.initialize_vocabulary();
    // println!("{:?}", tokenizer.vocabulary);
    println!("{:?}", tokenizer.vocabulary.len());
    println!("{:?}", tokenizer.last_token_id);
}
