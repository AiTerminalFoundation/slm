"""
We will add in a list of strings each row by each file we get
"""
class Corpus:
    def __init__(self, natural_language_file, commands_file, book_file):
        self.corpus = []

        with open(natural_language_file, "r", encoding="utf-8") as file:
            self.corpus.extend([line.strip() for line in file if line.strip()])

        with open(commands_file, "r", encoding="utf-8") as file:
            self.corpus.extend([line.strip() for line in file if line.strip()])

        with open(book_file, "r", encoding="utf-8") as file:
            self.corpus.extend([line.strip() for line in file if line.strip()])

    def __len__(self):
        return len(self.corpus)

    def __iter__(self):
        """Make the class itself iterable"""
        return iter(self.corpus)
