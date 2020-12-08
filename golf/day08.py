c=list(open("../input/day08.txt"))
def r():
 p=a=0
 v=[]
 while p not in v:
  if p>=len(c):print a;break
  v+=[p]
  o,n=c[p].split()
  n=int(n)
  a+=n*('c'in o)
  p+=(n-1)*('j'in o)+1
 return a
print r()
for i,x in enumerate(c):
 c[i]=x.replace(*'jn')if'j'in x else x.replace(*'nj')
 r()
 c[i]=x