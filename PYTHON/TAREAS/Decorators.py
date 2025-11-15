import time

#Manual
def Add_Greeting(fun):
    def wrapper():
        print("Before Calling Function")
        result = fun()
        print("After Calling")
        return result
    return wrapper

#Decorador
@Add_Greeting
def My_Function():
    return "Hello Word"

def testing():
    time.sleep(10)



#Manual
if __name__ == '__main__':
    #My_Function = Add_Greeting(My_Function)
    My_Function()
    Star_Time = time.time()
    testing()
    End_Time = time.time()
    print(f"{End_Time - Star_Time} seconds")
