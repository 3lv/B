use axum::{
    body::{Body, Bytes},
    extract::{self, Request, Path, Query, Multipart},
    response::{
        Response, IntoResponse, Redirect, Json,
    },
    http::{StatusCode, header},
};

use serde::{Serialize, Deserialize};

use chrono::prelude::*;

use std::{
    fs,
    path::PathBuf,
    process::Command,
};

use tokio_util::io::ReaderStream;

use crate::database::{self, CreateUser, Username};

use crate::file::{FileExtension, FileRename, FormFile};

// TryFromField
use axum_typed_multipart::{FieldData, TryFromMultipart, TypedMultipart};

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

pub async fn is_username_available(username: Username) -> impl IntoResponse {
    match !database::user_exists(username) {
        true => "0",
        false => "1",
    }
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
                    let file: FormFile = FormFile::async_from_field(field).await
                        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Couldn't receive file").into_response())?;
                    let dir = PathBuf::from("../storage/public/images");
                    file.save_as_date(&dir).map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Could not save file").into_response())?;
                }
            }
    Ok((StatusCode::OK, "image saved").into_response())
}

#[derive(TryFromMultipart)]
pub struct BackgroundForm {
    #[form_data(limit = "10MB")]
    image: FieldData<Bytes>,
    password: String,
}


pub async fn set_background_handler(data: TypedMultipart<BackgroundForm>) -> Result<impl IntoResponse, Response> {
    if data.password != "6969" {
        return Err("Incorrect password".into_response());
    }
    let file: FormFile = FormFile::from_with_copy(&data.image);
    let file_name = format!("{}.{}", Utc::now(), file.name.extension().unwrap_or("none"));
    let save_path: PathBuf = ["../storage/uploads", &file_name].iter().collect();
    file.save_as(&save_path);
    set_background(&save_path, BgOpt::Stretch)
        .map_err(|_| "Could not set image as background..".into_err_response())?;
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
    Command::new("bash").arg("-c")
        .arg("rm ../storage/uploads/CURRENT_BACKGROUND*")
        .status()?;
    let dummy = image_path.rename_without_ext("CURRENT_BACKGROUND");
    let _ = fs::copy(image_path, dummy);
    Ok(())
}

pub async fn reload_image_dir() -> &'static str {
    use crate::image_dir;
    image_dir::generate_image_dir();
    "reloaded"
}

#[derive(Deserialize)]
pub struct PathParams {
    username: database::Username,
}

pub async fn user_profile(
    Path(PathParams { username }): Path<PathParams>
) -> String {
    if database::user_exists(username.clone()) {
        format!("User {username} didn't add anything to their profile.")
    } else {
        format!("User {username} doesn't exist.")
    }
}

pub async fn get_current_background(req: Request<Body>) -> Response {
    let headers = req.headers();
    if !headers.contains_key("Token") {
        return (StatusCode::BAD_REQUEST, "").into_response();
    }
    if headers["Token"] != "*secret_token*" {
        return (StatusCode::UNAUTHORIZED, "").into_response();
    }
    let mut current_background = None;
    match fs::read_dir("../storage/uploads") {
        Err(_err) => panic!("can't read dir"),
        Ok(paths) => for path in paths {
            let path = path.unwrap().path();
            if path.to_str().unwrap().contains("CURRENT_BACKGROUND") {
                current_background = Some(path);
            }
        }
    }
    if current_background.is_none() {
        return (StatusCode::INTERNAL_SERVER_ERROR, "").into_response();
    }
    let current_background = current_background.unwrap();
    let file = match tokio::fs::File::open(current_background.clone()).await {
        Ok(file) => file,
        Err(_err) => return (StatusCode::INTERNAL_SERVER_ERROR, "").into_response(),
    };
    let content_type = match mime_guess::from_path(&current_background).first_raw() {
        Some(mime) => mime,
        None => return (StatusCode::BAD_REQUEST, "MIME Type couldn't be determined".to_string()).into_response()
    };
    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);
    let headers = [
        (header::CONTENT_TYPE, content_type),
        (
            header::CONTENT_DISPOSITION,
            &format!(r#"attachment; filename={:?}"#, current_background),
        )
    ];
    (headers, body).into_response()
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
