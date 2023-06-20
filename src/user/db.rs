use crate::user::{User, UserWithFood, Food};
use anyhow::{*, Ok, Result};
use sqlx::{Acquire, QueryBuilder, Postgres};

// query user with id in argument
pub async fn query_user(executor: impl sqlx::PgExecutor<'_>, id: i32) -> Result<Option<User>> {
    let maybe_user = sqlx::query_as("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_optional(executor)
        .await?;
    Ok(maybe_user)
}

// query all users
pub async fn query_all_uers(executor: impl sqlx::PgExecutor<'_>) -> Result<Vec<User>> {
    let users = sqlx::query_as("select * from users")
        .fetch_all(executor)
        .await?;
    Ok(users)
}

// query youngest user
pub async fn query_youngest_user(executor: impl sqlx::PgExecutor<'_>) -> Result<User> {
    let user = sqlx::query_as("select * from users order by age asc limit 1")
        .fetch_one(executor)
        .await?;
    Ok(user)
}

// query favorites foods for user with user_id as argument 
pub async fn query_user_fav_foods(executor: impl sqlx::PgExecutor<'_>, user_id: i32) -> Result<Vec<UserWithFood>> {
    let user_fav_foods = sqlx::query_as(
        r#"SELECT
            u.id AS user_id,
            u."name" AS user_name,
            f."name" AS food_name
        FROM
            users AS u
            INNER JOIN user_favorite_foods AS uff ON u.id = uff.user_id
            INNER JOIN foods AS f ON uff.food_id = f.id
        WHERE u.id = $1"#
    )
        .bind(user_id)
        .fetch_all(executor)
        .await?;
    Ok(user_fav_foods)
}

// insert food record
pub async fn insert_food(executor: impl sqlx::PgExecutor<'_>, food: &Food) -> Result<()> {
    let result = sqlx::query(
        r#"INSERT INTO foods (id, name) VALUES ( $1, $2 )"#
    )
    .bind(food.id)
    .bind(food.name.as_str())
    .execute(executor)
    .await?;
    if result.rows_affected() == 0 {
        bail!("rows affected count is 0 when inserting food, {:?}", food);
    }
    Ok(())
}

// insert user fav food record
pub async fn insert_user_fav_food(executor: impl sqlx::PgExecutor<'_>, id: i32, user_id: i32, food_id: i32) -> Result<()> {
    let result = sqlx::query(
        r#"INSERT INTO user_favorite_foods (id, user_id, food_id) VALUES ($1, $2, $3)"#
    )
    .bind(id)
    .bind(user_id)
    .bind(food_id)
    .execute(executor)
    .await?;
    if result.rows_affected() == 0 {
        bail!("rows affected count is 0 when inserting fav food, {} {} {}", id, user_id, food_id);
    }
    Ok(())
}

// user just favorited a new food
// so we should be a transaction, in which insert both records, and commit
pub async fn user_faved_new_food(conn: &mut sqlx::PgConnection, id: i32, user_id: i32, food: &Food) -> Result<()> {
    let mut transction = conn.begin().await?;
    insert_food(&mut transction, food).await?;
    insert_user_fav_food(&mut transction, id, user_id, food.id).await?;
    transction.commit().await?;
    Ok(())
}

// play with `QueryBuilder`
pub async fn user_bulk_insert(executor: impl sqlx::PgExecutor<'_>, users: &[User]) -> Result<()> {
    let mut builder: QueryBuilder<Postgres> = QueryBuilder::new("INSERT INTO users(id, name, age)");
    builder.push(" ");
    builder.push_values(users, | mut b, user| {
        b.push_bind(user.id)
            .push_bind(user.name.clone())
            .push_bind(user.age);
    });
    let result = builder.build()
        .execute(executor).await?;
    if result.rows_affected() != 2 {
        bail!("rows affected count not as expected, expect: {}, get: {}", users.len(), result.rows_affected())
    }
    Ok(())
}
