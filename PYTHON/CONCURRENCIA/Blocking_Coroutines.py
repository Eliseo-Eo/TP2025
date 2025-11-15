import asyncio
import time

async def Blocking_Task(Name: str) -> str:
    print(f"{Name} STARTED")
    #time.sleep(2)
    await asyncio.sleep(2)
    print(f"{Name} FINISHED")
    return f"{Name} DONE"

async def Main():
    Start = time.time()

    Tasks=[
        Blocking_Task("TASK 1"),
        Blocking_Task("TASK 2"),
        Blocking_Task("TASK 3")
    ]
    Results = await asyncio.gather(*Tasks)
    End = time.time()
    print(f"RESULTS: {Results}")
    print(F"TOTAL TIME: {End - Start:.2f} SECONDS")

if __name__ == "__main__":
    asyncio.run(Main())
