import matplotlib.pyplot as plt
import numpy as np

np.random.seed(91)

numSamples = 100000
viewNumSamples = 7
dataPointsPerSample = 100
start = -3
end = 3

samples = []
xTrain = []
yTrain = []

def makeLinearEqu(x):
    a = np.random.uniform(-2, 2)
    b = np.random.uniform(-3, 3)
    c = np.random.uniform(-5, 5)
    return  a*x + b

xTrain = np.linspace(start, end, dataPointsPerSample)

for i in range(numSamples):
    if (i % 1000 == 0):
        print(i)
    yTrain = makeLinearEqu(xTrain)
    samples.append(yTrain)
    yTrain = []

np.savez('orig_plots', start=start, end=end, dataPointsPerSample=dataPointsPerSample, samples=samples)
# np.save('orig_plots', samples)

for i in range(viewNumSamples):
    xAxis = np.linspace(start, end, dataPointsPerSample)
    yAxis = samples[np.random.randint(0, numSamples - 1)]
    plt.plot(xAxis, yAxis)
plt.show()


