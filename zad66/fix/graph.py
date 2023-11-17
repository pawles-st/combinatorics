from matplotlib import pyplot as plt
import numpy as np
import math

X = np.arange(1, 1000)
Y = []
results = open("results.txt", "r")

for line in results.readlines():
    Y.append(float(line.rstrip("\n")))

plt.plot(X, Y, label = "fix points")
plt.ylim([0.75, 1.27])
plt.yticks(np.arange(0.75, 1.25, 0.05)),
plt.plot(X, np.repeat(1, 999), color="red")
plt.show()
