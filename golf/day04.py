import re
r=[("byr","^(19[2-9][0-9]|200[012])$"),("iyr","^20(1[0-9]|20)$"),("eyr","^20(2[0-9]|30)$"),("hgt","^(1([5678][0-9]|9[0123])cm|(59|6[0-9]|7[0-6])in)$"),("hcl","^#[0-9a-f]{6}$"),("ecl","^(amb|blu|brn|gry|grn|hzl|oth)$")("pid","^[0-9]{9}$")]
a = [dict(v.split(':')for v in p.split())for p in open('../input/day04.txt').read().split("\n\n")]
print sum(len(p)-('cid'in p)==7 for p in a),sum(all(re.match(v,p.get(k,"")) for k,v in r) for p in a)