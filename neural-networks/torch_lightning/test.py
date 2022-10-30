from model import LeNet
from dataloader import get_mnist_dataloader

lenet = LeNet.load_from_checkpoint("LeNet.ckpt")

train_loader, valid_loader = get_mnist_dataloader()

lenet.test_accuracy(valid_loader)