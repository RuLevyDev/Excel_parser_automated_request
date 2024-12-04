use std::collections::HashMap;

/// Splits a comma-separated string into a vector of trimmed strings.
///
/// This function takes a string that contains comma-separated values, splits it by commas,
/// trims any leading or trailing whitespace from each item, and returns a `Vec<String>`
/// containing the resulting strings.
///
/// # Arguments
///
/// * `data` - A string slice (`&str`) that contains comma-separated values.
///
/// # Returns
///
/// A `Vec<String>` containing the individual strings after splitting and trimming.
pub fn split_to_vec(data: &str) -> Vec<String> {
    data.split(',').map(|s| s.trim().to_string()).collect()
}

/// Processes the duration text to transform it into a readable format.
///
/// # Arguments
///
/// * `raw_duration` - The raw duration string from the Excel sheet.
///
/// # Returns
///
/// A formatted duration string, e.g., "10m - 5m/pareja".
pub fn process_duration(raw_duration: &str) -> String {
    let cleaned = raw_duration.replace("\n", "");
    let parts: Vec<&str> = cleaned.split(&['(', ')'][..]).collect();

    if parts.len() == 3 {
        let main_part = parts[0].replace("'", "m").trim().to_string();
        let sub_part = parts[1].replace("'", "m").trim().to_string();
        format!("{} - {}", main_part, sub_part)
    } else {
        cleaned.replace("'", "m").trim().to_string()
    }
}

/// Processes the list of player counts, extracting integers from strings like "4 JUGADORES".
///
/// # Arguments
///
/// * `players` - A vector of strings containing the player information.
///
/// # Returns
///
/// A vector of integers representing the number of players.
pub fn process_num_jugadores(players: Vec<String>) -> Vec<i32> {
    players
        .into_iter()
        .filter_map(|player| {
            player
                .split_whitespace()
                .next()
                .and_then(|num| num.parse::<i32>().ok())
        })
        .collect()
}

/// Extracts the number from a string representing a shot.
///
/// # Arguments
///
/// * `golpe` - A string slice containing the shot data.
///
/// # Returns
///
/// An `Option<i32>` containing the parsed number, or `None` if parsing fails.
pub fn process_golpe(golpe: &str) -> Option<i32> {
    golpe
        .split_whitespace()
        .next()
        .and_then(|num| num.parse::<i32>().ok())
}

/// Translates typology strings from Spanish to English.
///
/// # Arguments
///
/// * `typology` - A vector of strings in Spanish representing typologies.
///
/// # Returns
///
/// A vector of translated typologies in English.
pub fn translate_typology_to_english(typology: Vec<String>) -> Vec<String> {
    let translation_map: HashMap<&str, &str> = [
        ("INICIACIÓN INFANTIL", "KIDS_BEGINNERS"),
        ("INICIACIÓN ADULTO", "ADULT_BEGINNERS"),
        ("PERFECCIONAMIENTO", "IMPROVEMENT"),
        ("COMPETICIÓN", "TOUR_COMPETITION"),
        ("PRE-COMPETICIÓN", "PRECOMPETITION"),
    ]
    .iter()
    .cloned()
    .collect();

    typology
        .into_iter()
        .map(|item| {
            translation_map
                .get(item.as_str())
                .unwrap_or(&item.as_str())
                .to_string()
        })
        .collect()
}

/// Translates level strings from Spanish to English.
///
/// # Arguments
///
/// * `level` - A vector of strings in Spanish representing levels.
///
/// # Returns
///
/// A vector of translated levels in English.
pub fn translate_level_to_english(level: Vec<String>) -> Vec<String> {
    let translation_map: HashMap<&str, &str> = [
        ("DIFÍCIL", "DIFFICULT"),
        ("MEDIO", "MEDIUM"),
        ("FÁCIL", "EASY"),
    ]
    .iter()
    .cloned()
    .collect();

    level
        .into_iter()
        .map(|item| {
            translation_map
                .get(item.as_str())
                .unwrap_or(&item.as_str())
                .to_string()
        })
        .collect()
}

