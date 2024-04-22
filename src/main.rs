use shuttle_runtime::{DeploymentMetadata, SecretStore};
use shuttle_service::Environment;
use sqlx::PgPool;

mod errors;
mod kafka;
mod queries;
mod routing;
mod state;

use routing::init_router;
use state::AppState;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] db: PgPool,
    #[shuttle_runtime::Secrets] secrets: SecretStore,
    #[shuttle_runtime::Metadata] metadata: DeploymentMetadata,
) -> shuttle_axum::ShuttleAxum {
    sqlx::migrate!().run(&db).await.unwrap();

    let con = match metadata.env {
        Environment::Local => kafka::create_kafka_consumer(&secrets),
        Environment::Deployment => kafka::create_kafka_consumer_upstash(&secrets),
    };

    tokio::spawn(async move {
        kafka::kafka_consumer_task(con, db).await;
    });

    let state = AppState::new(&secrets, &metadata);

    let rtr = init_router(state);

    Ok(rtr.into())
}
