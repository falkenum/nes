
from builtins import bytes

f = open("instr-6052.txt", "r")

s = f.readlines()

s = sorted(s, key=lambda x : x[4:].lower() )
f.close();
f = open("instr-sorted.txt", "w")

for line in s:
    f.write("fn %s() {}\n", line[0, 3].lower())
    f.write(line)


f.close()
