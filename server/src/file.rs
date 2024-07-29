use std::path::Path;
use std::ffi::OsStr;

use axum::{
    extract::multipart::{Field, MultipartError},
    body::Bytes,
};

pub trait FileExtension {
    fn extension(&self) -> Option<&str>;
    fn has_extension<S: AsRef<str>>(&self, extensions: &[S]) -> bool {
        if let Some(ref extension) = self.extension() {
            return extensions
                .iter()
                .any(|x| x.as_ref().eq_ignore_ascii_case(extension));
        }
        false
    }
    fn is_image(&self) -> bool {
        self.has_extension(&["png", "jpg", "jpeg", "gif"])
    }
}

impl<P: AsRef<Path>> FileExtension for P {
    fn extension(&self) -> Option<&str> {
            self.as_ref()
            .extension()
            .and_then(OsStr::to_str)
    }
}

pub struct FormFile {
    pub name: String,
    #[allow(dead_code)]
    pub content_type: String,
    pub bytes: Bytes,
}

impl FormFile {
    pub async fn async_from(field: Field<'_>) -> Result<FormFile, MultipartError> {
            Ok(FormFile {
                name: field.file_name().unwrap_or_default().to_owned(),
                content_type: field.content_type().unwrap_or_default().to_owned(),
                bytes: field.bytes().await?
            })
    }
}
