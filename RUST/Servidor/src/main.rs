use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::thread;
use mime_guess;

/// Estructura principal del servidor web.
/// Contiene la dirección y el directorio raíz donde se sirven los archivos estáticos.
#[derive(Debug)]
pub struct WebServer {
    address: String,
    root_dir: String,
}

impl WebServer {
    /// Crea una nueva instancia del servidor web.
    ///
    /// # Parámetros
    /// * `address` - Dirección IP y puerto en formato "127.0.0.1:8080".
    /// * `root_dir` - Directorio desde donde se servirán los archivos estáticos.
    pub fn new(address: &str, root_dir: &str) -> Self {
        WebServer {
            address: address.to_string(),
            root_dir: root_dir.to_string(),
        }
    }

    /// Inicia el servidor web y comienza a escuchar conexiones.
    ///
    /// Este método bloquea el hilo principal mientras el servidor está activo.
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(&self.address)?;
        println!("Servidor escuchando en {}", self.address);

        // Escucha conexiones entrantes
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    // Clonamos el root_dir para moverlo al hilo
                    let root_dir = self.root_dir.clone();

                    // Maneja cada conexión en un hilo separado
                    thread::spawn(move || {
                        if let Err(e) = handle_request(stream, &root_dir) {
                            eprintln!("Error al manejar la solicitud: {}", e);
                        }
                    });
                }
                Err(e) => eprintln!("Error en la conexión: {}", e),
            }
        }

        Ok(())
    }
}

/// Maneja la solicitud HTTP entrante.
///
/// Lee la solicitud, obtiene el archivo solicitado y envía la respuesta adecuada.
fn handle_request(mut stream: TcpStream, root_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer)?;

    if bytes_read == 0 {
        return Err("Solicitud vacía".into());
    }

    let request = String::from_utf8_lossy(&buffer[..bytes_read]);

    // Analiza la solicitud
    let requested_path = parse_request(&request);

    // Genera la ruta completa del archivo
    let file_path = format!("{}/{}", root_dir, requested_path);

    // Si el archivo existe, lo servimos
    if Path::new(&file_path).exists() && Path::new(&file_path).is_file() {
        match serve_file(&mut stream, &file_path) {
            Ok(_) => {}
            Err(_) => send_internal_error(&mut stream)?,
        }
    } else {
        send_not_found(&mut stream)?;
    }

    Ok(())
}

/// Analiza la solicitud HTTP y extrae la ruta solicitada.
///
/// Si la solicitud es inválida, devuelve `index.html` como valor por defecto.
fn parse_request(request: &str) -> String {
    let mut lines = request.lines();
    if let Some(first_line) = lines.next() {
        let parts: Vec<&str> = first_line.split_whitespace().collect();
        if parts.len() >= 2 && parts[0] == "GET" {
            let path = parts[1].trim_start_matches('/');
            if path.is_empty() {
                return "index.html".to_string();
            }
            return path.to_string();
        }
    }
    "index.html".to_string()
}

/// Sirve el archivo solicitado al cliente.
///
/// Determina el tipo MIME y envía la respuesta con encabezados HTTP correctos.
fn serve_file(stream: &mut TcpStream, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file_content = fs::read(file_path)?;
    let mime_type = mime_guess::from_path(file_path).first_or_octet_stream();

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: {}; charset=utf-8\r\nContent-Length: {}\r\n\r\n",
        mime_type,
        file_content.len()
    );

    stream.write_all(response.as_bytes())?;
    stream.write_all(&file_content)?;
    Ok(())
}

/// Envía una respuesta 404 (Archivo no encontrado).
fn send_not_found(stream: &mut TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let body = "404 - Archivo no encontrado";
    let response = format!(
        "HTTP/1.1 404 NOT FOUND\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
        body.len(),
        body
    );
    stream.write_all(response.as_bytes())?;
    Ok(())
}

/// Envía una respuesta 500 (Error interno del servidor).
fn send_internal_error(stream: &mut TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let body = "500 - Error interno del servidor";
    let response = format!(
        "HTTP/1.1 500 INTERNAL SERVER ERROR\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
        body.len(),
        body
    );
    stream.write_all(response.as_bytes())?;
    Ok(())
}

/// Punto de entrada principal del programa.
///
/// Crea el servidor y lo ejecuta en el puerto 8080, sirviendo desde `./public`.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = WebServer::new("127.0.0.1:8080", "./public");
    server.run()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::thread;
    use std::time::Duration;
    use std::net::TcpStream;

    /// Prueba básica: iniciar el servidor y verificar que responde con 200 OK.
    #[test]
    fn test_server_serves_index() {
        // Prepara el entorno
        fs::create_dir_all("public").unwrap();
        fs::write("public/index.html", "<h1>Hola Ejecutaste los Test</h1>").unwrap();

        // Inicia el servidor en un hilo separado
        thread::spawn(|| {
            let server = WebServer::new("127.0.0.1:8081", "./public");
            server.run().unwrap();
        });

        // Espera a que el servidor arranque
        thread::sleep(Duration::from_millis(500));

        // Realiza una conexión TCP simulando un navegador
        let mut stream = TcpStream::connect("127.0.0.1:8081").unwrap();
        stream
            .write_all(b"GET / HTTP/1.1\r\nHost: localhost\r\n\r\n")
            .unwrap();

        let mut buffer = String::new();
        stream.read_to_string(&mut buffer).unwrap();

        // Comprueba que el servidor responde con 200 OK
        assert!(buffer.contains("200 OK"));
    }
}