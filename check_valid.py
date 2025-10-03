n=31
pairs = [(1, 2), (3, 6), (4, 8), (5, 19), (7, 13), (9, 30), (10, 23), (11, 26), (12, 24), (14, 21), (15, 17), (16, 25), (18, 
29), (20, 28), (22, 27)]
sums = set([sum(pair) for pair in pairs])
diffs = set([(pair[0]-pair[1])%n for pair in pairs])
for pair in pairs:
    diffs.add(((pair[1]-pair[0])%n))
print(sums,len(sums))
print(diffs,len(diffs))
deltas={23, 29, 27, 15}
delta_sums = set()
for delta in deltas:
    good = True
    for s in sums:
        if (s*delta) in sums:
            good=False
            break
    if not good:
        raise Exception
