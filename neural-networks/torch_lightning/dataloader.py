import os
import torch
from torchvision import transforms
from torchvision.datasets import MNIST
from torch.utils.data import DataLoader
import pytorch_lightning as pl

from model import LeNet

BATCH_SIZE = 32

transforms = transforms.Compose([transforms.Resize((28, 28)),transforms.ToTensor()])

def get_mnist_dataset():
    # download and create datasets
    train_dataset = MNIST(root=os.getcwd(), train=True, transform=transforms,download=True)
    valid_dataset = MNIST(root=os.getcwd(), train=False, transform=transforms)

    return train_dataset, valid_dataset

def get_mnist_dataloader():
    train_dataset, valid_dataset = get_mnist_dataset()

    # define the data loaders
    train_loader = DataLoader(dataset=train_dataset, batch_size=BATCH_SIZE, shuffle=True, num_workers=8)
    valid_loader = DataLoader(dataset=valid_dataset, batch_size=BATCH_SIZE, shuffle=False, num_workers=8)

    return train_loader, valid_loader