import matplotlib.pyplot as plt
import numpy as np

num_samples_visualize = 1
noise_factor = .05

data = np.load('signal_waves_medium.npy')
x_val, y_val = data[:,0], data[:,1]

noisy_samples = []
for i in range(0, len(x_val)):
    if(i % 100 == 0):
        print(i)
    pure = np.array(y_val[i])
    noise = np.random.normal(0, 1, pure.shape)

    signal = pure + noise_factor * noise
    noisy_samples.append([x_val[i], signal])

np.save('signal_waves_noisy_medium.npy', noisy_samples)

for i in range(0, num_samples_visualize):
  random_index = np.random.randint(0, len(noisy_samples)-1)
  x_axis, y_axis = noisy_samples[random_index]
  plt.plot(x_axis, y_axis)
  plt.title(f'Visualization of noisy sample {random_index} ---- y: f(x) = x^2')
  plt.show()