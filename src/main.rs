mod models;
mod utils;

use std::error::Error;

use models::activity_sections::Seccion;
use utils::excel::load_actividades;
use utils::post_request::post_request;

fn main() -> Result<(), Box<dyn Error>> {
    // Load activities from the Excel file
    let path = "programming-table-2.xlsx";

    if !std::path::Path::new(path).exists() {
        println!("File not found at: {}", path);
        return Ok(());
    }

    // Load activities from the specified sheet
    let seccion: Seccion = load_actividades(path, "1. DERECHA PLANA")?;
    println!("{:#?}", seccion);

    // URL of the endpoint where the activities will be sent
    let endpoint =
        "";

    // Send warming-up activities
    for actividad in seccion.calentamiento {
        let actividad_json = serde_json::to_string(&actividad)?; // Convert the activity to JSON
        println!("Sending activity: {}", actividad.id);

        // Send the POST request
        match post_request(&actividad_json, endpoint) {
            Ok(response) => println!("Activity sent successfully: {}", response),
            Err(e) => eprintln!("Error sending activity '{}': {}", actividad.phase, e),
        }
    }

    // Send first set of exercises
    for actividad in seccion.ejercicio1 {
        let actividad_json = serde_json::to_string(&actividad)?; // Convert the activity to JSON
        println!("Sending activity: {}", actividad.id);

        // Send the POST request
        match post_request(&actividad_json, endpoint) {
            Ok(response) => println!("Activity sent successfully: {}", response),
            Err(e) => eprintln!("Error sending activity '{}': {}", actividad.phase, e),
        }
    }

    // Send second set of exercises
    for actividad in seccion.ejercicio2 {
        let actividad_json = serde_json::to_string(&actividad)?; // Convert the activity to JSON
        println!("Sending activity: {}", actividad.id);

        // Send the POST request
        match post_request(&actividad_json, endpoint) {
            Ok(response) => println!("Activity sent successfully: {}", response),
            Err(e) => eprintln!("Error sending activity '{}': {}", actividad.phase, e),
        }
    }

    // Send final phase activities
    for actividad in seccion.parte_final {
        let actividad_json = serde_json::to_string(&actividad)?; // Convert the activity to JSON
        println!("Sending activity: {}", actividad.id);

        // Send the POST request
        match post_request(&actividad_json, endpoint) {
            Ok(response) => println!("Activity sent successfully: {}", response),
            Err(e) => eprintln!("Error sending activity '{}': {}", actividad.phase, e),
        }
    }

    Ok(())
}
