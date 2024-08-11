use std::path::{Path, PathBuf};
use std::ffi::OsStr;
use axum_typed_multipart::FieldData;

use axum::{
    extract::multipart::{Field, MultipartError},
    body::Bytes,
};

use chrono::Utc;

pub trait FileRename {
    fn rename_without_ext(&self, new_name: &str) -> PathBuf;
}

impl<P> FileRename for P
where
    P: AsRef<Path>
{
    fn rename_without_ext(&self, new_name: &str) -> PathBuf {
        let path: &Path = self.as_ref();
        let mut result = path.to_owned();
        result.set_file_name(new_name);
        if let Some(ext) = path.extension() {
            result.set_extension(ext);
        }
        result
    }
}

fn change_file_name(path: impl AsRef<Path>, name: &str) -> PathBuf {
    let path = path.as_ref();
    let mut result = path.to_owned();
    result.set_file_name(name);
    if let Some(ext) = path.extension() {
        result.set_extension(ext);
    }
    result
}

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

impl<P> FileExtension for P
where
    P: AsRef<Path>
{
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
    pub async fn async_from_field(field: Field<'_>) -> Result<FormFile, MultipartError> {
            Ok(FormFile {
                name: field.file_name().unwrap_or_default().to_owned(),
                content_type: field.content_type().unwrap_or_default().to_owned(),
                bytes: field.bytes().await?
            })
    }
    pub fn from_with_copy(field_data: &FieldData<Bytes>) -> Self {
        FormFile {
            name: field_data.metadata.file_name.clone().unwrap_or_default(),
            content_type: field_data.metadata.content_type.clone().unwrap_or_default(),
            bytes: field_data.contents.clone(),
        }
    }
    pub fn save_as(&self, save_path: &PathBuf) -> Result<(), std::io::Error> {
        Ok(std::fs::write(&save_path, &self.bytes)?)
    }
    fn current_date_name(&self) -> PathBuf {
        let file_name = format!("{}.{}", Utc::now(), self.name.extension().unwrap_or("none"));
        PathBuf::from(file_name)
    }
    pub fn save_as_date(&self, dir: &PathBuf) -> Result<(), std::io::Error> {
        self.save_as(&dir.join(self.current_date_name()))?;
        Ok(())
    }
}
