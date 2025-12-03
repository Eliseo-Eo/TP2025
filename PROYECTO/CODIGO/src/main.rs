// ===============================================================
// Concurrent Benchmarking Tool - Multi-Language Execution
// ===============================================================
//
// Descripción:
//  Este programa permite ejecutar pruebas de rendimiento para varios
//  lenguajes de programación (Python, Kotlin, Rust, C, C++).
//  Mide tiempos de ejecución, consumo de memoria, uso de CPU y 
//  cantidad de elementos procesados.
//
// Características principales:
// - Concurrencia y paralelismo usando Arc, Mutex y Rayon.
// - Registro histórico de ejecuciones en CSV dentro de ./languages.
// - Soporte para ejecutar archivos ya compilados o compilar sobre la marcha.
// - Reportes detallados con formato de miles y fecha/hora.
//
// ===============================================================

// ---------------------------------------------------------------
// Librerías principales necesarias
// ---------------------------------------------------------------

// Rayon para paralelismo
use rayon::prelude::*;

// Para ejecutar comandos externos como python, java, gcc, g++
use std::process::Command;

// Para medir tiempos de ejecución
use std::time::Instant;

// Para manejo de archivos
use std::fs;
use std::fs::File;
use std::io::{self, BufRead, Write};

// Para generar números aleatorios en Rust
use rand::Rng;

// Para medir memoria del sistema
use sys_info;

// Para información detallada de CPU
use sysinfo::{System, SystemExt, ProcessorExt};

// Para compartir datos entre hilos de manera segura
use std::sync::{Arc, Mutex};

// Para convertir usize a u64
use std::convert::TryInto;

// Para fecha y hora
use chrono::Local;

// Para manejo de rutas de archivos
use std::path::Path;

// ===============================================================
// Funciones auxiliares
// ===============================================================

// ---------------------------------------------------------------
// Función: measure_memory_usage
// Descripción:
//  Obtiene el uso actual de memoria del sistema en bytes.
// ---------------------------------------------------------------
fn measure_memory_usage() -> Result<u64, String> {
    match sys_info::mem_info() {
        Ok(mem_info) => Ok(mem_info.total - mem_info.free),
        Err(e) => Err(format!("Error obteniendo información de memoria: {}", e)),
    }
}

// ---------------------------------------------------------------
// Función: measure_cpu_usage
// Descripción:
//  Obtiene el uso actual de CPU en porcentaje.
// Parámetros:
//  system: referencia a System protegido con Arc<Mutex>
// ---------------------------------------------------------------
fn measure_cpu_usage(system: &Arc<Mutex<System>>) -> f32 {
    let mut system = system.lock().unwrap();
    system.refresh_cpu();
    system.global_processor_info().cpu_usage()
}

// ---------------------------------------------------------------
// Función: format_with_commas
// Descripción:
//  Formatea un número agregando separadores de miles.
// ---------------------------------------------------------------
fn format_with_commas(number: u64) -> String {
    let number_str = number.to_string();
    let mut result = String::new();
    let mut counter = 0;

    for c in number_str.chars().rev() {
        if counter > 0 && counter % 3 == 0 {
            result.push(',');
        }
        result.push(c);
        counter += 1;
    }

    result.chars().rev().collect()
}

// ---------------------------------------------------------------
// Función: read_size_from_file
// Descripción:
//  Lee la cantidad de elementos a procesar desde un archivo de texto.
// ---------------------------------------------------------------
fn read_size_from_file(file_path: &str) -> Result<usize, io::Error> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    if let Some(Ok(line)) = reader.lines().next() {
        line.trim()
            .parse::<usize>()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "No se pudo leer el tamaño del archivo",
        ))
    }
}

// ---------------------------------------------------------------
// Función: log_execution_header
// Descripción:
//  Imprime un encabezado de ejecución con fecha y hora.
// ---------------------------------------------------------------
fn log_execution_header() {
    let now = Local::now();
    println!("===============================================================");
    println!("Ejecución de pruebas - Fecha y hora: {}", now.format("%Y-%m-%d %H:%M:%S"));
    println!("===============================================================");
}

// ---------------------------------------------------------------
// Función: log_separator
// Descripción:
//  Imprime un separador visual en consola.
// ---------------------------------------------------------------
fn log_separator() {
    println!("===============================================================");
}

