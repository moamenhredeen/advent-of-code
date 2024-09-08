
import re

with open("1_input.txt") as f:
    content = f.readlines()
    sum = 0
    for line in content:
        numbers = re.findall(r'\d', line)
        sum += int(numbers[0] + numbers[-1])
    print(sum)
