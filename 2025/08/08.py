import numpy as np
from sys import stdin
from collections import Counter


def read_input():
    return np.loadtxt(stdin, delimiter=',', dtype=int)


def euclidean_distance(points):
    s = np.sum(points**2, axis=1, keepdims=True)
    dist = np.sqrt(-2 * points.dot(points.T) + s + s.T, dtype='float32')
    np.fill_diagonal(dist, np.inf)
    return dist


def join_circuits(circuits, id1, id2):
    circuit = min(circuits[id1], circuits[id2])
    joined_circuits = (circuits == circuits[id1]) | (circuits == circuits[id2])
    circuits[joined_circuits] = circuit
    return circuit


def join_topk(distances, k):
    dist = distances.copy()
    npoints, _ = distances.shape

    circuits = np.arange(npoints, dtype=int)
    for i in range(k):
        (idx1, idx2) = np.unravel_index(dist.argmin(), dist.shape)
        circuit = join_circuits(circuits, idx1, idx2)
        dist[idx1, idx2] = dist[idx2, idx1] = np.inf

    return circuits


def join_all(distances):
    dist = distances.copy()
    npoints, _ = distances.shape

    circuits = np.arange(npoints, dtype=int)
    while True:
        (idx1, idx2) = np.unravel_index(dist.argmin(), dist.shape)
        circuit = join_circuits(circuits, idx1, idx2)
        dist[idx1, idx2] = dist[idx2, idx1] = np.inf
        if np.all(circuits == circuit):
            return idx1, idx2


def main():
    data = read_input()

    distances = euclidean_distance(data)
    circuits = join_topk(distances, 1000)
    top3_circuits = sorted(Counter(circuits).values())[-3:]
    print('Product of 3 largest circuits: ', np.prod(top3_circuits))

    last_pair = join_all(distances)
    x1, x2 = [int(data[p][0]) for p in last_pair]
    print('Product of X coordinate of last two connected points', x1 * x2)


if __name__ == '__main__':
    main()
