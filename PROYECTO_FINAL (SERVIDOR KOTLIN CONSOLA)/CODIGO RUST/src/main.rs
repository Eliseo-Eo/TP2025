use rayon::prelude::*;
use std::process::Command;
use std::time::Instant;
use std::fs;
use rand::Rng;
use sys_info;
use sysinfo::System; 
use std::sync::{Arc, Mutex};
use std::fs::File;
use std::io::{self, BufRead};
use std::net::SocketAddr;

// --- NUEVAS IMPORTACIONES PARA EL SERVIDOR AXUM Y JSON ---
use serde::{Serialize, Deserialize};
use axum::{
    routing::{get, post},
    http::StatusCode,
    Json,
    Router,
};
use tokio;

// --- ESTRUCTURAS DE DATOS PARA LA COMUNICACIÓN CON KOTLIN ---

/// Estructura que define los resultados de una prueba de rendimiento
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResultadoBenchmark {
    pub lenguaje: String,
    pub tiempo_segundos: f64,
    pub memoria_inicial_bytes: u64,
    pub memoria_final_bytes: u64,
    pub cpu_inicial_porcentaje: f32,
    pub cpu_final_porcentaje: f32,
    pub exito: bool,
    pub mensaje_error: Option<String>,
}

/// Estructura que define la solicitud enviada desde Kotlin
#[derive(Debug, Deserialize)]
pub struct SolicitudBenchmark {
    pub lenguajes_a_ejecutar: Vec<String>,
}

// --- FUNCIONES DE UTILIDAD ---

fn measure_memory_usage() -> Result<u64, String> {
    match sys_info::mem_info() {
        Ok(mem_info) => Ok(mem_info.total - mem_info.free),
        Err(e) => Err(format!("Error obteniendo información de memoria: {}", e)),
    }
}

fn measure_cpu_usage(system: &Arc<Mutex<System>>) -> f32 {
    let mut system = system.lock().unwrap();
    system.refresh_cpu();
    system.global_cpu_info().cpu_usage() 
}


fn read_size_from_file(file_path: &str) -> Result<usize, io::Error> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    if let Some(Ok(line)) = reader.lines().next() {
        line.trim().parse::<usize>().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "No se pudo leer el tamaño del archivo"))
    }
}

// --- FUNCIÓN DE PRUEBA MODIFICADA ---

