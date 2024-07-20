use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Photo {
    pub id: u32,
    pub filename: String,
    pub url: String,
}

impl PartialEq for Photo {
    fn eq(&self, other: &Self) -> bool {
        self.id != other.id
    }
}
