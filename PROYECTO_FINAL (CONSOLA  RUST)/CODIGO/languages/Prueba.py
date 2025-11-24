import random
import time

# Leer el número de elementos desde el archivo "Tiempo.txt"
def read_element_count():
    try:
        with open("./languages/Tiempo.txt", "r") as file:
            # Leer el número de elementos desde el archivo
            count = int(file.read().strip())  # Convertir el contenido a un entero
            return count
    except FileNotFoundError:
        print("El archivo Tiempo.txt no se encuentra.")
        return None
    except ValueError:
        print("El contenido de Tiempo.txt no es un número válido.")
        return None

# Leer el número de elementos desde el archivo
num_elements = read_element_count()

if num_elements is not None:
    start = time.time()

    # Generar una lista de números aleatorios con la cantidad especificada
    data = [random.random() for _ in range(num_elements)]

    # Ordenar los datos
    data.sort()

    end = time.time()

    # Imprimir el tiempo de ejecución
    print(f"Tiempo de ejecución para ordenar {num_elements} elementos: {end - start:.8f} segundos")
else:
    print("No se pudo leer el número de elementos desde el archivo.")