/// Ejecuta una prueba para un lenguaje y devuelve la estructura de resultados.
fn run_test(language: &str, system: Arc<Mutex<System>>) -> ResultadoBenchmark {
    let start = Instant::now();
    let initial_memory = measure_memory_usage().unwrap_or(0);
    let initial_cpu_usage = measure_cpu_usage(&system);

    // Eliminamos 'mut' aquí ya que la mutación ocurría incorrectamente en el scope local del arm "Rust"
    let (exito, tiempo, final_mem, final_cpu, mensaje_error) = match language {
        "Python" => {
            if !fs::metadata("./languages/Prueba.py").is_ok() {
                (false, 0.0, initial_memory, initial_cpu_usage, Some("El archivo Python no existe".to_string()))
            } else {
                match Command::new("python3").arg("./languages/Prueba.py").output() {
                    Ok(output) => {
                        let final_memory = measure_memory_usage().unwrap_or(0);
                        let final_cpu_usage = measure_cpu_usage(&system);
                        if output.status.success() {
                            (true, start.elapsed().as_secs_f64(), final_memory, final_cpu_usage, None)
                        } else {
                            (false, 0.0, final_memory, final_cpu_usage, Some(format!("Python terminó con código de salida: {}", output.status.code().unwrap_or(-1))))
                        }
                    },
                    Err(e) => (false, 0.0, initial_memory, initial_cpu_usage, Some(format!("Error ejecutando Python: {}", e))),
                }
            }
        },
        "Kotlin" => {
            let jar_exists = fs::metadata("./languages/Prueba.jar").is_ok();
            let kt_exists = fs::metadata("./languages/Prueba.kt").is_ok();

            if !jar_exists && !kt_exists {
                return ResultadoBenchmark {
                    lenguaje: language.to_string(),
                    tiempo_segundos: 0.0,
                    memoria_inicial_bytes: initial_memory,
                    memoria_final_bytes: initial_memory,
                    cpu_inicial_porcentaje: initial_cpu_usage,
                    cpu_final_porcentaje: initial_cpu_usage,
                    exito: false,
                    mensaje_error: Some("El archivo Kotlin (Prueba.kt o Prueba.jar) no existe".to_string()),
                };
            }

            if !jar_exists {
                match Command::new("kotlinc").arg("./languages/Prueba.kt").arg("-include-runtime").arg("-d").arg("./languages/Prueba.jar").output() {
                    Ok(compile_output) => {
                        if !compile_output.status.success() {
                            return ResultadoBenchmark {
                                lenguaje: language.to_string(),
                                tiempo_segundos: 0.0,
                                memoria_inicial_bytes: initial_memory,
                                memoria_final_bytes: initial_memory,
                                cpu_inicial_porcentaje: initial_cpu_usage,
                                cpu_final_porcentaje: initial_cpu_usage,
                                exito: false,
                                mensaje_error: Some(format!("Error compilando Kotlin: {}", compile_output.status.code().unwrap_or(-1))),
                            };
                        }
                    },
                    Err(e) => return ResultadoBenchmark {
                        lenguaje: language.to_string(),
                        tiempo_segundos: 0.0,
                        memoria_inicial_bytes: initial_memory,
                        memoria_final_bytes: initial_memory,
                        cpu_inicial_porcentaje: initial_cpu_usage,
                        cpu_final_porcentaje: initial_cpu_usage,
                        exito: false,
                        mensaje_error: Some(format!("Error compilando Kotlin: {}", e)),
                    },
                }
            }
            
            match Command::new("java").arg("-jar").arg("./languages/Prueba.jar").output() {
                Ok(output) => {
                    let final_memory = measure_memory_usage().unwrap_or(0);
                    let final_cpu_usage = measure_cpu_usage(&system);
                    if output.status.success() {
                        (true, start.elapsed().as_secs_f64(), final_memory, final_cpu_usage, None)
                    } else {
                        (false, 0.0, final_memory, final_cpu_usage, Some(format!("Kotlin terminó con código de salida: {}", output.status.code().unwrap_or(-1))))
                    }
                },
                Err(e) => (false, 0.0, initial_memory, initial_cpu_usage, Some(format!("Error ejecutando Kotlin: {}", e))),
            }
        },
        "Rust" => {
            let file_path = "./languages/Tiempo.txt";
            let size = match read_size_from_file(file_path) {
                Ok(s) => s,
                Err(e) => return ResultadoBenchmark {
                    lenguaje: language.to_string(),
                    tiempo_segundos: 0.0,
                    memoria_inicial_bytes: initial_memory,
                    memoria_final_bytes: initial_memory,
                    cpu_inicial_porcentaje: initial_cpu_usage,
                    cpu_final_porcentaje: initial_cpu_usage,
                    exito: false,
                    mensaje_error: Some(format!("Error leyendo el tamaño del archivo: {}", e)),
                },
            };

            let mut list: Vec<f64> = (0..size)
                .map(|_| rand::thread_rng().gen_range(0.0..1.0))
                .collect();

            let sort_start = Instant::now();
            list.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let sort_duration = sort_start.elapsed();

            let final_memory = measure_memory_usage().unwrap_or(0);
            let final_cpu_usage = measure_cpu_usage(&system);
            
            // CORRECCIÓN E0425: Calculamos el valor y lo guardamos en una variable local
            let tiempo_calculado = sort_duration.as_secs_f64();
            
            // Devolvemos la tupla usando la nueva variable calculada
            (true, tiempo_calculado, final_memory, final_cpu_usage, None)
        },
        "C++" => {
            let exec_exists = fs::metadata("./languages/Prueba_cpp").is_ok();
            let src_exists = fs::metadata("./languages/Prueba.cpp").is_ok();
            
            if !exec_exists && !src_exists {
                return ResultadoBenchmark {
                    lenguaje: language.to_string(),
                    tiempo_segundos: 0.0,
                    memoria_inicial_bytes: initial_memory,
                    memoria_final_bytes: initial_memory,
                    cpu_inicial_porcentaje: initial_cpu_usage,
                    cpu_final_porcentaje: initial_cpu_usage,
                    exito: false,
                    mensaje_error: Some("El archivo C++ (Prueba.cpp o Prueba_cpp) no existe".to_string()),
                };
            }
            
            if !exec_exists {
                match Command::new("g++").arg("./languages/Prueba.cpp").arg("-o").arg("./languages/Prueba_cpp").output() {
                    Ok(compile_output) => {
                        if !compile_output.status.success() {
                            return ResultadoBenchmark {
                                lenguaje: language.to_string(),
                                tiempo_segundos: 0.0,
                                memoria_inicial_bytes: initial_memory,
                                memoria_final_bytes: initial_memory,
                                cpu_inicial_porcentaje: initial_cpu_usage,
                                cpu_final_porcentaje: initial_cpu_usage,
                                exito: false,
                                mensaje_error: Some(format!("Error compilando C++: {}", compile_output.status.code().unwrap_or(-1))),
                            };
                        }
                    },
                    Err(e) => return ResultadoBenchmark {
                        lenguaje: language.to_string(),
                        tiempo_segundos: 0.0,
                        memoria_inicial_bytes: initial_memory,
                        memoria_final_bytes: initial_memory,
                        cpu_inicial_porcentaje: initial_cpu_usage,
                        cpu_final_porcentaje: initial_cpu_usage,
                        exito: false,
                        mensaje_error: Some(format!("Error compilando C++: {}", e)),
                    },
                }
            }

            match Command::new("./languages/Prueba_cpp").output() {
                Ok(output) => {
                    let final_memory = measure_memory_usage().unwrap_or(0);
                    let final_cpu_usage = measure_cpu_usage(&system);
                    if output.status.success() {
                        (true, start.elapsed().as_secs_f64(), final_memory, final_cpu_usage, None)
                    } else {
                        (false, 0.0, final_memory, final_cpu_usage, Some(format!("C++ terminó con código de salida: {}", output.status.code().unwrap_or(-1))))
                    }
                },
                Err(e) => (false, 0.0, initial_memory, initial_cpu_usage, Some(format!("Error ejecutando C++: {}", e))),
            }
        },
        "C" => {
            let exec_exists = fs::metadata("./languages/Prueba_c").is_ok();
            let src_exists = fs::metadata("./languages/Prueba.c").is_ok();
            
            if !exec_exists && !src_exists {
                return ResultadoBenchmark {
                    lenguaje: language.to_string(),
                    tiempo_segundos: 0.0,
                    memoria_inicial_bytes: initial_memory,
                    memoria_final_bytes: initial_memory,
                    cpu_inicial_porcentaje: initial_cpu_usage,
                    cpu_final_porcentaje: initial_cpu_usage,
                    exito: false,
                    mensaje_error: Some("El archivo C (Prueba.c o Prueba_c) no existe".to_string()),
                };
            }

            if !exec_exists {
                match Command::new("gcc").arg("./languages/Prueba.c").arg("-o").arg("./languages/Prueba_c").output() {
                    Ok(compile_output) => {
                        if !compile_output.status.success() {
                            return ResultadoBenchmark {
                                lenguaje: language.to_string(),
                                tiempo_segundos: 0.0,
                                memoria_inicial_bytes: initial_memory,
                                memoria_final_bytes: initial_memory,
                                cpu_inicial_porcentaje: initial_cpu_usage,
                                cpu_final_porcentaje: initial_cpu_usage,
                                exito: false,
                                mensaje_error: Some(format!("Error compilando C: {}", compile_output.status.code().unwrap_or(-1))),
                            };
                        }
                    },
                    Err(e) => return ResultadoBenchmark {
                        lenguaje: language.to_string(),
                        tiempo_segundos: 0.0,
                        memoria_inicial_bytes: initial_memory,
                        memoria_final_bytes: initial_memory,
                        cpu_inicial_porcentaje: initial_cpu_usage,
                        cpu_final_porcentaje: initial_cpu_usage,
                        exito: false,
                        mensaje_error: Some(format!("Error compilando C: {}", e)),
                    },
                }
            }
            
            match Command::new("./languages/Prueba_c").output() {
                Ok(output) => {
                    let final_memory = measure_memory_usage().unwrap_or(0);
                    let final_cpu_usage = measure_cpu_usage(&system);
                    if output.status.success() {
                        (true, start.elapsed().as_secs_f64(), final_memory, final_cpu_usage, None)
                    } else {
                        (false, 0.0, final_memory, final_cpu_usage, Some(format!("C terminó con código de salida: {}", output.status.code().unwrap_or(-1))))
                    }
                },
                Err(e) => (false, 0.0, initial_memory, initial_cpu_usage, Some(format!("Error ejecutando C: {}", e))),
            }
        },
        _ => (false, 0.0, initial_memory, initial_cpu_usage, Some("Lenguaje no soportado".to_string())),
    };

    // Devolvemos la estructura ResultadoBenchmark
    ResultadoBenchmark {
        lenguaje: language.to_string(),
        tiempo_segundos: tiempo,
        memoria_inicial_bytes: initial_memory,
        memoria_final_bytes: final_mem,
        cpu_inicial_porcentaje: initial_cpu_usage,
        cpu_final_porcentaje: final_cpu,
        exito,
        mensaje_error,
    }
}

