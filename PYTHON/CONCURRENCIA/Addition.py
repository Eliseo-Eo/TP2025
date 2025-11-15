import asyncio
from cgitb import reset

async def Add_Numbers(Num1, Num2):
    await asyncio.sleep(1)
    Result = Num1 + Num2
    print(f"{Num1} + {Num2} = {Result}")
    return Result

async def main():
    Result1 = await Add_Numbers(5, 5)
    Result2 = await Add_Numbers(10,7)
    Result3 = await Add_Numbers(2,8)

    print(f"Results: {Result1}, {Result2}, {Result3}")

if __name__ == "__main__":
    asyncio.run(main())