import torch
from torch.utils.data import Dataset

class NaturalLanguageToBashDataset(Dataset):

    def __init__(self, natural_language_file, commands_file):
        """
        Args:
            natural_language_file (str): path to text file with natural language prompts
            commands_file (str): path to text file with bash commands
        """
        with open(natural_language_file, "r", encoding="utf-8") as file:
            self.prompts = [line.strip() for line in file]

        with open(commands_file, "r", encoding="utf-8") as file:
            self.commands = [line.strip() for line in file]

    def __len__(self):
        return len(self.prompts)

    def __getitem__(self, idx):
        if torch.is_tensor(idx):
            idx = idx.tolist()

        sample = {
            "prompt": self.prompts[idx],
            "command": self.commands[idx]
        }
        return sample