// --- HANDLER DE LA API ---

async fn run_benchmark_handler(
    Json(payload): Json<SolicitudBenchmark>,
) -> (StatusCode, Json<Vec<ResultadoBenchmark>>) {
    println!("Recibida solicitud para ejecutar benchmarks en: {:?}", payload.lenguajes_a_ejecutar);

    let system = Arc::new(Mutex::new(System::new())); 
    let languages = payload.lenguajes_a_ejecutar;

    // Ejecutamos las pruebas de forma concurrente usando Rayon.
    let results: Vec<ResultadoBenchmark> = languages.par_iter().map(|lang| {
        run_test(lang, Arc::clone(&system))
    }).collect();

    println!("Benchmarks completados. Enviando {} resultados.", results.len());

    (StatusCode::OK, Json(results))
}

// --- FUNCIÓN PRINCIPAL DEL SERVIDOR AXUM ---

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/benchmark", post(run_benchmark_handler))
        .route("/health", get(|| async { "Rust Benchmark Backend is running!" }));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    println!("\n=============================================");
    println!("🚀 Rust Benchmark API escuchando en: http://{}", addr);
    println!("POST /benchmark para iniciar las pruebas.");
    println!("=============================================\n");

    let listener = match tokio::net::TcpListener::bind(addr).await {
        Ok(l) => l,
        Err(e) => {
            eprintln!("❌ Error al enlazar el puerto {}: {}. Asegúrate de que no haya otra aplicación ejecutándose en este puerto.", addr.port(), e);
            return;
        }
    };

    axum::serve(listener, app).await.unwrap();
}