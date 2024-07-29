use std::fs;

use axum::response::{Json, IntoResponse};

use crate::file::FileExtension;

fn get_images() -> impl Iterator<Item = String> {
    let paths = fs::read_dir("../storage/public/images").unwrap();
    paths.map(|x| x.unwrap())
        .filter(|x| x.metadata().unwrap().is_file())
        .map(|x| x.path())
        .filter(FileExtension::is_image)
        .map(|path| path.into_os_string().into_string().unwrap()
             .chars().skip(10).collect::<String>())
}

pub async fn get_image_dir() -> impl IntoResponse {
    Json(get_images().collect::<Vec<String>>())
}

// TODO: Use html template crate ore remove concept completely
// TODO: change the unwrap()
fn generate_image_dir_string() -> String {
    let img_elems = get_images().fold("".to_owned(), |acc, str_path| {
            acc + &format!(
                r#"
<a href="{0}" target="_blank">
    <img src="{0}" alt="{0}">
</a>
                "#,
                str_path) + "\n"
        });
r###"
<!DOCTYPE html>
<html lang="en">
<head>
	<title>Bee</title>
	<meta charset="utf-8">
	<meta name="viewport" content="width=device-width, initial-scale=1, maximum-scale=1, user-scalable=no">
	<link rel="stylesheet" type="text/css" href="style.css">
	<link rel="icon" type="image/x-icon" href="images/favicon.ico">
</head>
<body>
    <div class="image-dir">
"###.to_string() + &img_elems + r###"
    </div>
</body>
</html>
"###
}

pub fn generate_image_dir() {
    fs::write("../storage/public/index.html", generate_image_dir_string()).unwrap();
}
