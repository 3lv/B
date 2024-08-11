use std::path::PathBuf;
use dotenvy::dotenv;
use std::env;

pub mod db {
    const URL: &str = "postgres://localhost/diesel_db";
}

pub struct Dir {
    pub storage: PathBuf,
    pub public: PathBuf,
    pub images: PathBuf,
    pub uploads: PathBuf,
}

impl Dir {
    pub fn new() -> Self {
        dotenv().ok();
        #[allow(non_snake_case)]
        let storage: PathBuf = env::var("B_STORAGE_DIR").expect("B_STORAGE_DIR must be set").into();
        let public = storage.join("public");
        let images = public.join("images");
        let uploads = storage.join("uploads");
        Dir {
            storage,
            public,
            images,
            uploads,
        }
    }
}


/*
pub mod dir {
    const STORAGE: &str = "/home/vlad/workspace/rust/B/storage";

    use const_format::formatcp;

    pub const PUBLIC: &str = formatcp!("{STORAGE}/{}", "public");
    pub const IMAGES: &str = formatcp!("{PUBLIC}/{}", "images");

    pub const UPLOADS: &str = formatcp!("{STORAGE}/{}", "uploads");
}
*/
