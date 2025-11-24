### Benchmark Languages

El programa ejecuta diferentes scripts y programas de prueba escritos en varios lenguajes de programación y mide el rendimiento de cada uno. El lenguaje principal es RUST que utiliza **#Concurrencia y Paralelismo#**  Las pruebas incluyen la ejecución de código en Python, Kotlin, Rust, C++, y C, y reportan las siguientes métricas:

- **Tiempo de ejecución**: Medido en segundos.
- **Uso de memoria**: Diferencia entre la memoria total y la memoria libre en bytes.
- **Uso de CPU**: Porcentaje de uso de CPU antes y después de ejecutar el código.

### Lenguajes Soportados

1. **Python**: Ejecuta un script Python (`Prueba.py`).
2. **Kotlin**: Compila y ejecuta un archivo Kotlin (`Prueba.kt`).
3. **Rust**: Realiza una prueba de rendimiento que mide el tiempo de ordenación de una lista de números aleatorios.
4. **C++**: Compila y ejecuta un programa en C++ (`Prueba.cpp`).
5. **C**: Compila y ejecuta un programa en C (`Prueba.c`).

Cada uno de estos lenguajes se ejecuta en paralelo utilizando la biblioteca `rayon` para optimizar el tiempo de ejecución.

### Requisitos

Software Necesario. Asegúrate de tener instalados los siguientes programas y herramientas en tu sistema:

- **Rust**: Para compilar y ejecutar código Rust.
- **Python 3**: Para ejecutar el script de Python.
- **Java**: Para ejecutar el archivo compilado de Kotlin.
- **GCC** o **Clang**: Para compilar el código en C y C++.
- **Kotlin Compiler**: Para compilar el código de Kotlin.

### Librerías en Rust

Este proyecto utiliza las siguientes dependencias en Rust:

- `rayon`: Para la paralelización de tareas.
- `sysinfo`: Para obtener información del sistema como la memoria y la CPU.
- `rand`: Para generar números aleatorios.
- `std`: Para funciones estándar de Rust como manejo de archivos y ejecución de comandos del sistema.

### Instalación

1. Instala las dependencias de Rust:
    - bash
    - cargo build --release

Asegúrate de tener los archivos de prueba en el directorio adecuado:
    ./languages/Prueba.py (para Python)
    ./languages/Prueba.kt (para Kotlin)
    ./languages/Prueba.cpp (para C++)
    ./languages/Prueba.c (para C)
    ./languages/Tiempo.txt (archivo de tamaño para Rust)

Si no tienes estos archivos, puedes crearlos o adaptar los scripts de prueba para tus necesidades.

### Ejecución

Para ejecutar las pruebas de rendimiento en todos los lenguajes soportados, simplemente ejecuta el siguiente comando en la terminal:
    - bash
    - cargo run --release

El programa ejecutará cada prueba en paralelo y mostrará los resultados en la terminal. Para cada lenguaje, se mostrarán las siguientes métricas:

- **Tiempo de ejecución**: Medido en segundos.
- **Uso de memoria**: Diferencia entre la memoria total y la memoria libre en bytes.
- **Uso de CPU**: Porcentaje de uso de CPU antes y después de ejecutar el código.

### Ejemplo de salida

Memoria inicial: 1,024,000 bytes, Memoria final: 1,048,576 bytes, CPU inicial: 2.3%, CPU final: 3.2%, Tiempo: 0.4321 segundos. Lenguaje: Python
Memoria inicial: 1,024,000 bytes, Memoria final: 1,048,576 bytes, CPU inicial: 3.0%, CPU final: 4.1%, Tiempo: 0.3214 segundos. Lenguaje: Kotlin

### Cómo Funciona

El código se encarga de ejecutar las pruebas en cada lenguaje, medir el uso de memoria y CPU antes y después de la ejecución, y luego mostrar los resultados. Medición de memoria y CPU: El programa usa las librerías sys_info y sysinfo para obtener estadísticas del sistema, como la memoria total y libre, y el porcentaje de uso de la CPU. Ejecución de pruebas: Dependiendo del lenguaje seleccionado (Python, Kotlin, Rust, C++, C), el programa ejecuta un script o programa y mide el tiempo de ejecución.
Paralelización: Utiliza la librería rayon para ejecutar las pruebas de forma paralela, acelerando el proceso de comparación entre los lenguajes.
Resultados: Los resultados de cada prueba se imprimen en la terminal, mostrando la memoria utilizada, el uso de la CPU y el tiempo de ejecución.

### Cómo Personalizar

Si deseas modificar las pruebas o agregar más lenguajes, puedes hacerlo fácilmente modificando la función run_test y agregando el código para nuevos lenguajes o ajustando los existentes.