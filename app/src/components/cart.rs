use crate::models::photo::Photo;
use leptos::{component, view, For, IntoView, ReadSignal, SignalGet, SignalUpdate, WriteSignal};
use rand::Rng;

#[component]
pub fn CartIcon(cart: ReadSignal<Vec<Photo>>) -> impl IntoView {
    view! {
        {move || if !cart.get().is_empty() {
            view! {
                <ul>
                    cart.get()
                    .iter()
                    .map(|photo| view! {
                        <li>{&photo.title}</li>
                    }
                    )
                </ul>
            }
        } else {
            view! {
                <ul>"Add your first item to the cart."</ul>
            }
        }}
    }
}

#[component]
pub fn CartItems(cart: ReadSignal<Vec<Photo>>, set_cart: WriteSignal<Vec<Photo>>) -> impl IntoView {
    let mut rng = rand::thread_rng();
    let on_click = move |_| {
        set_cart.update(|cart_items| {
            let id: u32 = rng.gen();
            cart_items.push(Photo {
                id,
                filename: "test".to_string(),
                url: "test2".to_string(),
            })
        });
    };

    view! {
        <button on:click=on_click>"Add random item"</button>

        <div><h2>Items</h2></div>
        <For
            each=move || cart.get()
            key=|cart_item| cart_item.id
            children=move |cart_item: Photo| {
              view! {
                <div on:click=move |_| {
                    set_cart.update(|photos| {
                        let index = photos.iter().position(|photo| photo.id == cart_item.id).unwrap();
                        photos.remove(index);
                    })}><p>"Value: " {move || cart_item.id }</p></div>
              }
            }
        />
    }
}
