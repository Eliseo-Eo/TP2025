// Se importan las librerías necesarias para el programa:
// `rayon` permite paralelización de tareas.
// `std::process::{Command}` es para ejecutar comandos del sistema.
// `std::time::Instant` se usa para medir el tiempo de ejecución.
// `std::fs` para manipulación de archivos.
// `rand::Rng` para generar números aleatorios, necesario para la prueba en Rust.
// `sys_info` y `sysinfo` se usan para obtener información del sistema, como memoria y uso de CPU.
// `std::sync::{Arc, Mutex}` se usa para compartir el estado del sistema entre hilos de manera segura.
use rayon::prelude::*;  
use std::process::{Command};
use std::time::Instant;
use std::fs;
use rand::Rng; 
use sys_info;  
use sysinfo::{System, SystemExt, ProcessorExt}; 
use std::sync::{Arc, Mutex};  
use std::fs::File;
use std::io::{self, BufRead};

// Función que mide el uso de memoria del sistema en bytes.
// Calcula la memoria usada como la diferencia entre la memoria total y la memoria libre.
fn measure_memory_usage() -> Result<u64, String> {
    match sys_info::mem_info() {
        Ok(mem_info) => Ok(mem_info.total - mem_info.free),  // Memoria usada = total - libre
        Err(e) => Err(format!("Error obteniendo información de memoria: {}", e)),
    }
}

// Función que mide el uso de CPU en porcentaje.
// Actualiza el estado del sistema y retorna el porcentaje de uso de la CPU.
fn measure_cpu_usage(system: &Arc<Mutex<System>>) -> f32 {
    let mut system = system.lock().unwrap();
    system.refresh_cpu();  // Refresca la información de la CPU
    system.global_processor_info().cpu_usage()  // Devuelve el porcentaje de uso de la CPU
}

// Función para formatear un número de bytes con comas como separadores de miles.
fn format_with_commas(number: u64) -> String {
    let number_str = number.to_string();
    let mut result = String::new();
    let mut counter = 0;

    // Recorre el número de atrás hacia adelante agregando comas cada 3 dígitos
    for c in number_str.chars().rev() {
        if counter > 0 && counter % 3 == 0 {
            result.push(',');
        }
        result.push(c);
        counter += 1;
    }

    result.chars().rev().collect()  // Invierte el resultado para obtener el formato correcto
}

// Función para leer el tamaño de un archivo llamado "Tiempo.txt".
// Este archivo contiene el tamaño de los datos que se procesarán en las pruebas.
fn read_size_from_file(file_path: &str) -> Result<usize, io::Error> {
    let file = File::open(file_path)?;  // Intenta abrir el archivo
    let reader = io::BufReader::new(file);  // Crea un lector de búfer para leer el archivo
    if let Some(Ok(line)) = reader.lines().next() {
        line.trim().parse::<usize>().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "No se pudo leer el tamaño del archivo"))
    }
}

