from model import LeNet
from dataloader import get_mnist_dataloader

import torch
import torch.nn as nn

lenet = LeNet.load_from_checkpoint("LeNet.ckpt")

# print(next(lenet.feature_extractor.children()).weight)

def inv_model(model):

    for layer in model.feature_extractor.children():
        if hasattr(layer, 'weight'):
            layer.weight = nn.Parameter(-layer.weight)

    for layer in model.classifier.children():
        if hasattr(layer, 'weight'):
            layer.weight = nn.Parameter(-layer.weight)

    return model


_, val_loader = get_mnist_dataloader()

img, label = next(iter(val_loader))
img, label = img[0:1], label[0:1]


print(f"Label: {label.item()}")


print(f"Accuracy before inversion: {lenet.test_accuracy(val_loader)}")

out = lenet(img)
lenet = inv_model(lenet)
out_inv = lenet(img)

print(f"Accuracy before inversion: {lenet.test_accuracy_inv(val_loader)}")



print('-'*20)

print(f"Logits: {out}")
print(f"Prediction: {torch.argmax(out)}")

print('-'*20)

print(f"Logits inv: {out_inv}")
print(f"Prediction inv: {torch.argmin(out_inv)}")