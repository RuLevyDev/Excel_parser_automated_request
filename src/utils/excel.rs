use super::parser::{
    process_duration, process_golpe, process_num_jugadores, split_to_vec,
    translate_level_to_english, translate_material, translate_model_to_english,
    translate_part_to_practice, translate_shot_to_english, translate_typology_to_english,
};
use crate::models::{activity::Actividad, activity_sections::Seccion, content::Content};
use calamine::{open_workbook, DataType, Reader, Xlsx};
use std::collections::HashMap;
use std::error::Error;

/// Extracts content information from a row in an Excel sheet and organizes it
/// into a `HashMap` keyed by language codes (e.g., "ES" for Spanish).
///
/// # Arguments
///
/// * `row` - A slice of `DataType` representing a row of data from the Excel sheet.
/// * `start_index` - The starting index for extracting content in the given row.
///
/// # Returns
///
/// A `HashMap<String, Content>` containing the content for different languages.
pub fn create_content(row: &[DataType], start_index: usize) -> HashMap<String, Content> {
    let mut content = HashMap::new();
    if row.len() > start_index + 2 {
        content.insert(
            "ES".to_string(),
            Content {
                title: row[start_index].to_string(),
                goal: row[start_index + 1].to_string(),
                script: row[start_index + 2].to_string(),
            },
        );
    }
    // Additional languages can be added here.
    content
}

/// Constructs an `Actividad` instance using data extracted from a row in an Excel sheet.
///
/// # Arguments
///
/// * `row` - A slice of `DataType` representing a row of data from the Excel sheet.
/// * `start_index` - The starting index for extracting activity attributes.
/// * `content` - A `HashMap<String, Content>` containing the activity's content details.
/// * `phase` - A string indicating the phase of the activity (e.g., "WARM_UP").
///
/// # Returns
///
/// An `Option<Actividad>` that is `Some(Actividad)` if the row has sufficient data to create a valid activity,
/// or `None` if the data is incomplete.
pub fn create_actividad(
    row: &[DataType],
    start_index: usize,
    content: HashMap<String, Content>,
    phase: String,
) -> Option<Actividad> {
    if row.len() > start_index + 9 {
        Some(Actividad {
            golpe: process_golpe(&row[0].to_string()).unwrap_or(0),
            num_jugadores: process_num_jugadores(split_to_vec(&row[1].to_string().to_uppercase())),
            typology: translate_typology_to_english(split_to_vec(
                &row[2].to_string().to_uppercase(),
            )),
            level: translate_level_to_english(split_to_vec(&row[3].to_string().to_uppercase())),
            id: row[start_index].to_string(),
            model: translate_model_to_english(split_to_vec(
                &row[start_index + 1].to_string().to_uppercase(),
            )),
            shot: translate_shot_to_english(split_to_vec(
                &row[start_index + 2].to_string().to_uppercase(),
            )),
            part_to_practice: translate_part_to_practice(split_to_vec(
                &row[start_index + 3].to_string().to_uppercase(),
            )),
            equipment: translate_material(split_to_vec(
                &row[start_index + 4].to_string().to_uppercase(),
            )),
            duration: process_duration(&row[start_index + 5].to_string()),
            content,
            phase,
        })
    } else {
        None
    }
}

/// Loads activities from an Excel sheet and organizes them into sections.
///
/// Activities are categorized into four sections: warming up, main exercise (split into two parts),
/// and the final part.
///
/// # Arguments
///
/// * `path` - The file path to the Excel file.
/// * `sheet_name` - The name of the sheet containing the activity data.
///
/// # Returns
///
/// A `Result<Seccion, Box<dyn Error>>` containing the activity sections, or an error message if the sheet cannot be read.
pub fn load_actividades(path: &str, sheet_name: &str) -> Result<Seccion, Box<dyn Error>> {
    let mut workbook: Xlsx<_> = open_workbook(path)?;
    if let Some(Ok(range)) = workbook.worksheet_range(sheet_name) {
        let mut calentamiento = Vec::new();
        let mut ejercicio1 = Vec::new();
        let mut ejercicio2 = Vec::new();
        let mut parte_final = Vec::new();

        for row in range.rows().skip(3) {
            if row.len() >= 68 {
                // Extract warming-up activities
                if let Some(act) =
                    create_actividad(row, 4, create_content(row, 11), "WARM_UP".to_string())
                {
                    calentamiento.push(act);
                }
                // Extract main exercise phase 1
                if let Some(act) = create_actividad(
                    row,
                    20,
                    create_content(row, 27),
                    "MAIN_EXERCISE".to_string(),
                ) {
                    ejercicio1.push(act);
                }
                // Extract main exercise phase 2
                if let Some(act) = create_actividad(
                    row,
                    36,
                    create_content(row, 43),
                    "MAIN_EXERCISE".to_string(),
                ) {
                    ejercicio2.push(act);
                }
                // Extract final part activities
                if let Some(act) =
                    create_actividad(row, 52, create_content(row, 59), "FINAL_PART".to_string())
                {
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
        Err("Worksheet not found or unreadable.".into())
    }
}
