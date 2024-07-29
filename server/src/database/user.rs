use serde::{Serialize, Deserialize};
use diesel::prelude::*;
//use diesel::dsl::sql_query;
use crate::database::establish_connection;

#[derive(Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub password: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub enabled: bool,
}

// TODO: Look into AsChangeset
#[derive(Debug, Queryable, Selectable, Serialize, Clone)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub enabled: bool,
}

pub fn create_user(user: CreateUser) -> Result<(), diesel::result::Error> {
    use crate::schema::users::dsl::*;
    let mut connection = establish_connection();
    let new_user = NewUser {
        username: user.username,
        password: user.password,
        enabled: false,
    };
    diesel::insert_into(users)
        .values(&new_user)
        .execute(&mut connection)?;
    Ok(())
}

pub fn get_users() -> Result<Vec<User>, diesel::result::Error> {
    use crate::schema::users::dsl::*;
    let mut connection = establish_connection();
    users
        .filter(enabled.eq(true))
        .order(id.asc())
        //.select(username)
        .load::<User>(&mut connection)
}
