s={sum((l[i]in"BR")<<9-i for i in range(10))for l in open("../input/day05.txt")}
m=max(s)
print m,set(range(min(s),m))-s