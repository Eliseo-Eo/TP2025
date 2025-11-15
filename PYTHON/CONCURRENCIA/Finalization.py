import asyncio

async def Cancellable_Task():
    try:
        while True:
            print("Task is Running...")
            await asyncio.sleep(0.5)
    except asyncio.CancelledError:
            print("Task Cancelled")
    finally:
         print("Cleanup")
async def main():
    Task = asyncio.create_task(Cancellable_Task())
    await asyncio.sleep(2.0)
    Task.cancel()
    #Esperar a la cancelacion de la rutina
    try:
        await Task
    except asyncio.CancelledError:
         print("Main was cancelled")

if __name__ == "__main__":
    asyncio.run(main())