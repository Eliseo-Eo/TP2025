import asyncio

async def Fault_Coroutine():
    await asyncio.sleep(1)
    raise ValueError("Something went wrong") #Levanta un error

async def main():
    try:
        #El blocke de codigo que se ejecuta
        await Fault_Coroutine()

    except ValueError as e:
        print(f"Cache la Except {e}")

if __name__ == "__main__":
    asyncio.run(main())