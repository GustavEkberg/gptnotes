use std::str::from_utf8;
static DB: &str = "file://.db";

pub async fn get_api_key() -> Result<Option<String>, Box<dyn std::error::Error>> {
    let db = surrealdb::Datastore::new(DB).await?;

    let mut transaction = db.transaction(false, false).await?;
    let value = transaction.get("api_key").await?;
    if value.is_none() {
        Ok(None)
    } else {
        Ok(Some(from_utf8(&value.unwrap()).unwrap().to_string()))
    }
}

pub async fn set_api_key(api_key: String) -> Result<(), Box<dyn std::error::Error>> {
    let db = surrealdb::Datastore::new(DB).await?;

    let mut transaction = db.transaction(true, true).await?;
    transaction.set("api_key", api_key).await?;
    transaction.commit().await?;

    Ok(())
}

pub async fn get_notes_folder() -> Result<Option<String>, Box<dyn std::error::Error>> {
    let db = surrealdb::Datastore::new(DB).await?;

    let mut transaction = db.transaction(false, false).await?;
    let value = transaction.get("notes_folder").await?;
    if value.is_none() {
        Ok(None)
    } else {
        Ok(Some(from_utf8(&value.unwrap()).unwrap().to_string()))
    }
}

pub async fn set_notes_folder(api_key: String) -> Result<(), Box<dyn std::error::Error>> {
    let db = surrealdb::Datastore::new(DB).await?;

    let mut transaction = db.transaction(true, true).await?;
    transaction.set("notes_folder", api_key).await?;
    transaction.commit().await?;

    Ok(())
}
