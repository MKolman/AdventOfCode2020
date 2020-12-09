n=map(int,open("../input/day09.txt"))
r=range
m=[x for i,x in enumerate(n[25:])if x not in[b+n[j+i]for j in r(25)for b in n[i:j+i]]][0]
print m,{sum(n[i:j]):min(n[i:j])+max(n[i:j])for j in r(len(n))for i in r(j-2)}[m]