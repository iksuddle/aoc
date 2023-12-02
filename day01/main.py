phonetic = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]

sum = 0

with open("input.txt", "r") as f:
    for line in f.readlines():
        tokens = []
        for k in phonetic:
            if k in line:
                tokens.append((str(phonetic.index(k)+1), line.index(k)))
                tokens.append((str(phonetic.index(k)+1), line.rindex(k)))
        tokens.extend([(c, i) for i, c in enumerate(line) if c.isdigit()])

        l, r = min(tokens, key=lambda x: x[1]), max(tokens, key=lambda x: x[1])

        sum += int(l[0] + r[0])

print(sum)
