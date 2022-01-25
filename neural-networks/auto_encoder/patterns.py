import torch
import torch.nn as nn
import torch.nn.functional as F
import torch.optim as optim
from torchvision import datasets, transforms
from models import Net  

def train(model, device, train_loader, optimizer, epoch):
    model.train()
    for i, (img, _) in enumerate(train_loader):
        
        optimizer.zero_grad()
        img = img.to(device)
        output = model(img)
        loss = F.mse_loss(output, img)
        loss.backward()
        optimizer.step()

        if not (i+1) % 20:
            print(f"Epoch: {epoch}, loss: {loss.item()}")
            torch.save(model, "auto.pth")
        
# def view(model, device, test_loader):
#     model.eval()
#     test_loss = 0
#     correct = 0
#     with torch.no_grad():
#         for data, target in test_loader:
#             data, target = data.to(device), target.to(device)
#             output = model(data)
#             test_loss += F.nll_loss(output, target, reduction='sum').item()  # sum up batch loss
#             pred = output.argmax(dim=1, keepdim=True)  # get the index of the max log-probability
#             correct += pred.eq(target.view_as(pred)).sum().item()

#     test_loss /= len(test_loader.dataset)

#     print('\nTest set: Average loss: {:.4f}, Accuracy: {}/{} ({:.0f}%)\n'.format(
#         test_loss, correct, len(test_loader.dataset),
#         100. * correct / len(test_loader.dataset)))


def get_loader(batchsize ,device):

    train_kwargs = {'batch_size': batchsize}
    test_kwargs = {'batch_size': batchsize}

    if device == 'cuda':
        cuda_kwargs = {'num_workers': 1, 'pin_memory': True, 'shuffle': True}
        train_kwargs.update(cuda_kwargs)
        test_kwargs.update(cuda_kwargs)

    transform=transforms.Compose([
        transforms.ToTensor(),
        ])

    dataset1 = datasets.MNIST('./data', train=True, transform=transform)
    dataset2 = datasets.MNIST('./data', train=False, transform=transform)
    train_loader = torch.utils.data.DataLoader(dataset1,**train_kwargs)
    test_loader = torch.utils.data.DataLoader(dataset2, **test_kwargs)

    return train_loader, test_loader

if __name__ == '__main__':

    # Training settings
    torch.manual_seed(64)
    use_cuda = torch.cuda.is_available()
    device = torch.device("cuda" if use_cuda else "cpu")
    print(f"Using Device: {device}")

    model = Net().to(device)
    optimizer = optim.Adam(model.parameters())

    train_loader, test_loader = get_loader(32, device)

    for epoch in range(20):
        train(model, device, train_loader, optimizer, epoch + 1)
        # view(model, device, test_loader)

