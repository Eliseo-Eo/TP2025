# 🔄 Sistema Distribuido de Cola de Tareas en Python

Este proyecto implementa un **sistema distribuido de procesamiento de tareas** en Python, combinando **multiprocesamiento** con **asincronía** mediante `asyncio`.
El sistema simula un escenario donde un **proceso maestro** distribuye tareas a varios **procesos worker**, los cuales ejecutan las tareas de forma concurrente y retornan los resultados al maestro.


# 🧠 ¿Qué hace este programa?

- El **maestro** genera una lista de tareas (números enteros).
- Cada número representa una tarea que:
  - Espera `n` segundos (simulando una tarea pesada).
  - Devuelve el cuadrado de ese número.
- Las tareas se distribuyen a varios **workers** (procesos hijos).
- Los **workers** ejecutan las tareas de forma **concurrente** usando `asyncio`.
- Cuando terminan, envían el resultado al **maestro**.
- Finalmente, el maestro recoge todos los resultados y apaga los workers de forma controlada.


# 🧩 Estructura del sistema

- `task_queue`: Cola para enviar tareas desde el maestro a los workers.
- `result_queue`: Cola para recibir resultados desde los workers.
- `stop_event`: Señal de apagado compartida entre procesos.
- `worker_func()`: Función que ejecuta cada worker.
- `tarea_simulada()`: Simula una tarea asíncrona (esperar y devolver un cuadrado).
- `maestro()`: Orquesta todo el flujo de trabajo.


# 📋 Requisitos

- Python **3.7 o superior**
- Compatible con **Windows**, **Linux** y **macOS**


# ▶️ Cómo ejecutar

1. Clona o descarga este repositorio.
2. Asegúrate de tener Python 3.7+ instalado.
3. Ejecuta el script desde la terminal o IDE:

```bash
python3 Sistema_Distribuido.py


# 💡 Ejemplo de salida esperada

[Maestro] Tareas agregadas: [2, 3, 1, 4, ...]
[Worker] Ejecutando tarea: esperar 2 segundos.
[Worker] Ejecutando tarea: esperar 3 segundos.
...
[Worker] Tarea terminada: 3^2 = 9
[Maestro] Resultado recibido: 3^2 = 9
...
[Maestro] Todos los workers se han detenido. Resultados finales:
 - 2^2 = 4
 - 3^2 = 9
 - ...