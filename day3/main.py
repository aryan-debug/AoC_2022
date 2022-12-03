def get_priority(letter):
    if "a" <= letter <= "z":
        return ord(letter) - 96
    return ord(letter) - 38


def get_common(supplies):
    common = set(supplies[0]).intersection(set(supplies[1]))
    for i in range(len(supplies) - 1):
        common = common.intersection(set(supplies[i + 1]))
    return common


def part1(supplies):
    sum = 0
    for supply in supplies:
        mid = len(supply) // 2
        first_half, second_half = supply[:mid], supply[mid:]
        common = get_common([first_half, second_half])
        priority = get_priority([*common][0])
        sum += priority
    return sum


def part2(supplies):
    sum = 0
    for i in range(0, len(supplies), 3):
        three_supplies = supplies[i : i + 3]
        common = get_common(three_supplies)
        priority = get_priority([*common][0])
        sum += priority
    return sum


def main():
    with open("input.txt") as file:
        supplies = [item.strip() for item in file]
    print(f"Part 1 solution: {part1(supplies)}")
    print(f"Part 2 solution: {part2(supplies)}")


main()