// ===============================================================
// Función principal de prueba para cada lenguaje
// ===============================================================
fn run_test(
    language: &str,
    system: Arc<Mutex<System>>,
    size: usize,
) -> Result<(String, f64, u64, u64, f32, f32, usize, String), String> {
    // Marca el tiempo inicial
    let start = Instant::now();

    // Memoria y CPU inicial
    let initial_memory = measure_memory_usage().unwrap_or(0);
    let initial_cpu_usage = measure_cpu_usage(&system);

    // Variable para almacenar salida de cada lenguaje
    let output = match language {

        "Python" => {
            if !fs::metadata("./languages/Prueba.py").is_ok() {
                return Err("El archivo Python no existe".to_string());
            }

            let result = Command::new("python3")
                .arg("./languages/Prueba.py")
                .output()
                .map_err(|e| format!("Error ejecutando Python: {}", e))?;

            if !result.status.success() {
                return Err(format!(
                    "Python terminó con código de salida: {}",
                    result.status.code().unwrap_or(-1)
                ));
            }

            let duration = start.elapsed().as_secs_f64();
            let final_memory = measure_memory_usage().unwrap_or(0);
            let final_cpu_usage = measure_cpu_usage(&system);

            println!(
                "Elementos: {}, Memoria Inicial: {} | Memoria Final: {} | CPU Inicial: {:.2}% | CPU Final: {:.2}% | Tiempo: {:.4} s. Lenguaje: {}",
                format_with_commas(size.try_into().unwrap()),
                format_with_commas(initial_memory),
                format_with_commas(final_memory),
                initial_cpu_usage,
                final_cpu_usage,
                duration,
                language
            );

            Ok((
                language.to_string(),
                duration,
                initial_memory,
                final_memory,
                initial_cpu_usage,
                final_cpu_usage,
                size,
                Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
            ))
        },

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
                    .map_err(|e| format!("Error compilando Kotlin: {}", e))?;

                if !compile_result.status.success() {
                    return Err(format!(
                        "Error compilando Kotlin: {}",
                        compile_result.status.code().unwrap_or(-1)
                    ));
                }
            }

            let run_result = Command::new("java")
                .arg("-jar")
                .arg("./languages/Prueba.jar")
                .output()
                .map_err(|e| format!("Error ejecutando Kotlin con java: {}", e))?;

            if !run_result.status.success() {
                return Err(format!(
                    "Kotlin terminó con código de salida: {}",
                    run_result.status.code().unwrap_or(-1)
                ));
            }

            let duration = start.elapsed().as_secs_f64();
            let final_memory = measure_memory_usage().unwrap_or(0);
            let final_cpu_usage = measure_cpu_usage(&system);

            println!(
                "Elementos: {}, Memoria Inicial: {} | Memoria Final: {} | CPU Inicial: {:.2}% | CPU Final: {:.2}% | Tiempo: {:.4} s. Lenguaje: {}",
                format_with_commas(size.try_into().unwrap()),
                format_with_commas(initial_memory),
                format_with_commas(final_memory),
                initial_cpu_usage,
                final_cpu_usage,
                duration,
                language
            );

            Ok((
                language.to_string(),
                duration,
                initial_memory,
                final_memory,
                initial_cpu_usage,
                final_cpu_usage,
                size,
                Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
            ))
        },

        "Rust" => {
            let mut list: Vec<f64> = (0..size)
                .map(|_| rand::thread_rng().gen_range(0.0..1.0))
                .collect();

            let sort_start = Instant::now();
            // Paralelizar sort usando Rayon
            use rayon::slice::ParallelSliceMut;
            list.par_sort_by(|a, b| a.partial_cmp(b).unwrap());
            let sort_duration = sort_start.elapsed();

            let final_memory = measure_memory_usage().unwrap_or(0);
            let final_cpu_usage = measure_cpu_usage(&system);

            println!(
                "Elementos: {}, Memoria Inicial: {} | Memoria Final: {} | CPU Inicial: {:.2}% | CPU Final: {:.2}% | Tiempo: {:.4} s. Lenguaje: {}",
                format_with_commas(size.try_into().unwrap()),
                format_with_commas(initial_memory),
                format_with_commas(final_memory),
                initial_cpu_usage,
                final_cpu_usage,
                sort_duration.as_secs_f64(),
                language
            );

            Ok((
                language.to_string(),
                sort_duration.as_secs_f64(),
                initial_memory,
                final_memory,
                initial_cpu_usage,
                final_cpu_usage,
                size,
                Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
            ))
        },

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
                    .map_err(|e| format!("Error compilando C++: {}", e))?;

                if !compile_result.status.success() {
                    return Err(format!(
                        "Error compilando C++: {}",
                        compile_result.status.code().unwrap_or(-1)
                    ));
                }
            }

            let run_result = Command::new("./languages/Prueba_cpp")
                .output()
                .map_err(|e| format!("Error ejecutando C++: {}", e))?;

            if !run_result.status.success() {
                return Err(format!(
                    "C++ terminó con código de salida: {}",
                    run_result.status.code().unwrap_or(-1)
                ));
            }

            let duration = start.elapsed().as_secs_f64();
            let final_memory = measure_memory_usage().unwrap_or(0);
            let final_cpu_usage = measure_cpu_usage(&system);

            println!(
                "Elementos: {}, Memoria Inicial: {} | Memoria Final: {} | CPU Inicial: {:.2}% | CPU Final: {:.2}% | Tiempo: {:.4} s. Lenguaje: {}",
                format_with_commas(size.try_into().unwrap()),
                format_with_commas(initial_memory),
                format_with_commas(final_memory),
                initial_cpu_usage,
                final_cpu_usage,
                duration,
                language
            );

            Ok((
                language.to_string(),
                duration,
                initial_memory,
                final_memory,
                initial_cpu_usage,
                final_cpu_usage,
                size,
                Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
            ))
        },

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
                    .map_err(|e| format!("Error compilando C: {}", e))?;

                if !compile_result.status.success() {
                    return Err(format!(
                        "Error compilando C: {}",
                        compile_result.status.code().unwrap_or(-1)
                    ));
                }
            }

            let run_result = Command::new("./languages/Prueba_c")
                .output()
                .map_err(|e| format!("Error ejecutando C: {}", e))?;

            if !run_result.status.success() {
                return Err(format!(
                    "C terminó con código de salida: {}",
                    run_result.status.code().unwrap_or(-1)
                ));
            }

            let duration = start.elapsed().as_secs_f64();
            let final_memory = measure_memory_usage().unwrap_or(0);
            let final_cpu_usage = measure_cpu_usage(&system);

            println!(
                "Elementos: {}, Memoria Inicial: {} | Memoria Final: {} | CPU Inicial: {:.2}% | CPU Final: {:.2}% | Tiempo: {:.4} s. Lenguaje: {}",
                format_with_commas(size.try_into().unwrap()),
                format_with_commas(initial_memory),
                format_with_commas(final_memory),
                initial_cpu_usage,
                final_cpu_usage,
                duration,
                language
            );

            Ok((
                language.to_string(),
                duration,
                initial_memory,
                final_memory,
                initial_cpu_usage,
                final_cpu_usage,
                size,
                Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
            ))
        },

        _ => Err("Lenguaje no soportado".to_string()),
    };

    output
}

