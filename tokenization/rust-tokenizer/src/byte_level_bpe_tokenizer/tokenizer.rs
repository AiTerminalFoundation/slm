use crate::models::pair_frequency::PairFrequency;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct ByteLevelBPETokenizer {
    num_merges: u16,

    #[serde(skip)]
    vocabulary: HashMap<Vec<u8>, u32>, // maps byte sequences to ids
    // vocabulary_as_vec is not really useful in the algorithm, will just be used to have easier JSON serialization
    // as the HashMap will return an error as the key (vec<u8>) is not a string
    vocabulary_as_vec: Vec<(Vec<u8>, u32)>,

    merge_rules: Vec<(Vec<u8>, Vec<u8>)>,
    #[serde(skip)]
    decoder: HashMap<u32, Vec<u8>>, // decodes token ids to bytes sequences
    decoder_as_vec: Vec<(u32, Vec<u8>)>, // same thing of the vocabulary_as_vec, useful just for json serialization
    #[serde(skip)]
    last_token_id: u32,
}

impl ByteLevelBPETokenizer {
    pub fn new(num_merges: u16) -> Self {
        Self {
            num_merges,
            vocabulary: HashMap::new(),
            vocabulary_as_vec: Vec::new(),
            merge_rules: Vec::new(),
            decoder: HashMap::new(),
            decoder_as_vec: Vec::new(),
            last_token_id: 0,
        }
    }

    pub fn train(&mut self, corpus: &str) {
        self.initialize_vocabulary();
        let mut words_as_ids: Vec<Vec<u32>> = self.pre_tokenize_corpus(corpus);

        // iteratively find the most frequent pairs
        // merge pairs until we reach num_merges

        for _i in 0..self.num_merges {
            let mut pairs_frequencies = HashMap::new();

            let mut top_frequency: Option<PairFrequency> = None;

            for word in &words_as_ids {
                let word_pairs_frequencies = self.get_word_pairs(word);
                // adding the pairs frequencies to the main hashmap
                for item in word_pairs_frequencies {
                    let key = item.0;
                    let main_frequency = pairs_frequencies.get(&key).unwrap_or(&0);
                    let current_frequency = item.1; // the item of the map is a tuple with the key in 0 and freq in 1
                    let new_frequency = main_frequency + current_frequency;
                    pairs_frequencies.insert(key, new_frequency);

                    if top_frequency.is_none()
                        || top_frequency.as_ref().unwrap().frequency < new_frequency
                    {
                        top_frequency = Some(PairFrequency {
                            pair: key,
                            frequency: new_frequency,
                        })
                    }
                }
            }

            // at the end of the for loop we have the updated frequency map for the current merge iteration
            if top_frequency.is_none() {
                break;
            }

            let top_frequency_unwrapped = top_frequency.unwrap();
            let token_id =
                self.add_new_pair_to_vocabulary_and_merge_rules(&top_frequency_unwrapped.pair);

            // update all the words
            // we find the pair with higher frequency that we found in the earlier loop,
            // and we replace it with the new token id
            for word in words_as_ids.iter_mut() {
                let indexes = self.find_pairs(word, &top_frequency_unwrapped.pair);

                // going through the indexes in reverse to avoid index shifting
                for &index in indexes.iter().rev() {
                    word[index] = token_id;
                    word.remove(index + 1);
                }
            }
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
                word_as_bytes
                    .iter()
                    .map(|byte| *self.vocabulary.get(&vec![*byte]).unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect();

        words_as_ids
    }

    /// This function handles the vocabulary initialization.
    /// We are going to put all possible combinations of 8 bit (so 0-255) and give them an ID
    /// For now I will use a backtracking algo to generate all combinations of bits
    fn initialize_vocabulary(&mut self) {
        for i in 0..=255 {
            let byte_sequence = [i].to_vec();
            self.add_byte_sequence_to_vocabulary_and_decoder(byte_sequence);
        }
    }

    /// We add the new merge we found to the vocabulary
    /// The input is composed by 2 token IDS, we will decode it inside byte a sequence, and
    /// then we will create a new ID for that byte sequence and will add it to the vocabulary
    fn add_new_pair_to_vocabulary_and_merge_rules(&mut self, ids_pair: &[u32; 2]) -> u32 {
        let mut first_byte_sequence = self
            .decoder
            .get(&ids_pair[0])
            .unwrap_or(&Vec::new())
            .clone();

        let second_byte_sequence = self
            .decoder
            .get(&ids_pair[1])
            .unwrap_or(&Vec::new())
            .clone();

        let merge_rule = (first_byte_sequence.clone(), second_byte_sequence.clone());
        self.merge_rules.push(merge_rule);

        first_byte_sequence.extend(&second_byte_sequence);

        self.add_byte_sequence_to_vocabulary_and_decoder(first_byte_sequence)
    }

    fn add_byte_sequence_to_vocabulary_and_decoder(&mut self, byte_sequence: Vec<u8>) -> u32 {
        let token_id = self.last_token_id;
        self.vocabulary.insert(byte_sequence.clone(), token_id);

        // not updating the vocabulary as vector while adding to avoid overloading memory during training
        // it will be entirely populated in the json export
        // self.vocabulary_as_vec.push((byte_sequence.clone(), token_id));

        // updating the decoder, so we can retrieve a byte sequence by its id
        self.decoder.insert(token_id, byte_sequence.clone());

        self.last_token_id += 1;

        // return the current
        token_id
    }

    /// This function returns the 1st index of each non-overlapping pair if available
    fn find_pairs(&self, word: &[u32], pair: &[u32; 2]) -> Vec<usize> {
        let mut pairs_indexes = Vec::new();
        let mut i = 0;

        while i < word.len() - 1 {
            if word[i] == pair[0] && word[i + 1] == pair[1] {
                pairs_indexes.push(i);
                i += 2;
            } else {
                i += 1;
            }
        }

        pairs_indexes
    }

    /// This function writes the tokenizer object as a JSON and then export it to a given filepath
    pub fn export_to_json(&mut self, path: &str) -> std::io::Result<()> {
        // populating the vocabulary as a vector with the hashmap vocabulary values
        self.vocabulary_as_vec = self
            .vocabulary
            .iter()
            .map(|(key, value)| (key.clone(), *value))
            .collect();

        // populating decoder as a vector
        self.decoder_as_vec = self
            .decoder
            .iter()
            .map(|(key, value)| (*key, value.clone()))
            .collect();

        // serializing to JSON the tokenizer object
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    pub fn get_vocabulary(&self) -> HashMap<Vec<u8>, u32> {
        self.vocabulary.clone()
    }
}
