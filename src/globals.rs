use lazy_static::lazy_static;
use tokio::sync::Mutex;
lazy_static! {
    pub static ref LOGS_DB_ERROR: Mutex<String> = Mutex::new(String::new());
}