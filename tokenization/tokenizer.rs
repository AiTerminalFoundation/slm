struct ByteLevelBPETokenizer {
    num_merges: u16,
    vocabulary: HashSet<Vec<u8>, u32>, // maps byte sequences to ids
    merges: Vec<(Vec<u8>, Vec<u8>)>, // ordered list of merge operations
    byte_encoder: HashMap<u8, String>, // maps bytes to unicode characters
    byte_decoder: HashMap<String, u8>, // maps unicode characters to bytes
}

impl ByteLevelBPETokenizer {
    fn new(corpus: str, num_merges: i8) -> Self {

    }

    fn train(&self, corpus: str) {
        // pre-tokenize corpus
        // convert tokens to byte sequences
        // iteratively find the most frequent pairs
        // merge pairs until we reach num_merges
        // build the final vocabulary
    }

    fn get_word_pairs(&self, word: &[u8]) -> HashSet<([u8; 2])> {
        let mut word_pairs = HashSet::new();

        for i in 0..(word.len() - 1) {

            let mut pair: [u8; 2] = [word[i], word[i + 1]];

            word_pairs.insert(pair)
        }

        word_pairs
    }
}