#!/bin/python

starters = set()

while True:
    try:
        x=tuple(sorted(tuple(eval(input()))))
        if x not in starters:
            print(x)
        starters.add(x)
    except:
        break
