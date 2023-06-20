use anyhow::{Result};
use sqlx::postgres::PgPoolOptions;
use user::{User, Food, db};

#[tokio::main]
async fn main() -> Result<()>{
    let pool = PgPoolOptions::new() 
        .max_connections(5)
        .connect("postgres://postgres:123456@localhost/sqlx-app").await?;
    
    let mut conn = pool.acquire().await?;

    let user_id = 1;
    let specific_user = db::query_user(&mut conn, user_id).await?;
    if let Some(user) = specific_user  {
        println!("user fetched: {:?}", user)
    } else {
        println!("not existed user with id {}", user_id)
    }

    let users = db::query_all_uers(&mut conn).await?;
    println!("got users: {:?}", users);

    let youngest = db::query_youngest_user(&mut conn).await?;
    println!("got youngest user: {:?}", youngest);

    let fav_foods = db::query_user_fav_foods(&mut conn, user_id).await?;
    println!("user favrite foods: {:?}", fav_foods);

    let food = Food{id: 5, name: String::from("my name is delicious")};
    db::user_faved_new_food(&mut conn, 5, user_id, &food).await?;

    let users = vec![
        User{id: 3, name: String::from("User 3"), age: 18},
        User{id: 4, name: String::from("User 4"), age: 18}
    ];
    db::user_bulk_insert(&mut conn, &users).await?;
    Ok(())
}

mod user;
