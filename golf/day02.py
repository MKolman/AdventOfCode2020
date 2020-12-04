o=t=0
for l in open("../input/day02.txt"):
 n,(c,_),p=l.split()
 s,e=map(int,n.split('-'))
 o+=s<=p.count(c)<=e
 t+=(p[s-1]==c)^(p[e-1]==c)
print o,t