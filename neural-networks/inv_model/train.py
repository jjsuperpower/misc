import os
import torch
from torchvision import transforms
from torchvision.datasets import MNIST
from torch.utils.data import DataLoader
import pytorch_lightning as pl

from model import LeNet
from dataloader import get_mnist_dataloader


train_loader, valid_loader = get_mnist_dataloader()


lenet = LeNet()

trainer = pl.Trainer(accelerator="auto", max_epochs=5)
trainer.fit(model=lenet, train_dataloaders=train_loader, val_dataloaders=valid_loader)