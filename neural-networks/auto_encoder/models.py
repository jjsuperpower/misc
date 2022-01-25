import torch
import torch.nn as nn
import torch.nn.functional as F

class Net(nn.Module):
    def __init__(self):
        super(Net, self).__init__()
        
        # encoder
        self.conv1 = nn.Conv2d(1, 16, 3, stride=1, padding='same')
        self.conv2 = nn.Conv2d(16, 32, 3, stride=1, padding='same')
        self.conv3 = nn.Conv2d(32, 64, 3, stride=1, padding='same')
        self.fc1 = nn.Linear(28*28*64, 128)
        self.fc2 = nn.Linear(128, 64)
        self.fc3 = nn.Linear(64, 2)

        # decoder
        self.fc4 = nn.Linear(2, 64)
        self.fc5 = nn.Linear(64, 128)
        self.fc6 = nn.Linear(128, 28*28*64)
        self.deconv1 = nn.ConvTranspose2d(64, 32, 3, stride=1, padding=1)
        self.deconv2 = nn.ConvTranspose2d(32, 16, 3, stride=1, padding=1)
        self.deconv3 = nn.ConvTranspose2d(16, 2, 3, stride=1, padding=1)    #should be 1 not 2, but that breaks it for some reason

    def forward(self, x):
        enc = self.encoder(x)
        dec = self.decoder(enc)
        return dec

    def encoder(self, x):
        x = F.relu(self.conv1(x))
        x = F.relu(self.conv2(x))
        x = F.relu(self.conv3(x))

        x = torch.flatten(x, 1)

        x = F.relu(self.fc1(x))
        x = F.relu(self.fc2(x))
        x = F.relu(self.fc3(x))

        return x

    def decoder(self, x):

        x = F.relu(self.fc4(x))
        x = F.relu(self.fc5(x))
        x = F.relu(self.fc6(x))

        x = torch.reshape(x, (x.size(0), 64, 28, 28))
        x = F.relu(self.deconv1(x))
        x = F.relu(self.deconv2(x))
        x = F.relu(self.deconv3(x))
        x = x[:,0:1,:,:]
        x = torch.clip(x, max=1.0)

        return x