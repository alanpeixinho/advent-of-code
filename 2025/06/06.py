import numpy as np
from sys import stdin
from io import StringIO

def compute(values, operation):
    if operation == '*':
        return np.prod(values)
    elif operation == '+':
        return np.sum(values)
    else:
        raise ValueError('What the hell man')

def read_input_1(raw_input):
    data = np.loadtxt(StringIO(raw_input), dtype=str)
    operations = data[-1, :]
    values = data[:-1, :].astype('int')
    return values, operations.ravel()

def pad_right(l, n, val):
    return l + [val] * (n - len(l))

def read_input_2(raw_input):
    lines = raw_input.split('\n')
    lines = [
        lines[r].ljust(len(lines[0]), ' ') for r in range(len(lines))
        if lines[r].strip()
    ]

    rows, cols = len(lines), len(lines[0])
    operations = []
    values = []
    values_operation = []
    for c in range(cols - 1, -1, -1):
        value = 0
        empty_column = True
        for r in range(0, rows - 1):
            if lines[r][c].isdigit():
                value = value * 10 + int(lines[r][c])
                empty_column = False
        if not empty_column: # skip empty columns
            values_operation.append(value)
        operation = lines[-1][c]
        if operation in ('*', '+'):
            operations.append(operation)
            # make sure all operations have same number of operands
            # (fill with 1 or 0 just for simplicity)
            values_operation = pad_right(values_operation, rows - 1,
                                         1 if operation == '*' else 0)
            values.append(values_operation)
            values_operation = []

    return np.array(values).T, np.array(operations)


def main():
    raw_input = stdin.read()
    values, operations = read_input_1(raw_input)
    total = sum(
        compute(values[:, c], operations[c]) for c in range(values.shape[1])
    )
    print('Homework answer: ', total)
    values, operations = read_input_2(raw_input)
    total = sum(
        compute(values[:, c], operations[c]) for c in range(values.shape[1])
    )
    print('Correct homework answer: ', total)


if __name__ == '__main__':
    main()
