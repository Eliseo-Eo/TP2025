import asyncio

async def count():
    print("One")
    await asyncio.sleep(1)
    print("Two")
    return "Result"

async def main():
    #Agrupa la ejecucion de varias corutinas en una tarea.
    #await asyncio.gather(count(), count())
    #No-Blocking
    tasks = asyncio.create_task(count()) #Tasks Future
    #print(tasks)
    print("Done creating tasks")
    #Cuando se use el futuro
    result = await tasks #Esperar a la ejecuin de las tareas.
    print(result)

if __name__ == "__main__":
    asyncio.run(main())