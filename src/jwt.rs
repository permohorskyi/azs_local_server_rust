use chrono::Utc;
use serde::{Deserialize, Serialize};
use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, Header, Validation};
use crate::models::MysqlInfo;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id_user: i32,
    pub admin: bool,
    pub exp: usize,
    pub mysql_connect:MysqlInfo
}
pub fn create_token(id_user_:i32,admin_:bool,mysql_conn:MysqlInfo)->String{
    let my_claims = Claims {
        id_user:id_user_,
        admin:admin_,
        exp:10000000000,
        mysql_connect:mysql_conn
    };
    let token = encode(&Header::default(), &my_claims, &EncodingKey::from_secret("secret".as_ref()))
        .unwrap();
    token
}

pub fn validate_token(token:String, is_admin: &mut bool,mysql_info: MysqlInfo) -> bool {

    let decoding_key = DecodingKey::from_secret("secret".as_ref());
    let validation = Validation::default();
    *is_admin=false;
    match decode::<Claims>(token.as_str(), &decoding_key, &validation) {
        Ok(data) => {
            if data.claims.exp > Utc::now().timestamp() as usize && data.claims.mysql_connect==mysql_info {
                *is_admin=data.claims.admin;
                true
            } else {
                false
            }

        },
        Err(err) => {
            false
        }
    }
}