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
    data.split(',') // Split the string by commas
        .map(|s| s.trim().to_string()) // Trim whitespace and convert each split part to a String
        .collect() // Collect the results into a vector
}
