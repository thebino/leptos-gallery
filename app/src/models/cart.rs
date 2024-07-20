use serde::{Deserialize, Serialize};

use super::photo::Photo;

#[derive(Clone, Serialize, Deserialize)]
pub struct Cart {
    pub id: Option<i32>,
    pub items: Vec<Photo>,
}
