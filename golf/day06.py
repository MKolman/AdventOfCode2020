print sum(len(reduce(set.union,g.split(),set()))+1j*len(reduce(set.intersection,g.split(),set(g)))for g in open("../input/day06.txt").read().split('\n\n'))
