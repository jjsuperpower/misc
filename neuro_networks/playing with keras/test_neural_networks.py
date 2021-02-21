import matplotlib.pyplot as plt
import keras
from keras.models import Sequential, load_model
from keras.layers import Conv1D, Dense, MaxPool1D, MaxPool1D, Flatten, Reshape, Dropout, InputLayer
from keras.constraints import max_norm
import matplotlib.pyplot as plt
import numpy as np
import math


viewNumSamples = 1
inputShape = (1000, 1)
learningCurv = 'mean_squared_error'

print('\n')
print('Loading Original plots')
origLoad = np.load('x_sens_array.npy')


print('Processing data')
start = 1
end = 100
dataPointsPerSample = 100
origSamples2d = origLoad[0:100]

# Convert 2d (100000,100) array to 3d (100000,100,1) array
origSamples3d = origSamples2d.reshape(1 ,origSamples2d.shape[0], 1)


model = load_model('Trained_Neuro_Net.h5')
history = model

predicted3d = model.predict(origSamples3d)
predicted1d = predicted3d.reshape(predicted3d.shape[1])

randPick = []
for i in range(0, viewNumSamples):
    randPick.append(np.random.randint(0, len(origSamples2d) - 1))
xAxis = np.linspace(start, end, dataPointsPerSample)

print('Plotting Results')
plt.title('Orig vs Predicted')
for i in range(0, viewNumSamples):
    yAxis = origSamples2d
    plt.plot(xAxis, yAxis)
    yAxis = predicted1d
    plt.plot(xAxis, yAxis)

plt.show()