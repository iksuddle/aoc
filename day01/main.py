phonetic = {
    "one": "1",
    "two": "2",
    "three": "3",
    "four": "4",
    "five": "5",
    "six": "6",
    "seven": "7",
    "eight": "8",
    "nine": "9",
}

sum = 0

with open("input.txt", "r") as f:
    for line in f.readlines():
        tokens = []
        for k in phonetic:
            if k in line:
                tokens.append((phonetic[k], line.index(k)))
                tokens.append((phonetic[k], line.rindex(k)))
        tokens.extend([(c, i) for i, c in enumerate(line) if c.isdigit()])

        l, r = min(tokens, key=lambda x: x[1]), max(tokens, key=lambda x: x[1])

        sum += int(l[0] + r[0])

print(sum)
