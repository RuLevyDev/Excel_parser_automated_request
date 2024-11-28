use super::activity::Actividad;

/// Represents a section of an activity, which contains different stages or phases.
///
/// This structure is used to organize an activity into distinct parts, such as
/// warming up, performing exercises, and finishing with a final phase.
#[derive(Debug)]
pub struct Seccion {
    /// List of activities for the warm-up phase of the section.
    pub calentamiento: Vec<Actividad>,

    /// List of activities for the first exercise phase of the section.
    pub ejercicio1: Vec<Actividad>,

    /// List of activities for the second exercise phase of the section.
    pub ejercicio2: Vec<Actividad>,

    /// List of activities for the final phase of the section.
    pub parte_final: Vec<Actividad>,
}
