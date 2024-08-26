# Leptos Gallery

[![License: CC BY 4.0](https://licensebuttons.net/l/by/4.0/80x15.png)](https://creativecommons.org/licenses/by/4.0/)

Showing a grid of photos with an authentication to select different albums with an ALBUMCODE and a PASSCODE acting as secret.
This application uses Server-Side Rendering manipulating the DOM.

## 🌱 Usage

Define the _root_path_ and an _admin secret_ in the config.
> The admin secret is only required for the start of the application.

```toml
root_dir = "./public/"
secret = "secret"
```

Manage albums via these endpoints:

- `POST /album` to create a new album - returns `201 - location /album/ABCD1234`
- `POST /album/ABCD1234` - to add new items to an album
- `DELETE /album/ABCD1234` - to delete an album - returns `204 No Content`
- `DELETE /album/ABCD1234/f80d31b6-8193-40a4-92ff-fcc6b1f6f284` - to delete an item from the album

All these endpoints need an Authentication header containing the admin secret: `Authentication: secret`


## 🚀 Running your project

```bash
cargo leptos watch
```


# ⚖️⚖️ License

Copyright 2024 by Stürmer, Benjamin <benjamin@stuermer.pro> is licensed under [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/)

```    
    Creative Commons Attribution 4.0 International (CC BY 4.0) which basically means:
    
    Share — copy and redistribute the material in any medium or format
    
    Adapt — remix, transform, and build upon the material
    
    for *any* purpose, even commercially.
    
    The licensor cannot revoke these freedoms as long as you follow the license terms.
    
```
