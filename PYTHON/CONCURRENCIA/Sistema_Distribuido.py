import asyncio
import multiprocessing
import time
import random
import queue
from typing import Any

# -------- TAREA SIMULADA (Ejemplo) --------
async def tarea_simulada(numero: int):
    print(f"[Worker] Ejecutando tarea: esperar {numero} segundos.")
    await asyncio.sleep(numero)
    resultado = numero ** 2
    print(f"[Worker] Tarea terminada: {numero}^2 = {resultado}")
    return resultado

# -------- FUNCIÓN WORKER --------
def worker_func(task_queue: Any, result_queue: Any, stop_event: Any):
    async def procesar_tareas():
        while not stop_event.is_set():
            try:
                tarea = task_queue.get(timeout=0.1)
            except queue.Empty:
                await asyncio.sleep(0.1)
                continue

            if tarea == "STOP":
                print("[Worker] Señal de apagado recibida.")
                stop_event.set()
                break

            try:
                resultado = await tarea_simulada(tarea)
                result_queue.put((tarea, resultado))
            except Exception as e:
                print(f"[Worker] Error al procesar la tarea {tarea}: {e}")
                result_queue.put((tarea, f"Error: {e}"))

    try:
        asyncio.run(procesar_tareas())
    except RuntimeError as e:
        print(f"[Worker] Error al iniciar asyncio: {e}")

# -------- FUNCIÓN MAESTRO --------
def maestro():
    NUM_WORKERS = 3

    task_queue = multiprocessing.Queue()
    result_queue = multiprocessing.Queue()
    stop_event = multiprocessing.Event()

    # Crear y arrancar los workers
    workers = []
    for i in range(NUM_WORKERS):
        p = multiprocessing.Process(target=worker_func, args=(task_queue, result_queue, stop_event))
        p.start()
        workers.append(p)

    # Agregar tareas a la cola
    tareas = [random.randint(1, 5) for _ in range(10)]
    print(f"[Maestro] Tareas agregadas: {tareas}")
    for tarea in tareas:
        task_queue.put(tarea)

    # Recoger resultados
    resultados = []
    while len(resultados) < len(tareas):
        tarea, resultado = result_queue.get()
        print(f"[Maestro] Resultado recibido: {tarea}^2 = {resultado}")
        resultados.append((tarea, resultado))

    # Señal de apagado
    for _ in workers:
        task_queue.put("STOP")

    # Esperar a que todos los workers terminen
    for p in workers:
        p.join()

    print("\n[Maestro] Todos los workers se han detenido. Resultados finales:")
    for tarea, resultado in resultados:
        print(f" - {tarea}^2 = {resultado}")

# -------- EJECUCIÓN PRINCIPAL --------
if __name__ == '__main__':
    multiprocessing.set_start_method("spawn")  # Importante para Windows
    maestro()