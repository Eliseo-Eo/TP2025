def Apply_To_All(Func, Items):
    return [Func(Item) for Item in Items]

if __name__ == '__main__':

    #1) ARG: EXPRESSION
    Square = lambda Num: Num ** 2
    print(Square(5))

    #2) Multiple Artument
    Add =lambda Num_1, Num_2: Num_1 +Num_2
    print(Add(4,5))

    #3) Multiple Arguments: Default Values
    Greet = lambda Name, Greeting="Hello": f"{Greeting}, {Name}"
    print(Greet("Alice"))
    print(Greet("Bob", "Hi"))
    
    #4) Print of List
    Nums = [1, 2, 3, 4, 5,6]
    Square = list(map(lambda Num: Num**2, Nums))
    print(Square) 
    #List
    Students = [
        ("Alice", 85),
        ("Bob", 90),
        ("Charlie", 78),
        ("Diana", 92)
    ]
    #Call of Function
    By_Age = sorted(Students, key=lambda Student: Student[1])
    print(By_Age) 

    #Call of Function
    Cubed = Apply_To_All(lambda Num: Num ** 3, Nums)
    print(Cubed)