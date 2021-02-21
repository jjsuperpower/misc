import matplotlib.pyplot as plt

import numpy as np

np.random.seed(40)

viewNumSamples = 2
# noiseFactor = .3

loaded = np.load('orig_plots.npz')
start = loaded['start']
end = loaded['end']
dataPointsPerSample = loaded['dataPointsPerSample']
origSamples = loaded['samples']

noisySamples = []

for i in range(0, len(origSamples)):
    if (i % 1000 == 0):
        print(i)
    noiseFactor = np.random.uniform(4, 8)
    pure = np.array(origSamples[i])
    yNoise = np.random.normal(0, noiseFactor, pure.shape)
    signal = pure + yNoise
    noisySamples.append(signal)

np.savez('noisy_plots', start=start, end=end, dataPointsPerSample=dataPointsPerSample, samples=noisySamples)

for i in range(viewNumSamples):
    randPick = np.random.randint(0, len(noisySamples) - 1)
    xAxis = np.linspace(start, end, dataPointsPerSample)
    yAxis = noisySamples[randPick]
    plt.plot(xAxis, yAxis)
    yAxis = origSamples[randPick]
    plt.plot(xAxis, yAxis)
plt.show()
