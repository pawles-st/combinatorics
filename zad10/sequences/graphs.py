from matplotlib import pyplot as plt
import numpy as np

results = open("results.txt", "r")

N = 50

X = [i for i in range(1, N + 1)]
A = np.zeros(N)
B = np.zeros(N)
A_total = np.zeros(N)

for i in range(0, N):
    line = results.readline()

    A[i] = float(results.readline().rstrip()[8:])
    B[i] = float(results.readline().rstrip()[8:])
    A_total[i] = float(results.readline().rstrip()[11:])

plt.plot(X, A, label="aaa")
plt.plot(X, B, label="abb")
plt.xlabel("długość ciągu")
plt.ylabel("% wystąpień aaa")
plt.title("% wystąpień wzorca aaa/abb")
plt.legend()
plt.show()

plt.plot(X, A_total, label="aaa total")
plt.xlabel("długość ciągu")
plt.ylabel("średnia liczba aaa")
plt.title("średnie wystąpienia wzorca aaa")
plt.legend()
plt.show()
