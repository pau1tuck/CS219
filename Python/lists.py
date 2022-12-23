## LISTS

# Heterogeneous list
myList = ["python", 0.4, true]

# Manipulating lists
list_sum = sum([4, 8, 9, 10, 24, 3])
second_from_last = myList[-2]
slice_first_two = myList[:2] # (e.g. [2:5], [6:], [4:-1])
make_a_copy = myList[:]
every_third = x[::3] # stride
five_to_three = [5:2:-1] # stride


x.extend([4, 5, 6]) # concatenate
x.append(0) # append
y = x + [4, 5, 6] # new list addition
z = len(x)
_, y, z = [1, 2, 3] # unpack variables with throwaway value


# 'in' operator
if 0 in [1, 3, 4, 5]: print("True")