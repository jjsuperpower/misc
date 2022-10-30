from model import LeNet
from dataloader import get_mnist_dataloader

import torch
import torch.nn as nn

CHECKPOINT_FILE = "lightning_logs/version_1/checkpoints/epoch=4-step=9375.ckpt"

lenet = LeNet.load_from_checkpoint(CHECKPOINT_FILE)
lenet.eval()

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


print(f"Accuracy before inversion: {lenet.test_accuracy(val_loader)}")

out = lenet(img)
lenet = inv_model(lenet)
out_inv = lenet(img)

print(f"Accuracy after inversion: {lenet.test_accuracy_inv(val_loader)}")


print('-'*20)

print(f"Label: {label.item()}")

print('-'*20)

print(f"Logits: {out}")
print(f"Prediction: {torch.argmax(out)}")

print('-'*20)

print(f"Logits inv: {out_inv}")
print(f"Prediction inv: {torch.argmin(out_inv)}")

print('-'*20)

print("Extra stuff")
print(out + out_inv)
print(out - out_inv)