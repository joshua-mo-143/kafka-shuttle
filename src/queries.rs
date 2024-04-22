use crate::kafka::KafkaMessage;

#[tracing::instrument]
pub async fn create_message(message: KafkaMessage, db: &sqlx::PgPool) {
    if let Err(e) = sqlx::query(
        "INSERT INTO MESSAGES
                           (message_id, name, message)
                            VALUES
                            ($1, $2, $3)
                            ON CONFLICT (message_id) DO NOTHING",
    )
    .bind(message.message_id())
    .bind(message.data().name())
    .bind(message.data().message())
    .execute(db)
    .await
    {
        tracing::error!("Error while inserting message: {e}")
    }
}

#[tracing::instrument]
pub async fn update_message(message: KafkaMessage, db: &sqlx::PgPool) {
    if let Err(e) = sqlx::query(
        "UPDATE MESSAGES
                            SET
                            name = $1,
                            message = $2
                            where message_id = $3",
    )
    .bind(message.data().name())
    .bind(message.data().message())
    .bind(message.message_id())
    .execute(db)
    .await
    {
        tracing::error!("Error while updating message: {e}")
    }
}

#[tracing::instrument]
pub async fn delete_message(message: KafkaMessage, db: &sqlx::PgPool) {
    if let Err(e) = sqlx::query("DELETE from messages where message_id = $1")
        .bind(message.message_id())
        .execute(db)
        .await
    {
        tracing::error!("Error while deleting messages: {e}")
    }
}
