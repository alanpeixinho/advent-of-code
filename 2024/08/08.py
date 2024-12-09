import math
import sys

def read_input():
    return [[c for c in line.strip()] for line in sys.stdin]

def main():
    data = read_input()
    rows = len(data)
    cols = len(data[0])


    anti_nodes = find_antinodes(rows, cols, find_nodes(data))
    print(f'# of anti-nodes: {len(anti_nodes)}')

    anti_nodes = find_antinodes_new(rows, cols, find_nodes(data))
    print(f'# of anti-nodes new model: {len(anti_nodes)}')


def print_table(data):
    for line in data:
        print(''.join(line))
    print()

def find_nodes(table):
    all_nodes = {}
    for row, line in enumerate(table):
        for col, node in enumerate(line):
            if node != '.':
                all_nodes[node] = [*all_nodes.get(node, []), (row, col)]
    return all_nodes

def valid_antinode(rows, cols, anti_node):
    return (0 <= anti_node[0] < rows) and (0 <= anti_node[1] < cols)

def find_antinodes(rows, cols, nodes_list):
    all_antinodes = []
    for node in nodes_list:
        pos = nodes_list[node]
        for i in range(len(pos)):
            for j in range(i + 1, len(pos)):
                dist = (pos[i][0] - pos[j][0], pos[i][1] - pos[j][1])
                all_antinodes.append((pos[i][0] + dist[0], pos[i][1] + dist[1]))
                all_antinodes.append((pos[j][0] - dist[0], pos[j][1] - dist[1]))

    return set(a for a in all_antinodes if valid_antinode(rows, cols, a))

def find_antinodes_new(rows, cols, nodes_list):
    maxdist = int(math.sqrt(rows ** 2 + cols ** 2))
    all_antinodes = []
    for node in nodes_list:
        pos = nodes_list[node]
        for i in range(len(pos)):
            for j in range(i + 1, len(pos)):
                dist = (pos[i][0] - pos[j][0], pos[i][1] - pos[j][1])
                for k in range(maxdist):
                    all_antinodes.append((pos[i][0] + k * dist[0], pos[i][1] + k * dist[1]))
                    all_antinodes.append((pos[j][0] - k * dist[0], pos[j][1] - k * dist[1]))

    return set(a for a in all_antinodes if valid_antinode(rows, cols, a))

if __name__ == '__main__':
    main()
