s=[sum(2**(9-i)*(l[i]in"BR")for i in range(10))for l in open("../input/day05.txt")]
print max(s),[i-1 for i in s if i-2 in s and i-1 not in s]