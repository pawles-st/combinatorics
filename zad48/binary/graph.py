from matplotlib import pyplot as plt
import numpy as np
import math

f = open("results.txt", "r")

X = []
Y = []

for result in f.readlines():
    (n, sd) = result.rstrip().split(";")
    X.append(int(n))
    Y.append(int(sd))

#plt.plot(X, Y, label="s")
#plt.show()

a = 0.7155
A = np.arange(1, len(Y) + 1)
A = np.multiply(A, np.log(A)) * a

plt.plot(X, Y - A, label="asymptotic")
plt.show()
