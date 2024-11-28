mod models;
mod utils;

use std::error::Error;

use models::activity_sections::Seccion;
use utils::excel::load_actividades;
use utils::post_request::post_request;

fn main() -> Result<(), Box<dyn Error>> {
    // Cargar actividades desde el archivo Excel
    let path = "programming-table-2.xlsx";

    if !std::path::Path::new(path).exists() {
        println!("Archivo no encontrado en: {}", path);
        return Ok(());
    }

    // Suponemos que "load_actividades" devuelve una Sección con una lista de actividades
    let seccion: Seccion = load_actividades(path, "1. DERECHA PLANA")?;
    println!("{:#?}", seccion);

    // URL del endpoint donde se enviarán las actividades
    let endpoint = "";

    // Iterar sobre cada actividad y hacer un POST request
    for actividad in seccion.ejercicio1 {
        let actividad_json = serde_json::to_string(&actividad)?; // Convertir la actividad a JSON
        println!("Enviando actividad: {}", actividad.id);

        // Enviar la solicitud POST
        match post_request(&actividad_json, endpoint) {
            Ok(response) => {
                println!("Actividad enviada correctamente: {}", response);
            }
            Err(e) => {
                eprintln!("Error al enviar la actividad '{}': {}", actividad.phase, e);
            }
        }

        // Si se desea un pequeño retraso entre cada solicitud (por ejemplo, para evitar sobrecargar el servidor):
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    Ok(())
}
