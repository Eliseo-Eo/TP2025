import asyncio

async def Count_Items(Name, Start_Value, Times):
    Current = Start_Value
    for _ in range(Times):
        await asyncio.sleep(0.001)
        Current = Current + 1
        print(f"{Name}: Local Counter = {Current}")
    return Current

async def main():
    Tasks = [
        Count_Items("Task", 0, 3),
        Count_Items("Task", 0, 3),
        Count_Items("Task", 0, 3),
    ]
    Results = await asyncio.gather(*Tasks)
    Total = sum(Results)
    print(f"Total {Total}")

if __name__ == "__main__":
        asyncio.run(main())