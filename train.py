from dataset.dataset import NaturalLanguageToBashDataset
from tokenization.tokenizer import BytePairEncodingTokenizer

# Load dataset
dataset = NaturalLanguageToBashDataset(

    "dataset/data/all_natural_language.nl",
    "dataset/data/all_commands.cm"
)


tokenizer = BytePairEncodingTokenizer(dataset)
