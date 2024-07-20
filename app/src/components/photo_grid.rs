use std::{fs, path::Path};

use crate::models::photo::Photo;
use leptos::*;
use tracing::info;

#[component]
pub fn PhotoGrid(album: ReadSignal<String>) -> impl IntoView {
    let root_path: String = album.get();
    let root_exists = Path::new(&root_path).exists();
    if !root_exists {
        view! {
            <div>
            <p>"Something went wrong!"</p>
            </div>
        }
    } else {
        let thumb_path: String = format!("{}/thumbs", root_path);
        let photo_path: String = format!("{}/photos", root_path);
        let thumb_exists = Path::new(&thumb_path).exists();
        let photo_exists = Path::new(&photo_path).exists();

        if !thumb_exists {
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
                info!("{:?}", filename);
                photos.push(Photo {
                    id: 1,
                    filename: format!("{}/{}", thumb_path, filename),
                    url: format!("{}/{}", thumb_path, filename),
                })
            }
            // let title_path: String = format!("{}/title.txt", root_path);
            // let title = fs::read_to_string(title_path).ok();
            view! {
                <div>
                    // <h1>{title}</h1>
                    <ul>
                    {photos.into_iter()
                        .map(|photo|
                            view! { <li>
                                <div class="photo">
                                <img src={&photo.url} alt={&photo.filename} />
                                <p>{photo.url}</p>
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
