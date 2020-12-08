from itertools import combinations as c
n=map(int,open("../input/day01.txt"))
print{x+y:x*y for x,y in c(n,2)}[2020],{x+y+z:x*y*z for x,y,z in c(n,3)}[2020]