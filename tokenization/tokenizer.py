class BytePairEncodingTokenizer:
    def __init__(self, corpus):
        self.corpus = corpus

    def pre_tokenize(self):
        """
        We are going to pre_tokenize the corpus just by splitting words by spaces
        """
        return self.corpus.split(" ")


    def generate_vocabulary(self, pre_tokenized_corpus):
        """
        We generate vocabulary from the pre-tokenized corpus by calculate the frequency of each word
        """

        word_frequency = dict()

        for word in pre_tokenized_corpus:
            word_frequency[word] = word_frequency.get(word, 0) + 1

        return word_frequency


    def compute_vocabulary(self, vocabulary):
        """
        We compute the vocabulary by getting all the letters and symbols from each word,
        and then we split each word into letters and computing the pairs
        """
        pairs_frequency = dict()
        for word, frequency in vocabulary.items():
            # if the length of the word is less than 2, we skip it
            # as we have no pair
            if len(word) <= 1:
                continue

            for i in range(len(word) - 1):
                pair = word[i] + word[i + 1]
                pairs_frequency[pair] += 1 * frequency

        