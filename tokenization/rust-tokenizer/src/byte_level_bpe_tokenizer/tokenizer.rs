use crate::models::pair_frequency::PairFrequency;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

pub struct ByteLevelBPETokenizer {
    num_merges: u16,
    vocabulary: HashMap<Vec<u8>, u32>, // maps byte sequences to ids
    decoder: HashMap<u32, Vec<u8>>,    // decodes token ids to bytes sequences
    last_token_id: u32,
}

impl ByteLevelBPETokenizer {
    fn new(num_merges: i8) -> Self {
        return Self {
            num_merges: num_merges as u16,
            vocabulary: HashMap::new(),
            decoder: HashMap::new(),
            last_token_id: 0,
        };
    }

    fn train(&mut self, corpus: &str) {
        self.initialize_vocabulary();
        let words_as_ids: Vec<Vec<u32>> = self.pre_tokenize_corpus(corpus);

        // iteratively find the most frequent pairs
        // merge pairs until we reach num_merges

        for _i in 0..self.num_merges {
            let mut pairs_frequencies = HashMap::new();

            // min-heap useful to get always the most frequent pair
            let mut top_frequency: BinaryHeap<Reverse<PairFrequency>> = BinaryHeap::new();

            for word in words_as_ids.iter() {
                let word_pairs_frequencies = self.get_word_pairs(word);
                // adding the pairs frequencies to the main hashmap
                for item in word_pairs_frequencies {
                    let key = item.0;
                    let main_frequency = pairs_frequencies.get(&key).unwrap_or(&0);
                    let current_frequency = item.1; // the item of the map is a tuple with the key in 0 and freq in 1
                    let new_frequency = main_frequency + current_frequency;
                    pairs_frequencies.insert(key, new_frequency);

                    top_frequency.push(Reverse(
                        PairFrequency {
                            pair: key,
                            frequency: new_frequency,
                        }
                    ));

                    if top_frequency.len() > 1 {
                        top_frequency.pop();
                    }
                }
            }
            // at the end of the for loop we have the updated frequency map for the current merge iteration
            if top_frequency.is_empty() {
                break;
            }

            let top_pair_frequency = top_frequency.pop().unwrap().0;
            self.add_new_pair_to_vocabulary(&top_pair_frequency.pair);

            // update all the words
        }
    }

    // The input word is a vector of token IDs
    fn get_word_pairs(&self, word: &[u32]) -> HashMap<[u32; 2], u32> {
        let mut word_pairs_frequencies = HashMap::new();

        for i in 0..(word.len() - 1) {
            let pair: [u32; 2] = [word[i], word[i + 1]];

            let frequency = word_pairs_frequencies.get(&pair).unwrap_or(&0);
            word_pairs_frequencies.insert(pair, frequency + 1);
        }

        word_pairs_frequencies
    }

    /// Get the corpus as a string, split it by whitespace, and convert each word to a byte sequence.
    /// After that each byte sequence is replaced by its ID, to have a more elegant
    /// management of the merges later
    /// This is the first easy implementation of a pre-tokenization just splitting words by spaces
    /// instead of using a regex (that it will be the next step after all the LLM works decently)
    fn pre_tokenize_corpus(&self, corpus: &str) -> Vec<Vec<u32>> {
        let words_by_space: Vec<&str> = corpus.split_whitespace().collect();

        // Str::as_bytes() converts a utf-8 string to a byte vector
        let words_as_bytes: Vec<Vec<u8>> = words_by_space
            .iter()
            .map(|word_as_characters| word_as_characters.as_bytes().to_vec())
            .collect();

        let words_as_ids: Vec<Vec<u32>> = words_as_bytes
            .iter()
            .map(|word_as_bytes| {
                word_as_bytes.iter().map(|byte| {
                    *self.vocabulary.get(&vec![*byte]).unwrap()
                }).collect::<Vec<u32>>()
            })
            .collect();

        words_as_ids
    }

    /// This function handles the vocabulary initialization.
    /// We are going to put all possible combinations of 8 bit (so 0-255) and give them an ID
    /// For now I will use a backtracking algo to generate all combinations of bits
    fn initialize_vocabulary(&mut self) {
        self.backtrack_initialize_vocabulary(Vec::new());
    }

    fn backtrack_initialize_vocabulary(&mut self, mut actual_combination: Vec<u8>) {
        if actual_combination.len() == 8 {

            self.add_byte_sequence_to_vocabulary_and_decoder(actual_combination.clone());

            return;
        }

        for i in 0..2 {
            actual_combination.push(i);
            self.backtrack_initialize_vocabulary(actual_combination.clone());
            actual_combination.remove(actual_combination.len() - 1);
        }
    }

    /// We add the new merge we found to the vocabulary
    /// The input is composed by 2 token IDS, we will decode it inside byte a sequence and
    /// then we will create a new ID for that byte sequence and will add it to the vocabulary
    fn add_new_pair_to_vocabulary(&mut self, ids_pair: &[u32; 2]) {
        let mut first_byte_sequence = self.decoder.get(&ids_pair[0]).unwrap_or(&Vec::new()).clone();
        let second_byte_sequence = self.decoder.get(&ids_pair[1]).unwrap_or(&Vec::new()).clone();

        first_byte_sequence.extend(&second_byte_sequence);

        self.add_byte_sequence_to_vocabulary_and_decoder(first_byte_sequence)
    }

    fn add_byte_sequence_to_vocabulary_and_decoder(&mut self, byte_sequence: Vec<u8>) {
        self.vocabulary
            .insert(byte_sequence.clone(), self.last_token_id);

        // updating the decoder, so we can retrieve a byte sequence by its id
        self.decoder
            .insert(self.last_token_id, byte_sequence.clone());

        self.last_token_id += 1;
    }
}
