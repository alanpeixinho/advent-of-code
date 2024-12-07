import sys
from math import fabs

def read_input():
    all_reports = [
        [int(x) for x in line.strip().split() if x.isdigit()]
        for line in sys.stdin
    ]
    return all_reports

def diff(report):
    d = []
    for i in range(len(report) - 1):
        d.append(report[i+1] - report[i])
    return d

def sign(x):
    if x > 0:
        return 1
    elif x == 0:
        return 0
    else:
        return -1

def valid(diff_report):
    first_sign = sign(diff_report[0])
    for d in diff_report:
        if sign(d) != first_sign:
            return False
        if fabs(d) > 3 or fabs(d) <= 0:
            return False
    return True

def soft_valid(diff_report, report):
    sign_report = [sign(x) for x in diff_report]
    first_sign = sign_report[0]
    miss_idx = -1
    for i, d in enumerate(diff_report):
        if sign(d) != first_sign:
            if miss_idx > 0:
                return False
            miss_idx = i
            break
        if fabs(d) > 3 or fabs(d) <= 0:
            if miss_idx > -1:
                return False
            miss_idx = i
    if miss_idx > -1:
        for step in range(-1,2):
            new_report = [x for i, x in enumerate(report) if i != (miss_idx + step)]
            new_diffreport = diff(new_report)
            if valid(new_diffreport):
                miss_idx = -1
                break

    return miss_idx == -1

def soft_valid2(report):
    first_sign = report[1] - report[0]
    miss_idx = -1
    for i, d in enumerate(report[:-1]):
        cur_diff = report[i+1] - report[i]
        if sign(cur_diff) != first_sign:
            if miss_idx > 0:
                return False
            miss_idx = i
            break
        if fabs(d) > 3 or fabs(d) <= 0:
            if miss_idx > -1:
                return False
            miss_idx = i
    if miss_idx > -1:
        for step in range(-1,2):
            new_report = [x for i, x in enumerate(report) if i != (miss_idx + step)]
            new_diffreport = diff(new_report)
            if valid(new_diffreport):
                miss_idx = -1
                break

    return miss_idx == -1

def main():
    reports = read_input()
    diff_reports = list(map(diff, reports))
    valid_reports = list(map(valid, diff_reports))

    num_valid = sum(valid_reports)

    print(f'correct reports: {num_valid}')

    for i, report in enumerate(reports):
        if not valid_reports[i]:
            num_valid += soft_valid(diff_reports[i], report)


    print(f"total safe reports: {num_valid}")

if __name__ == '__main__':
    main()
