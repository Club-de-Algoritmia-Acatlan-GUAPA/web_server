use crate::database::schema::users;

use rocket::serde::{json, Deserialize, Serialize};
use diesel::{prelude::*, AsExpression, FromSqlRow};
use rocket_sync_db_pools::diesel;
// `diesel migration redo` and `diesel setup` to generate code


#[derive(Debug, Clone, Copy, PartialEq, Eq, AsExpression, FromSqlRow, Deserialize, Serialize)]
#[diesel(sql_type = diesel::sql_types::Integer)]
pub enum Roles {
    Admin,
    User,
    ProblemSetter,
}

// muy importante mantener el orden de los fields con respecto a 
// la tabla
#[derive(Queryable, Serialize, Deserialize, Debug, Insertable, Clone)]
#[diesel(table_name = users)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub is_validated: bool,
    pub creation_date: i32,
    pub username: String,
    pub user_role: i32,
}

#[derive(Insertable, Serialize, Deserialize, Debug,Clone, Queryable)]
#[diesel(table_name = users)]
#[serde(crate = "rocket::serde")]
pub struct NewUser<'a> {
    pub password: &'a str,
    pub email: &'a str,
    pub creation_date: i32,
    pub username: &'a str,
    pub user_role: i32,
}
