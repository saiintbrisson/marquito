use std::ffi::OsStr;

pub fn create_url(url: impl AsRef<OsStr>) -> String {
    let url = url.as_ref().to_str().unwrap();
    format!("http://{}/{}", crate::SERVER_BASE_URL, url)
}
