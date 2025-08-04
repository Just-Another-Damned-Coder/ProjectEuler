
def sumOfMultiples(factor: int, number: int) -> int:
    if number%factor == 0:
        limit = (number - factor)//factor
    else:
        limit = (number - number%factor)//factor
    return int(factor* (limit* (limit + 1)//2))

def main():
    t = int(input().strip())
    for a0 in range(t):
        n = int(input().strip())
        print(sumOfMultiples(3, n) + sumOfMultiples(5, n) - sumOfMultiples(15, n) )

if __name__ == '__main__':
    main()