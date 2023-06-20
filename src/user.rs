#[derive(sqlx::FromRow, Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub age: i32,
}

#[derive(sqlx::FromRow, Debug)]
pub struct Food {
    pub id: i32,
    pub name: String,
}

#[derive(sqlx::FromRow, Debug)]
pub struct UserWithFood {
    pub user_id: i32,
    pub user_name: String,
    pub food_name: String,
}
pub mod db;