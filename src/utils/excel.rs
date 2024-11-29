use super::parser::split_to_vec;
use crate::models::{activity::Actividad, activity_sections::Seccion, content::Content};
use calamine::{open_workbook, DataType, Reader, Xlsx};
use std::collections::HashMap;
use std::error::Error;

/// Creates a `HashMap<String, Content>` from a row in an Excel sheet, extracting
/// content information such as title, objective, and script for a specific language.
///
/// # Arguments
///
/// * `row` - A slice of `DataType` representing a row of data from the Excel sheet.
/// * `start_index` - The index at which to start extracting content for the current language.
///
/// # Returns
///
/// A `HashMap<String, Content>` containing the content for different languages, keyed by language code.
pub fn create_content(row: &[DataType], start_index: usize) -> HashMap<String, Content> {
    let mut content = HashMap::new();
    if row.len() > start_index + 2 {
        content.insert(
            "ES".to_string(),
            Content {
                title: row[start_index].to_string(),
                objetivo: row[start_index + 1].to_string(),
                script: row[start_index + 2].to_string(),
            },
        );
    }
    // Uncomment and extend for additional languages like "EN", "IT" if needed
    content
}

/// Creates an `Actividad` from a row in an Excel sheet, using the provided `content`
/// and extracting relevant activity details such as the type of shot, number of players, and materials.
///
/// # Arguments
///
/// * `row` - A slice of `DataType` representing a row of data from the Excel sheet.
/// * `start_index` - The index at which to start extracting the activity's attributes.
/// * `content` - A `HashMap<String, Content>` containing the content associated with the activity.
///
/// # Returns
///
/// An `Option<Actividad>` which is `Some(Actividad)` if the row has enough data to form a valid activity,
/// or `None` if the row is incomplete.
pub fn create_actividad(
    row: &[DataType],
    start_index: usize,
    content: HashMap<String, Content>,
    phase: String, // New parameter to indicate the phase
) -> Option<Actividad> {
    if row.len() > start_index + 9 {
        Some(Actividad {
            golpe: row[0].to_string(),
            num_jugadores: split_to_vec(&row[1].to_string()),
            tipologia_jugador: split_to_vec(&row[2].to_string()),
            nivel: split_to_vec(&row[3].to_string()),
            id: row[start_index].to_string(),
            goal: split_to_vec(&row[start_index + 1].to_string()),
            shot: split_to_vec(&row[start_index + 2].to_string()),
            part_to_practice: split_to_vec(&row[start_index + 3].to_string()),
            material: split_to_vec(&row[start_index + 4].to_string()),
            tiempo: row[start_index + 5].to_string(),
            content,
            phase,
        })
    } else {
        None
    }
}

/// Loads activities from an Excel sheet and organizes them into a `Seccion` struct,
/// categorizing them into warming up, exercises, and final sections.
///
/// # Arguments
///
/// * `path` - The path to the Excel file.
/// * `sheet_name` - The name of the sheet containing the activities.
///
/// # Returns
///
/// A `Result<Seccion, Box<dyn Error>>` that contains the loaded section data, or an error message if the sheet couldn't be read.
pub fn load_actividades(path: &str, sheet_name: &str) -> Result<Seccion, Box<dyn Error>> {
    let mut workbook: Xlsx<_> = open_workbook(path)?;
    if let Some(Ok(range)) = workbook.worksheet_range(sheet_name) {
        let mut calentamiento = Vec::new();
        let mut ejercicio1 = Vec::new();
        let mut ejercicio2 = Vec::new();
        let mut parte_final = Vec::new();

        for row in range.rows().skip(3) {
    if row.len() >= 68 {
        // Calentamiento (warming-up phase)
        if let Some(act) = create_actividad(
            row,
            4,
            create_content(row, 11),
            "calentamiento".to_string(),
        ) {
            calentamiento.push(act);
        }
        // Ejercicio 1 (exercise 1 phase)
        if let Some(act) = create_actividad(
            row,
            20,
            create_content(row, 27),
            "ejercicio1".to_string(),
        ) {
            ejercicio1.push(act);
        }
        // Ejercicio 2 (exercise 2 phase)
        if let Some(act) = create_actividad(
            row,
            36,
            create_content(row, 43),
            "ejercicio2".to_string(),
        ) {
            ejercicio2.push(act);
        }
        // Parte final (final phase)
        if let Some(act) = create_actividad(
            row,
            52,
            create_content(row, 59),
            "parte_final".to_string(),
        ) {
            parte_final.push(act);
        }
    }
}

        Ok(Seccion {
            calentamiento,
            ejercicio1,
            ejercicio2,
            parte_final,
        })
    } else {
        Err("No se encontró la hoja o no se pudo leer.".into())
    }
}
