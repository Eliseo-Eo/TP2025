class Maybe:
    def __init__(self, value=None) -> None:
        self.value =value

    def is_nothing(self):
        return self.value is None
    
    def bind(self, func):
        "Aplicar si tenemos valor"
        if self.is_nothing():
            return Maybe(None)
        return func(self.value)

    def __str__(self) -> str:
        return f"Maybe({self.value})"

def Divide (val1, val2):
        if val2 ==0:
            return Maybe(None)
        return Maybe(val1/val2)

def sqrt(val):
        if val < 0:
            return Maybe(None)
        return Maybe (val ** 0.5)

def calculation(val, val2,val3):
        resultado1= val/val2
        resultado2= resultado1/val3
        resultado3 = resultado2 ** 0.5
        return resultado3

def safe_calculation(val1, val2, val3):
        return (Maybe(val1)
            .bind(lambda val: Divide(val, val2))
            .bind(lambda val: Divide(val, val3))
            .bind(lambda val: sqrt(val))
        )

if __name__ == '__main__':
   print(safe_calculation(100,8,9))