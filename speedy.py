import random
import itertools 
import copy


'''
This algorithm is based off of:
A FAST ALGORITHM FOR FINDING STRONG STARTERS by J. H. DINITZt AND D. R. STINSON 1981
'''


def is_maximal(n,starter,all_pairs,diffs):
    good = False
    sums = [sum(pair)%n  for pair in starter]
    for pair in all_pairs:
        if sum(pair)%n>0 and (pair[1]-pair[0])%n in diffs and (pair[0]-pair[1])%n in diffs and sum(pair)%n not in sums:
            good=True
    return not good

def calculate_T(n,u_1,u_2,d,unused):
    assert u_1 in unused
    assert u_2 in unused
    T_1 = ((u_1-d)%n,(u_1+d)%n)
    T_2 = ((u_2-d)%n,(u_2+d)%n)
    T = [T_1[0],T_1[1],T_2[0],T_2[1]]
    return T


starter = [] # S={}
t=25
n=2*t+1
all_pairs = list(itertools.combinations(range(1,n),2))
deficiency=t
D=list(range(1,t+1))  # Differences
unused=list(range(1,n))  # Unused
unused_sums = list(range(1,n))



stack=[] #used for backtracking


def can_insert(pair,starter,unused,unused_sums,D,n):
    (a,b) = pair
    return a in unused and b in unused and (a+b)%n in unused_sums and min((a-b)%n,(b-a)%n) in D


def insert(pair,starter,unused,unused_sums,D,n):
    a=pair[0]
    b=pair[1]
    assert a in unused
    assert b in unused
    assert (a+b)%n in unused_sums
    assert min((a-b)%n,(b-a)%n) in D

    unused.remove(a)
    unused.remove(b)
    unused_sums.remove((a+b)%n)
    D.remove(min((a-b)%n,(b-a)%n))
    starter.append(pair)
def remove(pair,starter,unused,unused_sums,D,n):
    a=pair[0]
    b=pair[1]
    assert a not in unused
    assert b not in unused
    assert (a+b)%n not in unused_sums
    assert min((a-b)%n,(b-a)%n) not in D

    unused.append(a)
    unused.append(b)
    unused_sums.append((a+b)%n)
    D.append(min((a-b)%n,(b-a)%n))
    starter.remove(pair)

def get_pair_by_element(starter,element):
    for pair in starter:
        if element in pair:
            return pair
    return None
def get_pair_by_diff(starter,n,diff):
    for pair in starter:
        if min((pair[0]-pair[1])%n,(pair[1]-pair[0])%n) in pair:
            return pair
    return None



def A(u1,u2,d,starter,n,unused,unused_sums,D,stack):
    stack.append((u1,u2,d))
    T=calculate_T(n,u1,u2,d,unused)
    could_do_a=False
    for w in T:
        if w==0:
            continue
        x=(u1+w)%n
        y=(u2+w)%n
        if w in unused and x in unused_sums and can_insert((u1,w),starter,unused,unused_sums,D,n):
            insert((u1,w),starter,unused,unused_sums,D,n)
            could_do_a=True
            break
        elif w in unused and y in unused_sums and can_insert((u2,w),starter,unused,unused_sums,D,n):
            insert((u2,w),starter,unused,unused_sums,D,n)
            could_do_a=True
            break
    return could_do_a
