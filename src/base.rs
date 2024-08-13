use tokio::fs::File;
use tokio::io::AsyncReadExt;
use crate::models::{get_nowtime_str, MyError};

pub async fn file_openString(name_file:&str) -> Result<String, MyError>{
    let mut file = File::open(name_file).await.map_err(|e|{
        let str_error = format!("FILE OPEN|| {} error: {}\n", get_nowtime_str(), e.to_string());
        MyError::DatabaseError(str_error)
    })?;

    let mut contents = String::new();
    file.read_to_string(&mut contents).await.map_err(|e|{
        let str_error = format!("FILE OPEN|| {} error: {}\n", get_nowtime_str(), e.to_string());
        MyError::DatabaseError(str_error)
    })?;
    Ok(contents)
}