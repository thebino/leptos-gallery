pub mod add_album;
pub mod add_item;
pub mod delete_album;
pub mod delete_item;
pub mod get_album;
pub mod get_item;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Album {
    pub id: String,
}
