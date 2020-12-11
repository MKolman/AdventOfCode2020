n=[0]+list(map(int,open("../input/day10.txt")))
n.sort()
d=[y-x for x,y in zip(n,n[1:])]
m=[0]*len(n)
for i,c in enumerate(n[::-1]):m[i]=sum(m[i-len({c+1,c+2,c+3}&set(n)):i])or 1
print d.count(1)*(d.count(3)+1),m[-1]