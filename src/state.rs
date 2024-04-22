use crate::kafka;
use rdkafka::producer::FutureProducer;
use shuttle_runtime::DeploymentMetadata;
use shuttle_runtime::SecretStore;
use shuttle_service::Environment;

#[derive(Clone)]
pub struct AppState {
    kafka_producer: FutureProducer,
}

impl AppState {
    pub fn new(secrets: &SecretStore, metadata: &DeploymentMetadata) -> Self {
        let kafka_producer = match metadata.env {
            Environment::Local => kafka::create_kafka_producer(secrets),
            Environment::Deployment => kafka::create_kafka_producer_upstash(secrets),
        };

        Self { kafka_producer }
    }
}

impl<'a> AppState {
    pub fn producer(&'a self) -> &'a FutureProducer {
        &self.kafka_producer
    }
}
