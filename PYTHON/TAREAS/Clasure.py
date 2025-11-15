def Outer_Functions(Num):
    Outer_Variable = Num
    def Inner_Function():
        return Outer_Variable * 2
    return Inner_Function
 
class Counter:
    def __init__(self, start=0) -> None:
        self.Counter = start 

    def Increment(self):
        self.Counter +=1
        return self.Counter
    
    def get_counter(self):
        return self.Counter

def Create_Counter(start=0):
        Counter = start
        def Increment():
            nonlocal Counter
            Counter += 1
            return Counter
        return Increment

if __name__ == '__main__':
    My_Closure = Outer_Functions(20)
    print(My_Closure())

    Counter1 = Counter(10)
    Counter2 = Counter(100)

    print(Counter1.Increment())
    Counter1.Counter = 99 #Corrupted

    Counter3 = Create_Counter(10)
    print(Counter3())
