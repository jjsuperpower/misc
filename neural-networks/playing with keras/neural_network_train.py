import keras
from keras.models import Sequential
from keras.layers import Conv1D, Dense, MaxPool1D, MaxPool1D, Flatten, Reshape, Dropout, InputLayer
from keras.constraints import max_norm
import matplotlib.pyplot as plt
import numpy as np
import math
import h5py

# np.random.seed(38)

def convToFloat32(x):
    return np.float32(x)

# Basic Perams
viewNumSamples = 3
inputShape = (100,1)
batchSize = 32
epochs = 5
validationPercent = 0.2
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

print('Building Neuro Network')

model = Sequential()
model.add(Conv1D(filters=3, kernel_size=20, activation='linear', input_shape=inputShape))
model.add(Conv1D(filters=45, kernel_size=4, activation='relu'))
model.add(Flatten())
model.add(Dense(30, activation='relu'))
model.add(Dense(100, activation='linear'))
model.add(Reshape(inputShape))

print('\n')
model.summary()
model.compile(optimizer='adamax', loss=learningCurv)
history = model.fit(noisySamples3d, origSamples3d, epochs=epochs, validation_split=validationPercent, batch_size=batchSize)

print('\n')
print('Saving Neuro Network')
model.save('Trained_Neuro_Net.h5')

print('Plotting Results')
plt.figure(figsize=(13,5))

plt.subplot(1, 3, 1)
plt.title('Loss')
plt.ylabel(learningCurv)
plt.plot(history.history['loss'], label='loss')
plt.plot(history.history['val_loss'], label='Validated loss')
plt.legend(loc='upper right')

predicted3d = model.predict(noisySamples3d)
predicted2d = predicted3d.reshape(predicted3d.shape[0], predicted3d.shape[1])

randPick = []
for i in range(0, viewNumSamples):
    randPick.append(np.random.randint(0, len(origSamples2d) - 1))
xAxis = np.linspace(start, end, dataPointsPerSample)

plt.subplot(1, 3, 2)
plt.title('Orig. vs Noisy')
for i in range(0, viewNumSamples):
    
    yAxis = origSamples2d[randPick[i]]
    plt.plot(xAxis, yAxis, label='Orig. Line')
    yAxis = noisySamples2d[randPick[i]]
    plt.plot(xAxis, yAxis)

plt.subplot(1, 3, 3)
plt.title('Orig vs Predicted')
for i in range(0, viewNumSamples):
    yAxis = origSamples2d[randPick[i]]
    plt.plot(xAxis, yAxis)
    yAxis = predicted2d[randPick[i]]
    plt.plot(xAxis, yAxis)

plt.show()
