RED = 12
GREEN = 13
BLUE = 14

sum = 0

with open("input.txt", "r") as f:
    i = 1
    for line in f.readlines():
        _, r = line.split(":")
        r = r.strip()

        draws = [d.strip() for d in r.split(";")]
        cube_map = {}
        for d in draws:
            cubes = [x.strip() for x in d.split(",")]
            for c in cubes:
                info = c.split()
                if info[1] in cube_map:
                    cube_map[info[1]] = max(int(cube_map[info[1]]), int(info[0]))
                else:
                    cube_map[info[1]] = int(info[0])
        p = 1
        for a in cube_map.values():
            p *= a
        sum += p
        i += 1
print(sum)
