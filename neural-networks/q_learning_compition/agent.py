import torch
import torch.nn as nn
import torch.nn.functional as F

import torch.optim as optim

class nn_agent:
    def __init__(self, INPUT_SIZe, OUTPUT_SIZE):
        self.model = self._build_model()
        self.device = device = torch.device('cuda' if torch.cuda.is_available() else 'cpu')
        self.training = True

        self.model.to_device(self.device)

        self.criterion = nn.CrossEntropyLoss()
        self.optimizer = optim.Adam()

        self.EPOCHS = 3


    def train(self, data):
        inputs, labels = data

        running_loss = 0.0
        for _ in range(self.EPOCHS):

            self.optimizer.zero_grad()

            outputs = self.model(inputs)
            loss = self.criterion(outputs, labels)
            loss.backward()
            self.optim.step()

            running_loss += loss.item()

        return running_loss

    def test(self, data):
        inputs, _ = data
        return self.model(inputs)


    def _build_model(self, INPUT_SIZE, OUTPUT_SIZE):
        class Net(nn.Module):
            def __init__(self):
                super().__init__()
                self.conv1 = nn.Conv1d(INPUT_SIZE, 8, kernel_size=3, stride=1, padding='same')
                self.conv2 = nn.Conv1d(8, 8, kernel_size=3, stride=1, padding='same')
                self.fc1 = nn.Linear(8, 16)
                self.fc2 = nn.Linear(16, 16)
                self.fc3 = nn.Linear(16, 2)

            def forward(self, x):
                x = F.relu(self.conv1(x))
                x = F.relu(self.conv2(x))
                x = torch.flaten(x, 1)
                x = F.relu(self.fc1)
                x = F.relu(self.fc2)
                x = F.relu(self.fc3)

                return x
        return Net()

        
