from matplotlib import pyplot as plt
import numpy as np
import torch

from patterns import get_loader

NUM_SHOW = 10

def compare_gray(before, after):
    before = before.reshape(before.shape[0], before.shape[1],before.shape[2])
    after = after.reshape(after.shape[0], after.shape[1],after.shape[2])

    fig, axs = plt.subplots(2, before.shape[0], figsize=(15,5))

    for i in range(before.shape[0]):
        plt.gray()
        axs[0, i].axis('off')
        axs[0, i].imshow(before[i])
        
    for i in range(after.shape[0]):
        plt.gray()
        axs[1, i].axis('off')
        axs[1, i].imshow(after[i])
        
    axs[0,0].set_title('Before', fontsize=20)
    axs[1,0].set_title('After', fontsize=20)
    plt.savefig("comp.png")

use_cuda = torch.cuda.is_available()
device = torch.device("cpu")
print(f"Using Device: {device}")

_, test_loader = get_loader(NUM_SHOW, device)

model = torch.load('auto.pth')
model.to(device)
model.eval()

img = next(iter(test_loader))[0]
img = img.to(device)

labels = next(iter(test_loader))[1].numpy()

with torch.no_grad():
    output = model(img)
    enc = model.encoder(img)

    img = img.detach().numpy()
    output = output.detach().numpy()
    enc = enc.detach().numpy()
    
compare_gray(img[:,0], output[:, 0])


fig, ax = plt.subplots()
ax.scatter(enc[:, 0], enc[:, 1])

for i, txt in enumerate(labels):
    ax.annotate(txt, (enc[:, 0][i], enc[:, 1][i]))

plt.savefig('enc.png')