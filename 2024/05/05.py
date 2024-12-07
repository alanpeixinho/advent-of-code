from tqdm import tqdm
from itertools import takewhile, permutations
import sys
from functools import cmp_to_key, partial


def read_input():
    all_rules = takewhile(
        lambda rule: len(rule) > 1,
        (tuple(line.strip().split('|')) for line in sys.stdin))

    all_prints = (line.strip().split(',') for line in sys.stdin)

    aggregated_rules = {}

    for before, after in all_rules:
        agg = aggregated_rules.get(before, set())
        aggregated_rules[before] = {*agg, after}

    return (aggregated_rules, list(all_prints))


def is_print_correct(cur_print, all_rules):
    printed = set()
    for page in cur_print:
        for after_pages in all_rules.get(page, set()):
            if after_pages in printed:
                return False
        printed = {*printed, page}
    return True


def compute_middle_sum(prints):
    return sum(int(p[len(p) // 2]) for p in prints)

def compare_with_rules(p1, p2, all_rules):
    if p2 in all_rules.get(p1, set()):
        return -1
    else:
        return 1

def main():
    all_rules, all_prints = read_input()

    correct = []
    incorrect = []
    for i, cur_print in enumerate(all_prints):
        if is_print_correct(cur_print, all_rules):
            correct.append(i)
        else:
            incorrect.append(i)

    correct_middle_sum = compute_middle_sum(all_prints[i] for i in correct)
    print(f"correct print sum: {correct_middle_sum}")

    compare = partial(compare_with_rules, all_rules=all_rules)
    for i in incorrect:
        sorted_print = sorted(all_prints[i], key=cmp_to_key(compare))
        all_prints[i] = sorted_print

    fixed_middle_sum = compute_middle_sum(all_prints[i] for i in incorrect)
    print(f"fixed print sum: {fixed_middle_sum}")


if __name__ == '__main__':
    main()
