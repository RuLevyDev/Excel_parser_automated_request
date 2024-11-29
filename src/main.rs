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

    // We assume that "load_actividades" returns a Section with a list of activities
    let seccion: Seccion = load_actividades(path, "1. DERECHA PLANA")?;
    println!("{:#?}", seccion);

    // URL of the endpoint where the activities will be sent
    let endpoint = "";

    // Iterate over each activity and make a POST request
     for actividad in seccion.calentamiento {
        let actividad_json = serde_json::to_string(&actividad)?; // Convert the activity to JSON
        println!("Enviando actividad: {}", actividad.id);

         // Send the POST request
        match post_request(&actividad_json, endpoint) {
            Ok(response) => {
                println!("Actividad enviada correctamente: {}", response);
            }
            Err(e) => {
                eprintln!("Error al enviar la actividad '{}': {}", actividad.phase, e);
            }
        }

       
        // std::thread::sleep(std::time::Duration::from_secs(1));
    }
    for actividad in seccion.ejercicio1 {
        let actividad_json = serde_json::to_string(&actividad)?; // Convert the activity to JSON
        println!("Sending activity: {}", actividad.id);

        // Send the POST request
        match post_request(&actividad_json, endpoint) {
            Ok(response) => {
                println!("Activity sent successfully: {}", response);
            }
            Err(e) => {
                eprintln!("Error sending activity '{}': {}", actividad.phase, e);
            }
        }

        // If a small delay is desired between each request (for example, to avoid overloading the server):
        // std::thread::sleep(std::time::Duration::from_secs(1));
    }
    
  
    for actividad in seccion.ejercicio2 {
        let actividad_json = serde_json::to_string(&actividad)?; //  Convert the activity to JSON
        println!("Enviando actividad: {}", actividad.id);

         // Send the POST request
        match post_request(&actividad_json, endpoint) {
            Ok(response) => {
                println!("Actividad enviada correctamente: {}", response);
            }
            Err(e) => {
                eprintln!("Error al enviar la actividad '{}': {}", actividad.phase, e);
            }
        }

       
       
    }
     for actividad in seccion.ejercicio2 {
        let actividad_json = serde_json::to_string(&actividad)?; // Convert the activity to JSON
        println!("Enviando actividad: {}", actividad.id);

         // Send the POST request
        match post_request(&actividad_json, endpoint) {
            Ok(response) => {
                println!("Actividad enviada correctamente: {}", response);
            }
            Err(e) => {
                eprintln!("Error al enviar la actividad '{}': {}", actividad.phase, e);
            }
        }

       
      
    }
    

    Ok(())
}

