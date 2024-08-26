use leptos::*;

#[island]
pub fn PhotoGrid(images: Vec<String>) -> impl IntoView {
    view! {
        <div>
        <ul style="list-style-type:none;">
            {images.into_iter()
                .map(|photo|
                    view! { <li>
                        <div class="photo">
                            <img style="width: 200px;" src={format!("data:image/jpeg;base64,{}", photo)} />
                        </div>
                    </li>}
                )
                .collect::<Vec<_>>()}
            </ul>
        </div>
    }
}
