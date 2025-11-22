#include <iostream>
#include <vector>
#include <cstdlib>
#include <ctime>
#include <algorithm>
#include <fstream>  // Necesario para leer archivos
#include <iomanip>  // Necesario para setprecision

int main() {
    // Leer el tamaño desde un archivo de texto
    int SIZE = 0;
    std::ifstream input_file("./languages/Tiempo.txt");  // Nombre del archivo de texto

    if (input_file.is_open()) {
        input_file >> SIZE;  // Leer el valor del archivo
        input_file.close();  // Cerrar el archivo después de leer
    } else {
        std::cerr << "Error al abrir el archivo 'size.txt'" << std::endl;
        return 1;  // Terminar el programa si no se puede abrir el archivo
    }

    if (SIZE <= 0) {
        std::cerr << "El valor leído de 'size.txt' es inválido (debe ser mayor que 0)." << std::endl;
        return 1;
    }

    // Medir el tiempo de ejecución
    std::clock_t start_time = std::clock();

    // Generar números aleatorios
    std::vector<int> numbers(SIZE);

    std::srand(std::time(0));
    for (int i = 0; i < SIZE; ++i) {
        numbers[i] = std::rand();
    }

    // Ordenar los números
    std::sort(numbers.begin(), numbers.end());

    // Calcular el tiempo de ejecución
    double duration = (std::clock() - start_time) / (double) CLOCKS_PER_SEC;

    // Mostrar el tiempo de ejecución con 8 decimales
    std::cout << "Tiempo de ejecución para ordenar " << SIZE << " números aleatorios: "
              << std::fixed << std::setprecision(8) << duration << " segundos" << std::endl;

    return 0;
}
