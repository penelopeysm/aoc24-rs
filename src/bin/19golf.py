n,_,*m,_=open("../../data/inputs/19.txt").read().split("\n")
from functools import*
f=cache(lambda x:sum([f(x[len(y):])for y in n.split(", ")if y<=x<y+'~'])if x else 1)
*g,=map(f,m)
print(sum(map(bool,g)),sum(g))
