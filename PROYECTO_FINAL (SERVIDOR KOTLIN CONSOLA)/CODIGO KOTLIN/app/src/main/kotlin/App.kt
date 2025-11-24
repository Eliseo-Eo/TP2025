import kotlinx.serialization.Serializable
import io.ktor.client.*
import io.ktor.client.engine.cio.*
import io.ktor.client.plugins.contentnegotiation.*
import io.ktor.client.request.*
import io.ktor.client.call.*
import io.ktor.http.*
import io.ktor.serialization.kotlinx.json.*
import kotlinx.serialization.json.Json
import kotlinx.coroutines.runBlocking
import kotlinx.coroutines.launch
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.delay

// ==========================================================
// 1. ESTRUCTURAS DE DATOS
// ==========================================================

@Serializable
data class SolicitudBenchmark(
    val lenguajes_a_ejecutar: List<String>
)

@Serializable
data class ResultadoBenchmark(
    val lenguaje: String,
    val tiempo_segundos: Double,
    val memoria_inicial_bytes: Long,
    val memoria_final_bytes: Long,
    val cpu_inicial_porcentaje: Float,
    val cpu_final_porcentaje: Float,
    val exito: Boolean,
    val mensaje_error: String?
)

// ==========================================================
// 2. LÓGICA DE CONEXIÓN
// ==========================================================

// Cliente HTTP configurado para manejar JSON
private val httpClient = HttpClient(CIO) {
    install(ContentNegotiation) {
        json(Json {
            ignoreUnknownKeys = true
            isLenient = true
        })
    }
    // Timeout largo para dar tiempo a Rust de ejecutar las pruebas
    engine {
        requestTimeout = 300_000 // 5 minutos
    }
}

class BenchmarkService {
    suspend fun getBenchmarkResults(lenguajes: List<String>): List<ResultadoBenchmark> {
        val solicitud = SolicitudBenchmark(lenguajes_a_ejecutar = lenguajes)

        return try {
            val response: List<ResultadoBenchmark> = httpClient.post("http://127.0.0.1:8080/benchmark") {
                contentType(ContentType.Application.Json)
                setBody(solicitud)
            }.body()
            response

        } catch (e: Exception) {
            println("Error al obtener resultados del benchmark: ${e.message}")
            emptyList()
        }
    }
}

// ==========================================================
// 3. FUNCIÓN PRINCIPAL (Punto de Ejecución)
// ==========================================================

fun main() = runBlocking {

    // ⚠️ ¡Paso crucial!: Asegúrate de que tu servidor Rust esté corriendo en OTRA TERMINAL.

    println("--- Cliente Kotlin Benchmark Iniciado ---")

    val service = BenchmarkService()
    val lenguajesAComparar = listOf("Rust", "C++", "Python", "Kotlin", "C")

    // Lanza la operación de red en un contexto de Coroutine
    launch(Dispatchers.Default) {
        println("Enviando solicitud para comparar: $lenguajesAComparar...")

        val resultados = service.getBenchmarkResults(lenguajesAComparar)

        if (resultados.isNotEmpty()) {
            println("\n--- ✅ Resultados Recibidos del Servidor Rust ---")

            // Mostrar los datos en la terminal, ordenados por tiempo
            resultados.sortedBy { it.tiempo_segundos }.forEach { r ->
                val tiempo = "%.4f".format(r.tiempo_segundos)
                val memoria = (r.memoria_final_bytes - r.memoria_inicial_bytes) / 1024.0 / 1024.0

                println("-----------------------------------------")
                println(" 🥇 Lenguaje: ${r.lenguaje}")
                println(" ⏱️ Tiempo: ${tiempo}s")
                println(" 🧠 Memoria (Delta): ${"%.2f".format(memoria)} MB")
                println(" 📊 CPU Final: ${"%.2f".format(r.cpu_final_porcentaje)} %")
                if (!r.exito) {
                    println(" ❌ ERROR: ${r.mensaje_error}")
                }
            }
            println("-----------------------------------------")
        } else {
            println("\n--- ❌ No se pudo conectar con el servidor o no hay resultados ---")
            println("Asegúrate de que el backend de Rust esté corriendo en http://127.0.0.1:8080")
        }
    }

    // Esperar un poco para asegurar que la coroutine termine
    delay(50000)
}