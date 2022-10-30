import torch
from torch import nn
from torch.utils.data import DataLoader
import torch.nn.functional as F
import pytorch_lightning as pl
import torchmetrics

class LeNet(pl.LightningModule):

    def __init__(self, n_classes=10):
        super().__init__()

        self.accuracy = torchmetrics.Accuracy()
        
        self.feature_extractor = nn.Sequential(            
            nn.Conv2d(1, 8, 3, padding='same', stride=1),
            nn.Tanh(),
            nn.AvgPool2d(kernel_size=2),
            nn.Conv2d(8, 16, 3, padding='same', stride=1),
            nn.Tanh(),
            nn.AvgPool2d(kernel_size=2),
            nn.Conv2d(16, 32, 3, padding='same', stride=1),
            nn.Tanh(),
        )

        self.classifier = nn.Sequential(
            nn.Linear(in_features=7*7*32, out_features=128),
            nn.Tanh(),
            nn.Linear(in_features=128, out_features=n_classes),
        )


    def forward(self, x):
        x = self.feature_extractor(x)
        x = torch.flatten(x, 1)
        logits = self.classifier(x)
        return logits

    def training_step(self, batch, batch_index):
        x, y = batch
        y_one_hot = F.one_hot(y, num_classes=10)
        x_hat = F.softmax(self.forward(x), dim=1)
        pred = torch.argmax(x_hat, dim=1)

        loss = F.binary_cross_entropy(x_hat, y_one_hot.float())
        acc = self.accuracy(pred, y)
        
        self.log("Train Loss", loss)
        self.log("Train Accuracy", acc)
        return loss

    def validation_step(self, batch, batch_idx):
        x, y = batch
        y_one_hot = F.one_hot(y, num_classes=10)
        x_hat = F.softmax(self.forward(x), dim=1)
        pred = torch.argmax(x_hat, dim=1)

        loss = F.binary_cross_entropy(x_hat, y_one_hot.float())
        acc = self.accuracy(pred, y)

        self.log("Validation Loss", loss)
        self.log("Validation Accuracy", acc)
        return pred

    def test_accuracy(self, loader : DataLoader):
        self.eval()
        acc_sum = 0
        for idx, (x, y) in enumerate(loader):
            x_hat = F.softmax(self.forward(x), dim=1)
            pred = torch.argmax(x_hat, dim=1)
            acc = self.accuracy(pred, y)
            acc_sum += acc

        return acc_sum / (idx + 1)

    def test_accuracy_inv(self, loader : DataLoader):
        self.eval()
        acc_sum = 0
        for idx, (x, y) in enumerate(loader):
            x_hat = F.softmax(self.forward(x), dim=1)
            pred = torch.argmin(x_hat, dim=1)           #argmin not argmax
            acc = self.accuracy(pred, y)
            acc_sum += acc

        return acc_sum / (idx + 1)

     
    def configure_optimizers(self):
        return torch.optim.Adam(self.parameters(), lr=1e-3)