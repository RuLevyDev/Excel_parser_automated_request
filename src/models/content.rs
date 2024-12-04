use serde::{Deserialize, Serialize};

/// Represents the content associated with an activity.
///
/// This structure holds information related to the content, such as the title,
/// the objective of the content, and a script that provides further details or instructions.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Content {
    /// Title of the content, describing what the content is about.
    pub title: String,

    /// Objective of the content, describing what is intended to be achieved with it.
    pub goal: String,

    /// Script associated with the content, which could be instructions or details
    /// about how to perform or understand the content.
    pub script: String,
}
