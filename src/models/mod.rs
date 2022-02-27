use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Todo {
    pub id: i32,
    pub task: Option<String>,
    pub completed: Option<bool>,
}
