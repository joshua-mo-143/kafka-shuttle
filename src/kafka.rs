use crate::queries;
use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
use rdkafka::consumer::{CommitMode, Consumer, StreamConsumer};
use rdkafka::message::Message;
use rdkafka::producer::FutureProducer;
use serde::{Deserialize, Serialize};
use shuttle_runtime::SecretStore;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomMessage {
    name: String,
    message: String,
}

impl<'a> CustomMessage {
    pub fn name(&'a self) -> &'a str {
        &self.name
    }

    pub fn message(&'a self) -> &'a str {
        &self.message
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KafkaMessage {
    action: Action,
    message_id: i32,
    data: Option<CustomMessage>,
}

impl<'a> KafkaMessage {
    pub fn data(&'a self) -> &'a CustomMessage {
        self.data.as_ref().unwrap()
    }

    pub fn message_id(&'a self) -> &'a i32 {
        &self.message_id
    }
}

#[derive(Debug, Serialize, Deserialize)]
enum Action {
    Create,
    Update,
    Delete,
}

pub fn create_kafka_producer_upstash(secrets: &SecretStore) -> FutureProducer {
    let url = secrets.get("KAFKA_URL").unwrap();
    let user = secrets.get("KAFKA_SASL_USER").unwrap();
    let pw = secrets.get("KAFKA_SASL_PASS").unwrap();

    ClientConfig::new()
        .set("bootstrap.servers", url)
        .set("sasl.mechanism", "SCRAM-SHA-256")
        .set("security.protocol", "SASL_SSL")
        .set("sasl.username", user)
        .set("sasl.password", pw)
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error")
}

pub fn create_kafka_consumer_upstash(secrets: &SecretStore) -> StreamConsumer {
    let url = secrets.get("KAFKA_URL").unwrap();
    let user = secrets.get("KAFKA_SASL_USER").unwrap();
    let pw = secrets.get("KAFKA_SASL_PASS").unwrap();

    ClientConfig::new()
        .set("bootstrap.servers", url)
        .set("sasl.mechanism", "SCRAM-SHA-256")
        .set("security.protocol", "SASL_SSL")
        .set("sasl.username", user)
        .set("sasl.password", pw)
        .set("group.id", "hello")
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .set_log_level(RDKafkaLogLevel::Debug)
        .create()
        .expect("Consumer creation failed")
}

pub fn create_kafka_producer(secrets: &SecretStore) -> FutureProducer {
    let url = secrets.get("KAFKA_URL").unwrap();

    let log_level: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", url)
        .set("message.timeout.ms", "5000")
        .set("queue.buffering.max.ms", "0") // Do not buffer
        .set("allow.auto.create.topics", "true")
        .create()
        .expect("Producer creation error");

    log_level
}

pub fn create_kafka_consumer(secrets: &SecretStore) -> StreamConsumer {
    let url = secrets.get("KAFKA_URL").unwrap();

    ClientConfig::new()
        .set("group.id", "test")
        .set("bootstrap.servers", url)
        .set("enable.partition.eof", "false")
        .set("allow.auto.create.topics", "true")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .set("enable.auto.offset.store", "false")
        .set_log_level(RDKafkaLogLevel::Debug)
        .create()
        .expect("Consumer creation failed")
}

#[tracing::instrument(skip(con))]
pub async fn kafka_consumer_task(con: StreamConsumer, db: sqlx::PgPool) {
    con.subscribe(&["messages"])
        .expect("Failed to subscribe to topics");

    tracing::warn!("Starting the consumer loop...");

    loop {
        match con.recv().await {
            Err(e) => tracing::warn!("Kafka error: {}", e),
            Ok(m) => {
                let Some(payload) = m.payload() else {
                    tracing::error!("Could not find a payload :(");
                    continue;
                };

                let message: KafkaMessage = match serde_json::from_slice(payload) {
                    Ok(res) => res,
                    Err(e) => {
                        tracing::error!("Deserialization error: {e}");
                        continue;
                    }
                };

                tracing::info!("Got payload: {message:?}");
                match message.action {
                    Action::Create => queries::create_message(message, &db).await,
                    Action::Update => queries::update_message(message, &db).await,
                    Action::Delete => queries::delete_message(message, &db).await,
                }

                let _ = con
                    .store_offset_from_message(&m)
                    .inspect_err(|e| tracing::warn!("Error while storing offset: {}", e));
            }
        };
    }
}
