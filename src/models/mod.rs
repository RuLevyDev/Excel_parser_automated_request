/// Module containing the `Programing` struct and related functionality.
/// This module defines the structure for an `Actividad`, which represents an activity
/// with various fields such as `id`, `golpe`, `num_jugadores`, and more. It is used to
/// model the activities in the context of training sessions.
pub mod activity;

/// Module defining the structure for activity sections in a training program.
/// This module includes the `Seccion` struct, which organizes different types of activities
/// (e.g., calentamiento, ejercicio1, ejercicio2, parte_final) within the context of a session.
pub mod activity_sections;

/// Module containing content information for training activities.
/// This module defines the `Content` struct, which holds information like the title,
/// objective, and script for each activity in different languages (e.g., Spanish, English).
pub mod content;
