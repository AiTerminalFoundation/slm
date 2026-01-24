class BytePairEncodingTokenizer:

    def __init__(self, corpus, num_merges=100):
        self.corpus = corpus
        self.merges_vocabulary = set()

    def pre_tokenize(self):
        """
        We are going to pre_tokenize the corpus just by splitting words by spaces
        """
        return self.corpus.split(" ")


    def generate_vocabulary(self, pre_tokenized_corpus):
        """
        We generate vocabulary from the pre-tokenized corpus by calculate the frequency of each word
        """

        words_frequency = dict()

        for word in pre_tokenized_corpus:
            words_frequency[word] = words_frequency.get(word, 0) + 1
            for letter in word:
                self.merges_vocabulary.add(letter)

        return words_frequency


    def compute_vocabulary(self, words_frequency):
        """
        We compute the vocabulary by getting all the letters and symbols from each word,
        and then we split each word into letters and computing the pairs
        """
        pairs_frequency = dict()

        for word, frequency in words_frequency.items():
            # if the length of the word is less than 2, we skip it
            # as we have no pair
            if len(word) <= 1:
                continue

            for i in range(len(word) - 1):
                pair = [word[i], word[i + 1]]
                pairs_frequency[pair] += 1 * frequency

        print(pairs_frequency)


    def train_tokenizer(self):
        split_corpus = self.pre_tokenize()
        vocabulary = self.generate_vocabulary(split_corpus)
        self.compute_vocabulary(vocabulary)