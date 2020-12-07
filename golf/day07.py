r=[l.split(" bags contain")for l in open("../input/day07.txt")]
f=lambda c:set().union(*[f(b).union({b})for b,w in r if c in w])
p=lambda l:[]if l[1]=="n"else[(int(w[1]),w[3:].rsplit(" ",1)[0])for w in l.split(",")]
g=lambda c:sum(m+m*g(c)for m,c in p(dict(r)[c]))
s="shiny gold"
print len(f(s)),g(s)