// ===============================================================
// Función: save_to_csv
// Descripción:
//  Guarda los resultados en CSV dentro de ./languages
// ===============================================================
fn save_to_csv(results: &Vec<(String, f64, u64, u64, f32, f32, usize, String)>) {
    let dir_path = "./languages";
    let file_path = format!("{}/Datos_Almacenados.csv", dir_path);

    if !Path::new(dir_path).exists() {
        fs::create_dir_all(dir_path).expect("No se pudo crear la carpeta languages");
    }

    use std::fs::OpenOptions;

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&file_path)
        .expect("No se pudo abrir o crear el archivo CSV");

    writeln!(file, "===============================================================").unwrap();
    writeln!(file, "Fecha/Hora de ejecución: {}", Local::now().format("%Y-%m-%d %H:%M:%S")).unwrap();
    writeln!(
        file,
        "Lenguaje,Tiempo(s),Memoria Inicial,Memoria Final,CPU Inicial,CPU Final,Cantidad de Elementos,Fecha/Hora Prueba"
    )
    .unwrap();

    for (lang, time, mem_i, mem_f, cpu_i, cpu_f, count, timestamp) in results.iter() {
        writeln!(
            file,
            "{},{:.4},{},{},{:.2},{:.2},{},{}",
            lang,
            time,
            mem_i,
            mem_f,
            cpu_i,
            cpu_f,
            format_with_commas((*count).try_into().unwrap()),
            timestamp
        )
        .unwrap();
    }

    writeln!(file, "===============================================================").unwrap();
}

// ===============================================================
// Función principal
// ===============================================================
fn main() {
    log_execution_header();

    let languages = vec!["C", "C++", "Python", "Kotlin", "Rust"];
    let system = Arc::new(Mutex::new(System::new_all()));

    let size = match read_size_from_file("./languages/Elementos.txt") {
        Ok(s) => s,
        Err(e) => {
            println!("Error leyendo cantidad de elementos: {}", e);
            return;
        }
    };

    // Vector seguro compartido entre hilos para resultados
    let results: Arc<Mutex<Vec<(String, f64, u64, u64, f32, f32, usize, String)>>> =
        Arc::new(Mutex::new(Vec::new()));

    // ---------------------------------------------------------------
    // Paralelismo usando Rayon
    // ---------------------------------------------------------------
    languages.par_iter().for_each(|lang| {
        match run_test(lang, Arc::clone(&system), size) {
            Ok(result) => {
                let mut results = results.lock().unwrap();
                results.push(result);
            }
            Err(e) => println!("Error en {}: {}", lang, e),
        }
    });

    let results = results.lock().unwrap();
    save_to_csv(&results);

    log_separator();
    println!("Datos guardados en ./languages/Datos_Almacenados.csv");
}