def Square(Number:float) -> float:
    return Number ** 2

def Is_Even(Number:int) -> bool:
   return Number % 2 ==0

if __name__ == '__main__':
  Numbers = [1, 2, 3, 4, 5, 6]
  Squared = list(map(Square,Numbers))
  Evens = list(filter(Is_Even, Numbers))
  print(Evens)