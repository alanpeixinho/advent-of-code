import sys
from collections import Counter

def read_input():
    all_ids = [
        tuple(int(x) for x in line.strip().split() if x.isdigit())
        for line in sys.stdin
    ]

    list1, list2 = zip(*all_ids)
    return sorted(list1), sorted(list2)

def main():
    list1, list2 = read_input()

    total_distance = sum(abs(x1 - x2) for x1, x2 in zip(list1, list2))

    hist2 = Counter(list2)
    similarity_score = sum(x * hist2.get(x, 0) for x in list1)

    print(f"total distance: {total_distance}")
    print(f"similarity score: {similarity_score}")


if __name__ == "__main__":
    main()
