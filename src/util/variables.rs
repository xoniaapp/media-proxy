use std::env;

lazy_static! {
    pub static ref HOST: String =
        env::var("MEDIA_HOST").expect("Missing MEDIA_HOST environment variable.");
}