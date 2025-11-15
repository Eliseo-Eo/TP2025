import time
import multiprocessing

def Calculate_Square(Number: int) -> int:
    print(f"CALLED: {Number}")
    time.sleep(1)
    Result = Number * Number
    print(f"RESULT OF: {Number} = {Result}")
    return Result

def Main_Parallelism():
    Numbers = [1, 2, 3, 4, 5]
    with multiprocessing.Pool() as Pool:
        Results=Pool.map(Calculate_Square, Numbers)   
    print(F"RESULTS: {Results}")


if __name__ == "__main__":
    Main_Parallelism()