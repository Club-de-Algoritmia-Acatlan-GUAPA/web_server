use actix_web::{web,HttpResponse };
use sqlx::{
    postgres::PgPool, 
    Pool, 
    Postgres
};
use sqlx::types::chrono::TimeZone;
use anyhow::Result;

pub struct FormData {
    name : String,
    email : String,
    password: String,
}
pub async fn register(
    form: web::Form<FormData>,
    pool: web::Data<Pool<Postgres>>,
) -> Result<HttpResponse> {
    // sqlx::query!(
    //     r#"
    //     INSERT INTO users (
    //         email,
    //         password,
    //         is_validated,
    //         creation_date,
    //         username,
    //         user_role
    //     )
    //     VALUES (
    //         $1,
    //         $2,
    //         $3,
    //         $4,
    //         $5,
    //         $6
    //     )
    //     "#,
    //     form.0.email,
    //     form.0.password,
    //     false,
    //     123123,
    //     form.0.name,
    //     1
    // )
    // .execute(pool)
    // .await?;

    Ok(HttpResponse::Ok().finish())
}
