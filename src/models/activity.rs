use super::content::Content;
use serde::Serialize;
use std::collections::HashMap;

/// Represents an activity within the application.
///
/// This structure contains detailed information about an activity,
/// including its ID, type of shot, players involved, level, and other
/// aspects related to its content and configuration.
#[derive(Debug, Serialize)]
pub struct Actividad {
    /// Unique identifier for the activity.
    pub id: String,

    /// Type of shot associated with the activity.
    pub golpe: i32,

    /// Phase of shot associated with the activity.
    pub phase: String,

    /// List of player IDs involved in the activity.
    pub num_jugadores: Vec<i32>,

    /// Player typologies the activity is designed for.
    pub typology: Vec<String>,

    /// Skill levels targeted by the activity.
    pub level: Vec<String>,

    /// Objectives intended to be achieved through the activity.
    pub model: Vec<String>,

    /// Types of shots practiced in the activity.
    pub shot: Vec<String>,

    /// Body parts or skills to be practiced during the activity.
    pub part_to_practice: Vec<String>,

    /// Equipment or materials required for the activity.
    pub equipment: Vec<String>,

    /// Estimated duration of the activity, represented as text (e.g., "30 minutes").
    pub duration: String,

    /// Associated content for the activity, represented as a key-value map.
    /// Keys are strings, and values are instances of `Content`.
    pub content: HashMap<String, Content>,
}
