use std::{fs, path::Path};

use crate::models::photo::Photo;
use leptos::*;

#[component]
pub fn PhotoGrid(album: ReadSignal<String>, root: ReadSignal<String>) -> impl IntoView {
    let album_id: String = album.try_get().expect("Fail to read album from signal!");
    let root_path: String = format!("{}/{}", root.get(), album_id.clone());
    let root_exists = Path::new(&root_path).exists();
    if !root_exists {
        view! {
            <div>
            <p>"Directory is missing!"</p>
            </div>
        }
    } else {
        let thumb_path: String = format!("{}/thumbs", root_path);
        let photo_path: String = format!("{}/photos", root_path);
        let thumb_exists = Path::new(&thumb_path).exists();
        let photo_exists = Path::new(&photo_path).exists();
        
        if !thumb_exists | !photo_exists {
            // error
            view! {
                <div>
                <p>"This album does not contain any pictures!"</p>
                </div>
            }
        } else {
            // content
            let mut photos: Vec<Photo> = Vec::new();
            for entry in fs::read_dir(&thumb_path).unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();
                let filename = path.file_name().unwrap().to_string_lossy().into_owned();
                photos.push(Photo {
                    id: 1,
                    filename: format!("{}/{}", thumb_path.replace("public/", ""), filename),
                    url: format!("{}/{}", thumb_path.replace("public/", ""), filename),
                })
            }
            view! {
                <div>
                    <ul style="list-style-type:none;">
                    {photos.into_iter()
                        .map(|photo|
                            view! { <li>
                                <div class="photo">
                                <img style="width: 200px;" src={&photo.url} alt={&photo.filename} />
                                </div>
                            </li>}
                        )
                        .collect::<Vec<_>>()}
                    </ul>
                </div>
            }
        }
    }
}
