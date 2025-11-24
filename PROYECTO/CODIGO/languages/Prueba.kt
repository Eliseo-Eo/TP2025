import kotlin.random.Random
import kotlin.system.measureTimeMillis
import java.io.File

// Función para leer el número de elementos desde el archivo Tiempo.txt
fun readElementCount(): Int? {
    return try {
        val file = File("./languages/Tiempo.txt")
        val content = file.readText().trim()
        content.toInt()
    } catch (e: Exception) {
        println("Error al leer o convertir el archivo Tiempo.txt: ${e.message}")
        null
    }
}

fun main() {
    // Leer el número de elementos desde el archivo
    val numElements = readElementCount()

    if (numElements != null) {
        // Generar la lista de números aleatorios
        val list = MutableList(numElements) { Random.nextDouble() }

        // Medir el tiempo de ejecución para ordenar la lista
        val timeInMillis = measureTimeMillis { list.sort() }

        // Convertir el tiempo a segundos
        val timeInSeconds = timeInMillis / 1000.0

        // Imprimir el tiempo de ejecución
        println("Tiempo de ejecución para ordenar $numElements elementos: %.8f segundos".format(timeInSeconds))
    } else {
        println("No se pudo obtener el número de elementos desde el archivo.")
    }
}