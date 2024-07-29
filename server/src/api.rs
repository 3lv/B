use axum::{
    extract::{self, Path, Query, Multipart},
    response::{Response, IntoResponse, Redirect, Json},
    http::StatusCode,
};

use serde::{Serialize, Deserialize};

use chrono::prelude::*;

use std::{
    path::PathBuf,
    process::Command,
};

use crate::database::{self, CreateUser};

use crate::file::{FileExtension, FormFile};

// TODO: Look into AsRef<str>
trait IntoErrResponse {
    fn into_err_response(self) -> Response;
}
// TODO: Also use 400 (BAD REQUEST) etc
impl<T> IntoErrResponse for T
where
    T: IntoResponse,
{
    fn into_err_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self).into_response()
    }
}

//struct MyErr(Box<dyn std::error::Error>)
struct MyErr(diesel::result::Error);
impl IntoErrResponse for MyErr {
    fn into_err_response(self) -> Response {
        //(StatusCode::INTERNAL_SERVER_ERROR, format!("{0}", self.0)).into_response()
        IntoErrResponse::into_err_response(format!("{0}", self.0))
    }
}

pub async fn create_user(extract::Json(payload): extract::Json<CreateUser>) -> Response {
    match database::create_user(payload) {
        //Err(err) => format!("{err}").into_err_response(),
        Err(err) => MyErr(err).into_err_response(),
        Ok(()) => (StatusCode::OK, "user created").into_response()
    }
    /*
    use diesel::result::Error;
    use diesel::result::DatabaseErrorKind as DbError;
    match user::create_user(payload) {
        Err(Error::DatabaseError(db_error, _)) =>
            match db_error {
                DbError::UniqueViolation => "Username is already in use".into_err_response(),
                DbError::NotNullViolation => "Username and passwords must not be null".into_err_response(),
                DbError::CheckViolation => "Username must not be empty and password>=8".into_err_response(),
                _ => "Unkown database error".into_err_response()
            },
        Err(_) => "Unknown error".into_err_response(),
        Ok(()) => (StatusCode::OK, "User created").into_response()
    }
    */
    //user::create_user(payload).map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Could not save user").into_response())?;
    //Ok((StatusCode::OK, "User created").into_response())
}

pub async fn get_users() -> Result<impl IntoResponse, Response> {
    Ok(Json(database::get_users().map_err(|e| format!("{e}").into_err_response())?))
}


#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    secrets: Option<Vec<String>>,
}

pub async fn json_response() -> impl IntoResponse {
    let person = Person {
        name: "Cazan".to_owned(),
        age: 19,
        secrets: Some(vec![
            "Mi am luat bully toata viata!".to_owned(),
            "Nu mai vreau sa ma cheme cazan".to_owned()
        ]),
    };
    Json(person)
}

pub async fn save_image(mut multipart: Multipart) -> Result<impl IntoResponse, Response> {
    if let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR.into_response())?
    {
        if let Some("image") = field.name() {
            let file: FormFile = FormFile::async_from(field).await
                .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Couldn't receive file").into_response())?;
            let file_name = Utc::now().to_string() + "." + &file.name.extension().unwrap_or("none");
            let save_path: PathBuf = ["../storage/public/images", &file_name].iter().collect();
            std::fs::write(&save_path, file.bytes)
                .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Could not save file").into_response())?;
        }
    }
    Ok((StatusCode::OK, "image saved").into_response())
}

pub async fn set_background_handler(mut multipart: Multipart) -> Result<impl IntoResponse, Response> {
    if let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR.into_response())?
    {
        if let Some("image") = field.name() {
            let file: FormFile = FormFile::async_from(field).await
                .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Couldn't receive file").into_response())?;
            let file_name = Utc::now().to_string() + "." + &file.name.extension().unwrap_or("none");
            let save_path: PathBuf = ["../storage/uploads", &file_name].iter().collect();
            std::fs::write(&save_path, file.bytes).map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Could not save file").into_response())?;
            set_background(&save_path, BgOpt::Stretch).map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Could not set image as background").into_response())?;
        }
    }
    //Ok((StatusCode::OK, "Background changed!").into_response())
    Ok(Redirect::to("/"))
}

enum BgOpt {
    Stretch,
    #[allow(dead_code)]
    NoStretch,
}

fn set_background(image_path: &PathBuf, option: BgOpt) -> Result<(), std::io::Error> {
    let arg = match option {
        BgOpt::Stretch => "--bg-scale",
        BgOpt::NoStretch => "--bg-max"
    };
    Command::new("feh")
        .arg(arg)
        .arg(image_path)
        .status()?;
    Ok(())
}

pub async fn reload_image_dir() -> &'static str {
    use crate::image_dir;
    image_dir::generate_image_dir();
    "reloaded"
}

#[derive(Deserialize)]
pub struct PathParams {
    username: String,
}

pub async fn user_profile(
    Path(PathParams { username }): Path<PathParams>
) -> String {
    username + " profile was never created. (wip)"
}

// TODO: Look into #[serde(default = "fn_name")]
#[derive(Deserialize)]
pub struct SecretParams {
    name: Option<String>,
    pass: Option<String>,
}

pub async fn secret(
    Query(SecretParams { name, pass }): Query<SecretParams>
) -> &'static str {
    if name == Some("sexy".to_string()) && pass == Some("cazan".to_string()) {
r###"
                               _.._
                             .'    '.
                            (____/`\ \
        Sexy nu?           (  |' ' )  )
                           )  _\ _/  (
                 __..---.(`_.'  ` \    )
                `;----._(_( .      `; (
                /       `-`'--'     ; )
               /    /  .    ( .  ,| |(
_.-`'---...__,'    /-,..___.-'--'_| |_)
'-'``'-.._       ,'  |   / .........'
          ``;---`;   |   `-`
             `'..__.'
"###
    } else {
r###"
                   -`
                  .o+`
                 `ooo/
                `+oooo:
               `+oooooo:
               -+oooooo+:
             `/:-:++oooo+:
            `/++++/+++++++:
           `/++++++++++++++:
          `/+++ooooooooooooo/`
         ./ooosssso++osssssso+`
        .oossssso-````/ossssss+`
       -osssssso.      :ssssssso.
      :osssssss/        osssso+++.
     /ossssssss/        +ssssooo/-
   `/ossssso+/:-        -:/+osssso+-
  `+sso+:-`                 `.-/+oso:
 `++:.                           `-/+/
 .`                                 `
 */
"###
    }
}