/// Translates model strings from Spanish to English.
///
/// # Arguments
///
/// * `model` - A vector of strings in Spanish representing models.
///
/// # Returns
///
/// A vector of translated models in English.
pub fn translate_model_to_english(model: Vec<String>) -> Vec<String> {
    let translation_map: HashMap<&str, &str> = [
        ("TÉCNICA", "TECHNIQUE"),
        ("TÁCTICA", "TACTIC"),
        ("SOCIAL", "SOCIAL"),
        ("FÍSICO", "PHYSIQUE"),
    ]
    .iter()
    .cloned()
    .collect();

    model
        .into_iter()
        .map(|item| {
            translation_map
                .get(item.as_str())
                .unwrap_or(&item.as_str())
                .to_string()
        })
        .collect()
}

/// Translates shot strings from Spanish to English and converts them to uppercase.
///
/// # Arguments
///
/// * `shot` - A vector of strings in Spanish representing shots.
///
/// # Returns
///
/// A vector of translated shots in English (uppercase).
pub fn translate_shot_to_english(shot: Vec<String>) -> Vec<String> {
    let translation_map: HashMap<&str, &str> = [
        ("DERECHA", "Forehand"),
        ("REVÉS", "Backhand"),
        ("PARED DE FONDO", "Back Wall"),
        ("PARED LATERAL REVÉS", "Backhand Side Wall"),
        ("PARED LATERAL DE DERECHA", "Forehand Side Wall"),
        ("DOBLE PARED DE REVÉS", "Double Backhand Wall"),
        ("DOBLE PARED DE DERECHA", "Double Forehand Wall"),
        ("OVERHEAD", "Overhead"),
        ("VOLEA", "Volley"),
        ("GOLPE A LA REJA", "Fence Hit"),
        ("TRICK SHOT", "Trick Shot"),
        ("SAQUE", "Serve"),
        ("RESTO", "Return"),
    ]
    .iter()
    .cloned()
    .collect();

    shot.into_iter()
        .map(|item| {
            translation_map
                .get(item.as_str())
                .unwrap_or(&item.as_str())
                .to_uppercase()
        })
        .collect()
}

/// Translates practice part strings from Spanish to English and converts them to uppercase.
///
/// # Arguments
///
/// * `part` - A vector of strings in Spanish representing parts to practice.
///
/// # Returns
///
/// A vector of translated parts to practice in English (uppercase).
pub fn translate_part_to_practice(part: Vec<String>) -> Vec<String> {
    let translation_map: HashMap<&str, &str> = [
        ("DIRECCIÓN", "DIRECTION"),
        ("POSICIONAMIENTO", "POSITIONING"),
        ("ESTRATEGIA", "STRATEGY"),
        ("POTENCIA", "POWER"),
        ("PROFUNDIDAD", "DEPTH"),
        ("ALTURA", "HEIGHT"),
    ]
    .iter()
    .cloned()
    .collect();

    part.into_iter()
        .map(|item| {
            translation_map
                .get(item.as_str())
                .unwrap_or(&item.as_str())
                .to_uppercase()
        })
        .collect()
}

/// Translates material strings from Spanish to English and converts them to uppercase.
///
/// # Arguments
///
/// * `material` - A vector of strings in Spanish representing materials.
///
/// # Returns
///
/// A vector of translated materials in English (uppercase).
pub fn translate_material(material: Vec<String>) -> Vec<String> {
    let translation_map: HashMap<&str, &str> = [
        ("CONOS", "CONES"),
        ("LÍNEAS DE SEÑALIZACIÓN AMARILLAS", "YELLOW_MARKING_LINES"),
        ("CONOS GRANDES", "LARGE_CONES"),
        ("ESCALERA DE COORDINACIÓN", "COORDINATION_LADDER"),
        ("RIÑONERA DE PELOTAS", "BALL_BELT"),
        ("AROS", "HOOPS"),
        ("MINI RED", "MINI_NET"),
        ("PICKIS RECOGE BOLAS", "BALL_PICKER_PICKIS"),
        ("MINI PORTERIA", "MINI_GOAL"),
    ]
    .iter()
    .cloned()
    .collect();

    material
        .into_iter()
        .map(|item| {
            translation_map
                .get(item.as_str())
                .unwrap_or(&item.as_str())
                .to_uppercase()
        })
        .collect()
}
