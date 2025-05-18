// src/models.rs
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Todo {
    #[serde(default = "Uuid::new_v4")] 
    pub id: Uuid,
    pub text: String,
    pub completed: bool,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct CreateTodo { 
    pub text: String,
    #[serde(default)] 
    pub completed: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpdateTodo { 
    pub text: Option<String>,
    pub completed: Option<bool>,
}