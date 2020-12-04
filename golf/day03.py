f=lambda n:sum(l[(n*i)%(len(l)-1)]=='#'if n else l[(i/2)%(len(l)-1)]=='#'and 1-i%2 for i,l in enumerate(open("../input/day03.txt")))
print f(3),f(1)*f(3)*f(5)*f(7)*f(0)