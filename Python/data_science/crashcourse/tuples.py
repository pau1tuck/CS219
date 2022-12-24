## TUPLES (i.e. Lists' immutable cousins)

# A convenient way to return multiple values from a function

def sum_and_product(x, y):
	return (x + y), (x * y)
s, p = sum_and_product(5, 10) # s = 15, p = 50