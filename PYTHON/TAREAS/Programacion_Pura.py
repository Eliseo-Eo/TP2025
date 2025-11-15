import datetime

def Calculate_Circle_Area(Radius):
    return 3.14159 * Radius * Radius

def Format_Full_Name(first_name,last_name):
    return f"{first_name.strip().title()} {last_name.strip().title()}"

def Get_Greeting():
    Hour=datetime.datetime.now().hour
    return "Good Morning" if Hour < 12 else "Goog Evening"

Counter = 0
def Increment_And_Decrement():
    global Counter
    Counter =+ 1
    return Counter


def Sort_List(Numbers):
    Numbers.sort()
    return Numbers

if __name__ == '__main__':
    print("Pure Functions: ")
    print(f"Circle Area: {Calculate_Circle_Area(2.0)}")
    print(Format_Full_Name("Guillermo","Lopez"))
    
    print("Impure Functions: ")
    print(Get_Greeting())
    print(Increment_And_Decrement())
    ##print(Sort_List())