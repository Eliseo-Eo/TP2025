#include <stdio.h>
#include <stdlib.h>
#include <time.h>

int compare(const void *a, const void *b) {
    return (*(int*)a - *(int*)b);
}

int main() {
    // Leer el tamaño desde el archivo Tiempo.txt
    FILE *file = fopen("./languages/Tiempo.txt", "r");
    if (file == NULL) {
        printf("No se pudo abrir el archivo Tiempo.txt.\n");
        return 1;
    }

    int SIZE;
    if (fscanf(file, "%d", &SIZE) != 1) {
        printf("Error leyendo el tamaño desde el archivo.\n");
        fclose(file);
        return 1;
    }

    fclose(file);  // Cerrar el archivo después de leer

    // Medir el tiempo de ejecución
    clock_t start_time = clock();

    // Generar números aleatorios con malloc
    int *numbers = (int *)malloc(SIZE * sizeof(int));  // Asignar memoria dinámicamente
    if (numbers == NULL) {
        printf("Error al asignar memoria.\n");
        return 1;
    }

    srand(time(0));

    for (int i = 0; i < SIZE; i++) {
        numbers[i] = rand();
    }

    // Ordenar los números
    qsort(numbers, SIZE, sizeof(int), compare);

    // Calcular el tiempo de ejecución
    double duration = (double)(clock() - start_time) / CLOCKS_PER_SEC;

    // Imprimir el tiempo de ejecución con 8 decimales
    printf("Tiempo de ejecución: %.8f segundos\n", duration);

    // Liberar la memoria después de usarla
    free(numbers);

    return 0;
}