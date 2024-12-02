def main():
    """
    Sort, find diff, sum

    """
    l1, l2 = [], []
    with open("inputs/day1.txt") as f:
        for line in f:
            a, b = line.split()
            l1.append(int(a))
            l2.append(int(b))
    l1 = sorted(l1)
    l2 = sorted(l2)

    diffs = [abs(a - b) for a, b in zip(l1, l2)]
    print(f"part1: {sum(diffs)}")

    # Part 2
    score = 0
    for item in l1:
        score += item * l2.count(item)
    
    print(f"part2: {score}")



if __name__ == "__main__":
    main()
