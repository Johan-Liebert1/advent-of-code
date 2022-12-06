with open('src/day3/input.txt') as file:
    small_priority_start = 96;
    capital_priority_start = 38;

    lines = file.readlines()

    priority = 0
    i = 0

    while i < len(lines):
        first = lines[i]

        if len(first) == 0:
            break

        second = lines[i + 1]
        third = lines[i + 2]

        for char in first:
            if char in second:
                if char in third:
                    priority += ord(char) - (small_priority_start if ord(char) >= 97 else capital_priority_start)
                    break
        i += 3

    print(priority)
