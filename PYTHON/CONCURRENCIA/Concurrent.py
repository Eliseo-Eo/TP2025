import asyncio

async def Calculate_Square(Number: int) -> int:
    #SIMULAR UN DELAY DE 1 SEGUNDO
    print(f"CALL FROM {Number}")
    await asyncio.sleep(1)
    Result = Number * Number
    print(f"RESULT OF: {Number} = {Result}")
    return Result

async def Main_Async():
    Numbers = [1, 2, 3, 4, 5]
    #CREAMOS TAREAS A REALIZAR
    Tasks = [Calculate_Square(Num) for Num in Numbers]

    #LLAMAR FUNCIONES
    #GATHER ES CALENDARIZADO A 5 TAREAS
    Results = await asyncio.gather(*Tasks)
    print(f"RESULTS: {Results}")

if __name__ == "__main__":
    #LANZA LA CORUTINA PRINCIPAÑ
    asyncio.run(Main_Async())