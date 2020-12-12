p,d,r,w=0,1j,0,1+10j
for l in open("../input/day12.txt"):
 c,n=l[0],int(l[1:])
 b=2*(c in"RNE")-1
 a,i=1j**(n/90*b),1j*n
 if"F"==c:p+=n*d;r+=n*w
 if c in"RL":d*=a;w*=a
 if c in"NS":p+=n*b;w+=n*b
 if c in"EW":p+=i*b;w+=i*b
f=lambda x:abs(x.imag)+abs(x.real)
print f(p),f(r)