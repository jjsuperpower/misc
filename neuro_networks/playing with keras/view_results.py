import matplotlib.pyplot as plt
import keras
from keras.models import Sequential, load_model
from keras.layers import Conv1D, Dense, MaxPool1D, MaxPool1D, Flatten, Reshape, Dropout, InputLayer
from keras.constraints import max_norm
import matplotlib.pyplot as plt
import numpy as np
import math


viewNumSamples = 3
inputShape = (100, 1)
learningCurv = 'mean_squared_error'

print('\n')
print('Loading Original plots')
origLoad = np.load('orig_plots.npz',)
print('Loading Noisy Plots')
noisyLoad = np.load('noisy_plots.npz')

print('Processing data')
start = origLoad['start']
end = origLoad['end']
dataPointsPerSample = origLoad['dataPointsPerSample']
origSamples2d = origLoad['samples']
noisySamples2d = noisyLoad['samples']

# Convert 2d (100000,100) array to 3d (100000,100,1) array
origSamples3d = origSamples2d.reshape(origSamples2d.shape[0], origSamples2d.shape[1], 1)
noisySamples3d = noisySamples2d.reshape(noisySamples2d.shape[0], noisySamples2d.shape[1], 1)

model = Sequential()
model.add(InputLayer(input_shape=inputShape))
model.add(Conv1D(filters=3, kernel_size=20, activation='linear'))
model.add(Conv1D(filters=45, kernel_size=4, activation='relu'))
model.add(Flatten())
model.add(Dense(30, activation='relu'))
model.add(Dense(100, activation='linear'))
model.add(Reshape(inputShape))

model.compile(optimizer='adamax', loss=learningCurv)

model = load_model('Trained_Neuro_Net.h5')
history = model

predicted3d = model.predict(noisySamples3d)
predicted2d = predicted3d.reshape(predicted3d.shape[0], predicted3d.shape[1])

randPick = []
for i in range(0, viewNumSamples):
    randPick.append(np.random.randint(0, len(origSamples2d) - 1))
xAxis = np.linspace(start, end, dataPointsPerSample)

print('Plotting Results')
plt.figure(figsize=(8,5))

plt.subplot(1, 2, 1)
plt.title('Orig. vs Noisy')
for i in range(0, viewNumSamples):
    
    yAxis = origSamples2d[randPick[i]]
    plt.plot(xAxis, yAxis, label='Orig. Line')
    yAxis = noisySamples2d[randPick[i]]
    plt.plot(xAxis, yAxis)

plt.subplot(1, 2, 2)
plt.title('Orig vs Predicted')
for i in range(0, viewNumSamples):
    yAxis = origSamples2d[randPick[i]]
    plt.plot(xAxis, yAxis)
    yAxis = predicted2d[randPick[i]]
    plt.plot(xAxis, yAxis)

plt.show()