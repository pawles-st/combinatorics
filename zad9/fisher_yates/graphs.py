from matplotlib import pyplot as plt
import numpy as np
import math

results = open("results.txt", "r")

N = 875

X = [i for i in range(2, N + 1)]
Zeros = np.zeros(N - 1)
Ones = np.zeros(N - 1)
Cycles = np.zeros(N - 1)

for i in range(2, N + 1):
    line = results.readline()

    Zeros[i - 2] = float(results.readline().rstrip()[5:]) / 1000 / math.sqrt(1000)
    Ones[i - 2] = float(results.readline().rstrip()[6:]) / 1000 / math.sqrt(1000)
    Cycles[i - 2] = float(results.readline().rstrip()[9:]) / 1000 / math.sqrt(1000)

plt.plot(X[3:], Zeros[3:], label="no fixed pts")
plt.plot(X[3:], Ones[3:], label="one fixed pt")
plt.plot(X, [math.exp(-1) for i in range(2, N + 1)], label="1/e")
plt.xlabel("n")
plt.ylabel("% permutacji")
plt.title("")
plt.legend()
plt.show()

plt.plot(X, Cycles, label="cycles")
plt.plot(X, [math.log(i) + np.euler_gamma for i in range(2, N + 1)], label="ln(x) + gamma")
plt.xlabel("n")
plt.ylabel("average cycles")
plt.title("average no. cycles")
plt.legend()
plt.show()
