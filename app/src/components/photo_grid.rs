use leptos::*;

#[island]
pub fn PhotoGrid(images: Vec<String>) -> impl IntoView {
    view! {
        <section class="text-gray-600 body-font">
        <div class="container px-5 py-10 mx-auto lg:pt-12 lg:px-32">
            <div class="grid grid-cols-3 gap-3" id="lightgallery">
            {images.into_iter()
                .map(|photo|
                    view! {
                        <img style="width: 400px;" src={format!("data:image/jpeg;base64,{}", photo)} class="block object-cover object-center w-full h-full rounded-lg" loading="lazy" />
                    }
                )
                .collect::<Vec<_>>()}
             </div>
          </div>
        </section>
    }
}
