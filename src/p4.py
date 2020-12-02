import re

file1 = open("./data/d3.txt")
lines = file1.readlines()

def check(min, max, letter, password):
    min = int(min)
    max = int(max)

    a = password[min - 1]
    b = password[max - 1]

    if a != b and (a == letter or b == letter):
        return 1
    else:
        return 0


def testLine(x):
    x = re.match(r"(\d+)-(\d+) (\w): (\w+)", x)
    return check(x.group(1), x.group(2), x.group(3), x.group(4))


print(sum(testLine(line) for line in lines))
