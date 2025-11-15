import asyncio

#Mutable State - DANGEROUS!!!
Counter = 0

async def Increment_Counter(Name, Times):
    global Counter
    for _ in range(Times):
        Temp = Counter
        await asyncio.sleep(0.001) #Sleep 1 ms
        Counter = Temp +1
        print(f"{Name}: Counter = {Counter}")
    return Counter
async def main():
    Tasks=[
        Increment_Counter("Task1", 3),
        Increment_Counter("Task1", 3),
        Increment_Counter("Task1", 3),
    ]
    Result = await asyncio.gather(*Tasks)
    print(f"Final Result {Counter}")

if __name__ == "__main__":
    asyncio.run(main())