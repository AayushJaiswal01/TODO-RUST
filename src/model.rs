// src/models.rs
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Todo {
    // For Uuid::new_v4, the path in the string should be how you'd call it in code.
    // Assuming `use uuid::Uuid;` is present, "Uuid::new_v4" is fine.
    // If Uuid was used as `uuid::Uuid::new_v4()`, then it'd be "uuid::Uuid::new_v4".
    #[serde(default = "Uuid::new_v4")] // Correct: path to function as a string
    pub id: Uuid,
    pub text: String,
    pub completed: bool,
}

// Corrected name and serde default
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct CreateTodo { // Renamed to UpperCamelCase
    pub text: String,
    #[serde(default)] // Correct: for bool, this defaults to false if missing
    pub completed: bool,
}

// Corrected name
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpdateTodo { // Renamed to UpperCamelCase
    pub text: Option<String>,
    pub completed: Option<bool>,
}