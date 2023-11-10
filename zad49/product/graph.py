from matplotlib import pyplot as plt
import numpy as np
import math

N = 10000

results = open("results.txt", "r")

X = np.arange(0, N + 1)
Y = np.empty(N + 1)
Y[0] = 1

for i in range(1, N + 1):
    Y[i] = int(results.readline().rstrip());

plt.plot(X, Y, label="coefficients")
plt.show()

A = np.exp(math.pi * np.sqrt(2 * X / 3)) / 4 / X / math.sqrt(3)

plt.plot(X, Y / A, label="complexity")
plt.show()
