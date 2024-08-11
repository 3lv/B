use serde::{Serialize, Deserialize};

use axum::{
    response::{Response, IntoResponse},
    http::StatusCode
};

use diesel::{
    sql_types::*,
    serialize::{self, ToSql, Output},
    deserialize::{self, FromSql},
    backend::Backend,
};

use std::fmt;

#[derive(Debug, thiserror::Error)]
pub enum PasswordError {
    #[error("is too short")]
    TooShort(),
    #[error("contains invalid characters (can't contain {0})")]
    InvalidChars(String),
    #[error("is too weak")]
    Weak()
}

impl IntoResponse for PasswordError {
    fn into_response(self) -> Response {
        (StatusCode::BAD_REQUEST, self.to_string()).into_response()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum UsernameError {
    #[error("is already in use")]
    InUse(),
    #[error("contains invalid characters (can't contain {0})")]
    InvalidChars(String),
}

impl IntoResponse for UsernameError {
    fn into_response(self) -> Response {
        (StatusCode::BAD_REQUEST, self.to_string()).into_response()
    }
}

/*
// Causes problems with recursion? (It should be instead of impl ToSql
impl Into<String> for Password {
    fn into(self) -> String {
        self.0
    }
}
*/

macro_rules! impl_tosql_fromsql {
    ($ty:path, $sql_type:ty) => {
        impl<Db: Backend> ToSql<$sql_type, Db> for $ty
        where
            String: ToSql<$sql_type, Db>
        {
            fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Db>) -> serialize::Result {
                self.0.to_sql(out)
            }
        }
        impl<Db: Backend> FromSql<$sql_type, Db> for $ty
        where
            String: FromSql<$sql_type, Db>
        {
            fn from_sql(bytes: Db::RawValue<'_>) -> deserialize::Result<Self> {
                String::from_sql(bytes)
                    .map(|s| $ty(s))
            }
        }
    };
    /*
    ($type:ty, $sql_type:ty | $($other_sql_types:ty)|+) => {
        impl_tosql!($type, $sql_type);
        impl_tosql!($type, $($other_sql_types)|+);
    }
    */
}

#[derive(Clone, Debug, Serialize, Deserialize, FromSqlRow, AsExpression)]
#[serde(try_from = "String")]
#[diesel(sql_type = Varchar)]
pub struct Password(String);
impl_tosql_fromsql!(Password, Varchar);

use std::collections::BTreeSet;

impl TryFrom<String> for Password {
    type Error = PasswordError;
    fn try_from(password: String) -> Result<Self, Self::Error> {
        if password.len() < 8 {
            return Err(Self::Error::TooShort());
        }
        let valid_chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()";
        let mut invalid_chars = BTreeSet::new();
        for ch in password.chars() {
            if !valid_chars.contains(ch) {
                invalid_chars.insert(ch.to_string());
            }
        }
        if !invalid_chars.is_empty() {
            return Err(Self::Error::InvalidChars(
                invalid_chars.into_iter()
                .take(5)
                .map(|c| format!("'{c}'"))
                .collect::<Vec<String>>().join(", ")
            ));
        }
        // TODO: Hash password
        Ok(Password(password))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, FromSqlRow, AsExpression)]
#[serde(try_from = "String")]
#[diesel(sql_type = Varchar)]
pub struct Username(String);
impl_tosql_fromsql!(Username, Varchar);

impl TryFrom<String> for Username {
    type Error = UsernameError;
    fn try_from(username: String) -> Result<Self, Self::Error> {
        let valid_chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        let mut invalid_chars = BTreeSet::new();
        for ch in username.chars() {
            if !valid_chars.contains(ch) {
                invalid_chars.insert(ch);
            }
        }
        if !invalid_chars.is_empty() {
            return Err(Self::Error::InvalidChars(
                invalid_chars.into_iter()
                .take(5)
                .map(|c| format!("'{c}'"))
                .collect::<Vec<String>>().join(", ")
            ));
        }
        // TODO: Hash password
        Ok(Username(username))
    }
}

impl fmt::Display for Username {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

/*
impl<'de> Deserialize<'de> for Password {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>
    {
        let proxy = PasswordProxy::deserialize(deserializer)?;
        if proxy.is_valid() {
        }
    }
}
*/
