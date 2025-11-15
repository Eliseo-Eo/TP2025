# 🦀 Concurrent Static File Web Server in Rust

## 📘 Descripción general
Este proyecto implementa un **servidor web concurrente en Rust** capaz de servir archivos estáticos desde un directorio especificado.  
El servidor utiliza **la librería estándar de Rust** para manejar conexiones TCP y crear hilos concurrentes, además de usar el crate [`mime_guess`](https://crates.io/crates/mime_guess) para determinar los tipos MIME según las extensiones de archivo.

El servidor soporta múltiples solicitudes simultáneamente y responde con los códigos HTTP apropiados (200, 404, 500).  
Es un ejercicio práctico de **concurrencia, manejo de archivos, redes y control de errores en Rust**.

---

## ⚙️ Características principales
- ✅ Usa `std::net::TcpListener` y `std::thread` para manejar múltiples clientes.
- ✅ Sirve archivos estáticos desde un directorio local.
- ✅ Detecta automáticamente el tipo MIME de cada archivo.
- ✅ Maneja errores comunes: archivo no encontrado (404) y error interno del servidor (500).
- ✅ Incluye comentarios y documentación en formato Rustdoc (`///`).
- ✅ Pruebas automáticas de integración con `cargo test`.

---

## 🏗️ Estructura del proyecto

servidor/
├── Cargo.toml
├── src/
│ └── main.rs
└── public/
└── index.html


## Archivos importantes:
- **`src/main.rs`** → Contiene el código fuente principal del servidor.  
- **`public/`** → Directorio donde se almacenan los archivos estáticos (HTML, CSS, imágenes, etc.).  
- **`Cargo.toml`** → Archivo de configuración del proyecto y dependencias.

---

## 🧩 Dependencias

El proyecto requiere únicamente el crate [`mime_guess`](https://crates.io/crates/mime_guess) para determinar los tipos MIME de los archivos servidos.

Agrega esta línea a tu `Cargo.toml`:

```toml
[dependencies]
mime_guess = "2.0"

🚀 Cómo ejecutar el proyecto

1️⃣ Clonar o crear el proyecto

Si ya tienes el proyecto creado:
cd servidor

O créalo desde cero:
cargo new servidor
cd servidor

2️⃣ Agregar la dependencia

Edita tu Cargo.toml y agrega:
[dependencies]
mime_guess = "2.0"

3️⃣ Crear el directorio público y un archivo HTML

mkdir public
echo "<h1>Servidor en Rust funcionando ✅</h1>" > public/index.html

4️⃣ Ejecutar el servidor

cargo run

Si todo está correcto, verás en consola:
Servidor escuchando en 127.0.0.1:8080

5️⃣ Abrir el navegador

Ingresa en tu navegador a:
👉 http://127.0.0.1:8080

Si ves tu mensaje HTML, ¡el servidor está funcionando correctamente! 🎉
🧠 Cómo usarlo

Coloca dentro de la carpeta public/ todos los archivos que desees servir:

public/
├── index.html
├── about.html
├── style.css
└── images/
    └── logo.png

Luego podrás acceder desde el navegador así:

    http://127.0.0.1:8080/

→ muestra index.html

http://127.0.0.1:8080/about.html

http://127.0.0.1:8080/style.css

http://127.0.0.1:8080/images/logo.png

🧪 Ejecutar pruebas automáticas

El proyecto incluye pruebas de integración que verifican que el servidor funcione correctamente.

Ejecuta las pruebas con:

cargo test

Si todo está correcto, verás:

running 1 test
test tests::test_server_serves_index ... ok

🧱 Estructura interna del código

El servidor está organizado de la siguiente manera:
Función / Método	Descripción
WebServer::new()	Crea una nueva instancia del servidor.
WebServer::run()	Inicia el servidor y escucha conexiones.
handle_request()	Procesa cada solicitud entrante.
parse_request()	Analiza la línea inicial del mensaje HTTP.
serve_file()	Envía el archivo solicitado al cliente.
send_not_found()	Envía una respuesta 404 si el archivo no existe.
send_internal_error()	Envía una respuesta 500 en caso de error del servidor.
🧰 Errores HTTP manejados
Código	Descripción	Situación
200 OK	Respuesta exitosa	Archivo encontrado y servido correctamente.
404 NOT FOUND	Archivo no encontrado	El archivo solicitado no existe.
500 INTERNAL SERVER ERROR	Error interno	Problemas al leer o enviar un archivo.


💡 Notas adicionales

    El servidor se ejecuta hasta que se detiene manualmente con Ctrl + C.

    Puedes cambiar el puerto o la carpeta raíz modificando esta línea en main():

    let server = WebServer::new("127.0.0.1:8080", "./public");

    Para usar otro puerto, por ejemplo 8081:

    let server = WebServer::new("127.0.0.1:8081", "./public");


💡 Bibliografia

1.- The Rust Programming Language (“Rust Book”) — Capítulo sobre concurrencia: “Using Threads to Run Code Simultaneously”. 
doc.rust-lang.org
URL: https://doc.rust-lang.org/book/ch16-01-threads.html
→ Explica cómo usar hilos con std::thread, cuáles son los retos de concurrencia en Rust.

2.- ust estándar: módulo de red (std::net) — Documentación de TcpListener y TcpStream para escuchar conexiones TCP. 
doc.rust-lang.org
URL: https://doc.rust-lang.org/std/net/index.html
→ Muestra cómo construir un servidor TCP escuchando, aceptar conexiones, etc.

3.- mime_guess — Crate de Rust para “adivinar” el tipo MIME a partir de la extensión del archivo. Documentación oficial. 
docs.rs
URL: https://docs.rs/mime_guess/latest
→ Útil para tu requisito de servir archivos con el tipo MIME correcto.

4.- Rust Book — Capítulo “Final Project: Building a Multithreaded Web Server”. 
doc.rust-lang.org
URL: https://doc.rust-lang.org/book/ch21-00-final-project-a-web-server.html
→ Un ejemplo completo que va muy alineado con el trabajo (HTTP, TCP, concurrencia).

5.- Artículo: “Concurrency and Multithreading in Rust” (MasteringBackend) — análisis más amplio sobre concurrencia en Rust. 
Mastering Backend
URL: https://masteringbackend.com/hubs/intermediate-rust/concurrency-and-multithreading-in-rust
→ Buen complemento para ver enfoques, patrones y mejores prácticas.



👩‍💻 Créditos

Proyecto creado con ❤️ en Rust
Autor: Guillermo Eliseo Guzman Lopez
Materia: TP2025
Año: 2025