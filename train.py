from dataset.dataset import NaturalLanguageToBashDataset

dataset = NaturalLanguageToBashDataset(

    "dataset/data/all_natural_language.nl",
    "dataset/data/all_commands.cm"
)

print(dataset[0])

