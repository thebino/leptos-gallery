use crate::LeptosAppState;
use app::models::cart::Cart;
use app::models::photo::Photo;
use axum::extract::State;
use axum::Json;
use sqlx::types::Uuid;

pub async fn add_to_cart(
    State(app_state): State<LeptosAppState>,
    Json(cart): Json<Cart>,
) -> Json<String> {
    let id = Uuid::new_v4();

    let items_json = serde_json::to_string(&cart.items).unwrap();

    sqlx::query("INSERT INTO carts (items) VALUES (?)")
        .bind(items_json)
        .execute(&app_state.pool)
        .await
        .unwrap();

    Json(id.to_string())
}

pub async fn get_cart(
    State(app_state): State<LeptosAppState>,
    Json(id): Json<Uuid>,
) -> Json<Option<Cart>> {
    let row = sqlx::query_as::<_, (i32, String)>("SELECT id, items FROM carts WHERE id = ?")
        .bind(id.to_string())
        .fetch_optional(&app_state.pool)
        .await
        .unwrap();

    if let Some((id, items)) = row {
        let items: Vec<Photo> = serde_json::from_str(&items).unwrap();
        Json(Some(Cart {
            id: Some(id),
            items,
        }))
    } else {
        Json(None)
    }
}