def B(u1,u2,d,starter,n,unused,unused_sums,D,stack):
    stack.append((u1,u2,d))
    T=calculate_T(n,u1,u2,d,unused)
    i=0
    could_complete_b = False
    based_w=-1
    for w in T:
        if w==0:
            continue
        x=(u1+w)%n
        y=(u2+w)%n
        x_i=0
        y_i=0
        if w not in unused and x in unused_sums:
            (x_i,y_i) = p=get_pair_by_element(starter,w)
            remove(p,starter,unused,unused_sums,D,n)
            if can_insert((u2,w),starter,unused,unused_sums,D,n):
                insert((u2,w),starter,unused,unused_sums,D,n)
            else:
                insert(p,starter,unused,unused_sums,D,n)
                continue
            i=1
            could_complete_b = True
            based_w=w
            break
        elif w not in unused and y in unused_sums:
            (x_i,y_i) = p =get_pair_by_element(starter,w)
            remove(p,starter,unused,unused_sums,D,n)
            if can_insert((u2,w),starter,unused,unused_sums,D,n):
                insert((u2,w),starter,unused,unused_sums,D,n)
            else:
                insert(p,starter,unused,unused_sums,D,n)
                continue

            i=2
            could_complete_b = True
            based_w=w
            break
    
    if i==1 and based_w==x_i:
        stack.append((u2,y_i,min((u2-y_i)%n,(y_i-u2)%n)))
    if i==1 and based_w==y_i:
        stack.append((u2,x_i,min((u2-y_i)%n,(y_i-u2)%n)))
    if i==2 and based_w==x_i:
        stack.append((u1,y_i,min((u2-y_i)%n,(y_i-u2)%n)))
    if i==2 and based_w==y_i:
        stack.append((u1,x_i,min((u2-y_i)%n,(y_i-u2)%n)))
    return could_complete_b

def C(stack):
    stack.pop()
    return stack[-1]

def D_(u1,u2,d,D,stack):
    stack.append((u1,u2,d))
    x=random.choice(D)
    while(x==d):
        x=random.choice(D)
    return (u1,u2,d)

def E(u1,u2,d,starter,n,unused,unused_sums,D,stack):
    stack.append((u1,u2,d))
    d_1 = min((u1-u2)%n,(u2-u1)%n)
    if d_1 not in D and (u1+u2)%n in unused_sums:
        (x,y) = p=get_pair_by_diff(starter,n,d_1)
        remove(p,starter,unused,unused_sums,D,n)
        insert((u1,u2),starter,unused,unused_sums,D,n)
        (u1,u2) = (x,y)
    else:
        print(":(") # should be an error/fail but I have not had this case yet may require some more debugging
    return (u1,u2,d)



def pick_random_unused(unused,D):
    u1=random.choice(unused)
    u2=random.choice(unused)
    diff = random.choice(D)
    while u1==u2:
        u1=random.choice(unused)
        u2=random.choice(unused)
    return (u1,u2,diff)


def speedy(starter,n,deficiency,D,unused,unused_sums):
    #lowkey kinda cursed loop, there may be some bugs since the paper 
    while deficiency>0:

        temp_data = (u1,u2,d) = pick_random_unused(unused,D) #choose any distinct u1,u2, and d

        could_do_a = A(u1,u2,d,starter,n,unused,unused_sums,D,stack)
        if could_do_a:
            deficiency-=1
            if deficiency>0:
                (u1,u2,d) = pick_random_unused(unused,D) #choose any distinct u1,u2, and d
        else:
            could_do_b=False
            while not could_do_b:
                could_do_b = B(u1,u2,d,starter,n,unused,unused_sums,D,stack)
                if not could_do_b:
                    if len(stack)>0:
                        (u1,u2,d) = C(stack)
                        break
                    elif len(D)>0:
                        (u1,u2,d) = D_(u1,u2,d,D,stack)
                        break
                    else:
                        print("E")
                        (u1,u2,d) = E(u1,u2,d,D,stack)
                        break
                else:
                    (u1,u2,d) = stack[-1]
        
        #I think I have some bug somewhere in the algorithm so this is how i am checking for fails
        if len(stack)>1000*n: 
            print("failed")
            return None

    return starter
                        






print(f"N = {n}")
x = speedy(starter,n,deficiency,D,unused,unused_sums)
while not x:
    starter = [] # S={}
    deficiency=t
    D=list(range(1,t+1))  # Differences
    unused=list(range(1,n))  # Unused
    unused_sums = list(range(1,n))
    stack=[]
    x = speedy(starter,n,deficiency,D,unused,unused_sums)
print(starter)
