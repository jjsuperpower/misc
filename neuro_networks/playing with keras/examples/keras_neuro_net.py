import matplotlib.pyplot as plt
import numpy as np

num_samples = 100000
num_elements = 1
interval_per_element = 0.01
total_num_elements = int(num_elements / interval_per_element)
starting_point = int(0 - .5 * total_num_elements)

num_samples_visualize = 1

samples = []
xs = []
ys = []

for j in range(0, num_samples):
    if(j % 100 == 0):
        print(j)
    for i in range(starting_point, total_num_elements):
        x_val = i * interval_per_element
        y_val = x_val ** 2
        xs.append(x_val)
        ys.append(y_val)
    samples.append((xs, ys))
    xs = []
    ys = []

print(np.shape(np.array(samples[0][0])))
np.save('signal_waves_medium.npy', samples)

for i in range(0, num_samples_visualize):
    random_index = np.random.randint(0, len(samples)-1)
    x_axis, y_axis = samples[random_index]
    plt.plot(x_axis, y_axis)
    plt.title(f'Visualization of sample {random_index} ---- y: f(x) = x^2')
    plt.show()