use crate::models::item::{Item, NewItem};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn create_item(pool: &PgPool, new_item: NewItem) -> Result<Item, sqlx::Error> {
    let item = sqlx::query_as!(
        Item,
        r#"
        INSERT INTO r_items (name, description)
        VALUES ($1, $2)
        RETURNING id, name, description
        "#,
        new_item.name,
        new_item.description
    )
        .fetch_one(pool)
        .await?;

    Ok(item)
}

pub async fn get_item(pool: &PgPool, item_id: Uuid) -> Result<Item, sqlx::Error> {
    let item = sqlx::query_as!(
        Item,
        r#"
        SELECT id, name, description
        FROM r_items
        WHERE id = $1
        "#,
        item_id
    )
        .fetch_one(pool)
        .await?;

    Ok(item)
}

pub async fn get_all_items(pool: &PgPool) -> Result<Vec<Item>, sqlx::Error> {
    let items = sqlx::query_as!(
        Item,
        r#"
        SELECT id, name, description
        FROM r_items
        "#,
    )
        .fetch_all(pool)
        .await?;

    Ok(items)
}

pub async fn update_item(pool: &PgPool, item_id: Uuid, updated_item: NewItem) -> Result<Item, sqlx::Error> {
    let item = sqlx::query_as!(
        Item,
        r#"
        UPDATE r_items
        SET name = $1, description = $2
        WHERE id = $3
        RETURNING id, name, description
        "#,
        updated_item.name,
        updated_item.description,
        item_id
    )
        .fetch_one(pool)
        .await?;

    Ok(item)
}

pub async fn delete_item(pool: &PgPool, item_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        DELETE FROM r_items
        WHERE id = $1
        "#,
        item_id
    )
        .execute(pool)
        .await?;

    Ok(())
}
