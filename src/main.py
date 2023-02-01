import time
def lowestmultiple(inp):
    working = True
    curr = 0
    numbers = ['1', '0']
    while working:
        curr += inp
        currstring = str(curr)
        passing = True
        for char in currstring:
            if char not in numbers:
                passing = False
        if passing:
            print(currstring)
            working = False
    return curr
start = time.perf_counter_ns()
inp = int(input('Please enter your number'))
curr = lowestmultiple(inp)
print(f'{curr/inp} * {inp} = {curr}')
end = time.perf_counter_ns()
print(f'milli second duration: {(end-start)/1000000}')