// Función principal que ejecuta las pruebas de rendimiento para diferentes lenguajes de programación.
// Mide el tiempo de ejecución, el uso de memoria y el uso de CPU, e imprime los resultados.
fn run_test(language: &str, system: Arc<Mutex<System>>) -> Result<f64, String> {
    let start = Instant::now();  // Inicia el temporizador antes de ejecutar el proceso

    // Medir el uso de memoria y CPU al inicio
    let initial_memory = measure_memory_usage().unwrap_or(0);
    let initial_cpu_usage = measure_cpu_usage(&system);

    let output = match language {
        // Prueba para Python
        "Python" => {
            if !fs::metadata("./languages/Prueba.py").is_ok() {
                return Err("El archivo Python no existe".to_string());
            }

            let result = Command::new("python3")
                .arg("./languages/Prueba.py")
                .output()
                .map_err(|e| format!("Error ejecutando Python: {}", e));

            match result {
                Ok(output) => {
                    if output.status.success() {
                        let duration = start.elapsed().as_secs_f64();
                        let final_memory = measure_memory_usage().unwrap_or(0);
                        let final_cpu_usage = measure_cpu_usage(&system);
                        println!(
                            "Memoria inicial: {} bytes, Memoria final: {} bytes, CPU inicial: {:.2}%, CPU final: {:.2}%, Tiempo: {:.4} segundos. Lenguaje: {}",
                            format_with_commas(initial_memory), format_with_commas(final_memory), initial_cpu_usage, final_cpu_usage, duration, language
                        );
                        Ok(duration)
                    } else {
                        Err(format!("Python terminó con código de salida: {}", output.status.code().unwrap_or(-1)))
                    }
                },
                Err(e) => Err(e),
            }
        },
        // Prueba para Kotlin
        "Kotlin" => {
            if !fs::metadata("./languages/Prueba.jar").is_ok() {
                if !fs::metadata("./languages/Prueba.kt").is_ok() {
                    return Err("El archivo Kotlin (Prueba.kt) no existe".to_string());
                }

                let compile_result = Command::new("kotlinc")
                    .arg("./languages/Prueba.kt")
                    .arg("-include-runtime")
                    .arg("-d")
                    .arg("./languages/Prueba.jar")
                    .output()
                    .map_err(|e| format!("Error compilando Kotlin: {}", e));

                match compile_result {
                    Ok(compile_output) => {
                        if compile_output.status.success() {
                            let result = Command::new("java")
                                .arg("-jar")
                                .arg("./languages/Prueba.jar")
                                .output()
                                .map_err(|e| format!("Error ejecutando Kotlin con java: {}", e));

                            match result {
                                Ok(output) => {
                                    if output.status.success() {
                                        let duration = start.elapsed().as_secs_f64();
                                        let final_memory = measure_memory_usage().unwrap_or(0);
                                        let final_cpu_usage = measure_cpu_usage(&system);
                                        println!(
                                            "Memoria inicial: {} bytes, Memoria final: {} bytes, CPU inicial: {:.2}%, CPU final: {:.2}%, Tiempo: {:.4} segundos. Lenguaje: {}",
                                            format_with_commas(initial_memory), format_with_commas(final_memory), initial_cpu_usage, final_cpu_usage, duration, language
                                        );
                                        Ok(duration)
                                    } else {
                                        Err(format!("Kotlin terminó con código de salida: {}", output.status.code().unwrap_or(-1)))
                                    }
                                },
                                Err(e) => Err(e),
                            }
                        } else {
                            Err(format!("Error compilando Kotlin: {}", compile_output.status.code().unwrap_or(-1)))
                        }
                    },
                    Err(e) => Err(e),
                }
            } else {
                let result = Command::new("java")
                    .arg("-jar")
                    .arg("./languages/Prueba.jar")
                    .output()
                    .map_err(|e| format!("Error ejecutando Kotlin con java: {}", e));

                match result {
                    Ok(output) => {
                        if output.status.success() {
                            let duration = start.elapsed().as_secs_f64();
                            let final_memory = measure_memory_usage().unwrap_or(0);
                            let final_cpu_usage = measure_cpu_usage(&system);
                            println!(
                                "Memoria inicial: {} bytes, Memoria final: {} bytes, CPU inicial: {:.2}%, CPU final: {:.2}%, Tiempo: {:.4} segundos. Lenguaje: {}",
                                format_with_commas(initial_memory), format_with_commas(final_memory), initial_cpu_usage, final_cpu_usage, duration, language
                            );
                            Ok(duration)
                        } else {
                            Err(format!("Kotlin terminó con código de salida: {}", output.status.code().unwrap_or(-1)))
                        }
                    },
                    Err(e) => Err(e),
                }
            }
        },
        // Prueba para Rust
        "Rust" => {
            let file_path = "./languages/Tiempo.txt";  // Ruta del archivo con el tamaño de los datos
            let size = match read_size_from_file(file_path) {
                Ok(size) => size,   // Si la lectura es exitosa, obtenemos el tamaño
                Err(e) => return Err(format!("Error leyendo el tamaño del archivo: {}", e)),  // En caso de error, lo manejamos
            };

            // Genera una lista de números aleatorios y la ordena
            let mut list: Vec<f64> = (0..size)
                .map(|_| rand::thread_rng().gen_range(0.0..1.0)) 
                .collect();

            // Medir el tiempo de ordenación
            let sort_start = Instant::now();
            list.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let sort_duration = sort_start.elapsed();

            let final_memory = measure_memory_usage().unwrap_or(0);
            let final_cpu_usage = measure_cpu_usage(&system);

            // Imprimir los resultados de la prueba
            println!(
                "Memoria inicial: {} bytes, Memoria final: {} bytes, CPU inicial: {:.2}%, CPU final: {:.2}%, Tiempo: {:.4} segundos. Lenguaje: {}",
                format_with_commas(initial_memory), format_with_commas(final_memory), initial_cpu_usage, final_cpu_usage, sort_duration.as_secs_f64(), language
            );

            Ok(sort_duration.as_secs_f64())  // Devolvemos el tiempo de ordenación
        },
        // Prueba para C++
        "C++" => {
            if !fs::metadata("./languages/Prueba_cpp").is_ok() {
                if !fs::metadata("./languages/Prueba.cpp").is_ok() {
                    return Err("El archivo C++ (Prueba.cpp) no existe".to_string());
                }

                let compile_result = Command::new("g++")
                    .arg("./languages/Prueba.cpp")
                    .arg("-o")
                    .arg("./languages/Prueba_cpp")
                    .output()
                    .map_err(|e| format!("Error compilando C++: {}", e));

                match compile_result {
                    Ok(compile_output) => {
                        if compile_output.status.success() {
                            let result = Command::new("./languages/Prueba_cpp")
                                .output()
                                .map_err(|e| format!("Error ejecutando C++: {}", e));

                            match result {
                                Ok(output) => {
                                    if output.status.success() {
                                        let duration = start.elapsed().as_secs_f64();
                                        let final_memory = measure_memory_usage().unwrap_or(0);
                                        let final_cpu_usage = measure_cpu_usage(&system);
                                        println!(
                                            "Memoria inicial: {} bytes, Memoria final: {} bytes, CPU inicial: {:.2}%, CPU final: {:.2}%, Tiempo: {:.4} segundos. Lenguaje: {}",
                                            format_with_commas(initial_memory), format_with_commas(final_memory), initial_cpu_usage, final_cpu_usage, duration, language
                                        );
                                        Ok(duration)
                                    } else {
                                        Err(format!("C++ terminó con código de salida: {}", output.status.code().unwrap_or(-1)))
                                    }
                                },
                                Err(e) => Err(e),
                            }
                        } else {
                            Err(format!("Error compilando C++: {}", compile_output.status.code().unwrap_or(-1)))
                        }
                    },
                    Err(e) => Err(e),
                }
            } else {
                let result = Command::new("./languages/Prueba_cpp")
                    .output()
                    .map_err(|e| format!("Error ejecutando C++: {}", e));

                match result {
                    Ok(output) => {
                        if output.status.success() {
                            let duration = start.elapsed().as_secs_f64();
                            let final_memory = measure_memory_usage().unwrap_or(0);
                            let final_cpu_usage = measure_cpu_usage(&system);
                            println!(
                                "Memoria inicial: {} bytes, Memoria final: {} bytes, CPU inicial: {:.2}%, CPU final: {:.2}%, Tiempo: {:.4} segundos. Lenguaje: {}",
                                format_with_commas(initial_memory), format_with_commas(final_memory), initial_cpu_usage, final_cpu_usage, duration, language
                            );
                            Ok(duration)
                        } else {
                            Err(format!("C++ terminó con código de salida: {}", output.status.code().unwrap_or(-1)))
                        }
                    },
                    Err(e) => Err(e),
                }
            }
        },
        // Prueba para C
        "C" => {
            if !fs::metadata("./languages/Prueba_c").is_ok() {
                if !fs::metadata("./languages/Prueba.c").is_ok() {
                    return Err("El archivo C (Prueba.c) no existe".to_string());
                }

                let compile_result = Command::new("gcc")
                    .arg("./languages/Prueba.c")
                    .arg("-o")
                    .arg("./languages/Prueba_c")
                    .output()
                    .map_err(|e| format!("Error compilando C: {}", e));

                match compile_result {
                    Ok(compile_output) => {
                        if compile_output.status.success() {
                            let result = Command::new("./languages/Prueba_c")
                                .output()
                                .map_err(|e| format!("Error ejecutando C: {}", e));

                            match result {
                                Ok(output) => {
                                    if output.status.success() {
                                        let duration = start.elapsed().as_secs_f64();
                                        let final_memory = measure_memory_usage().unwrap_or(0);
                                        let final_cpu_usage = measure_cpu_usage(&system);
                                        println!(
                                            "Memoria inicial: {} bytes, Memoria final: {} bytes, CPU inicial: {:.2}%, CPU final: {:.2}%, Tiempo: {:.4} segundos. Lenguaje: {}",
                                            format_with_commas(initial_memory), format_with_commas(final_memory), initial_cpu_usage, final_cpu_usage, duration, language
                                        );
                                        Ok(duration)
                                    } else {
                                        Err(format!("C terminó con código de salida: {}", output.status.code().unwrap_or(-1)))
                                    }
                                },
                                Err(e) => Err(e),
                            }
                        } else {
                            Err(format!("Error compilando C: {}", compile_output.status.code().unwrap_or(-1)))
                        }
                    },
                    Err(e) => Err(e),
                }
            } else {
                let result = Command::new("./languages/Prueba_c")
                    .output()
                    .map_err(|e| format!("Error ejecutando C: {}", e));

                match result {
                    Ok(output) => {
                        if output.status.success() {
                            let duration = start.elapsed().as_secs_f64();
                            let final_memory = measure_memory_usage().unwrap_or(0);
                            let final_cpu_usage = measure_cpu_usage(&system);
                            println!(
                                "Memoria inicial: {} bytes, Memoria final: {} bytes, CPU inicial: {:.2}%, CPU final: {:.2}%, Tiempo: {:.4} segundos. Lenguaje: {}",
                                format_with_commas(initial_memory), format_with_commas(final_memory), initial_cpu_usage, final_cpu_usage, duration, language
                            );
                            Ok(duration)
                        } else {
                            Err(format!("C terminó con código de salida: {}", output.status.code().unwrap_or(-1)))
                        }
                    },
                    Err(e) => Err(e),
                }
            }
        },
        // Si el lenguaje no está soportado, devuelve un error
        _ => return Err("Lenguaje no soportado".to_string()),
    };

    output
}

// Función principal que ejecuta pruebas en varios lenguajes de programación de forma paralela.
// Utiliza `rayon` para paralelizar la ejecución de las pruebas.
fn main() {
    let languages = vec!["Python", "Kotlin", "Rust", "C++", "C"];
    
    // Compartimos el estado del sistema entre hilos usando un Mutex
    let system = Arc::new(Mutex::new(System::new_all()));

    // Ejecutamos las pruebas de forma concurrente usando rayon
    languages.par_iter().for_each(|lang| {
        match run_test(lang, Arc::clone(&system)) {
            Ok(_) => {},  // Si la prueba es exitosa, no hacemos nada
            Err(e) => {
                println!("Error en {}: {}", lang, e);  // Si ocurre un error, lo mostramos
            },
        }
    });
}