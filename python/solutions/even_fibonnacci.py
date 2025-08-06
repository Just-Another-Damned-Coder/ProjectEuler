from math import log, sqrt

def fibonacchi(number:int) -> list:
    array = [0,1]
    for i in range(2, number):
        array.append(array[i-1] + array[i-2])
    return array

def binet(N: int) -> int:
    return int(log(N * sqrt(5) )/ log(1.618)) + 5



number_of_cases = int(input().strip())
cases = list()
for _ in range(number_of_cases):
    cases.append(int(input().strip()))

limit = max(cases)
array = fibonacchi(limit)
for case in cases:
    numbers = filter(lambda x : x%2 ==0 and x < case, array)
    print(sum(numbers))