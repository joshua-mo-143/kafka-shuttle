use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("RDKafka error: {0}")]
    RDKafka(#[from] rdkafka::error::RDKafkaError),
    #[error("Kafka error: {0}")]
    Kafka(rdkafka::error::KafkaError),
    #[error("De/serialization error: {0}")]
    SerdeJson(#[from] serde_json::Error),
    #[error("Oneshot message was canceled")]
    CanceledMessage(#[from] futures::channel::oneshot::Canceled),
}

impl<'a>
    From<(
        rdkafka::error::KafkaError,
        rdkafka::producer::FutureRecord<'a, str, std::vec::Vec<u8>>,
    )> for ApiError
{
    fn from(
        e: (
            rdkafka::error::KafkaError,
            rdkafka::producer::FutureRecord<'a, str, std::vec::Vec<u8>>,
        ),
    ) -> Self {
        Self::Kafka(e.0)
    }
}

impl From<(rdkafka::error::KafkaError, rdkafka::message::OwnedMessage)> for ApiError {
    fn from(e: (rdkafka::error::KafkaError, rdkafka::message::OwnedMessage)) -> Self {
        Self::Kafka(e.0)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, body) = match self {
            Self::Kafka(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            Self::RDKafka(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            Self::SerdeJson(e) => (StatusCode::BAD_REQUEST, e.to_string()),
            Self::CanceledMessage(e) => (StatusCode::BAD_REQUEST, e.to_string()),
        };

        (status, body).into_response()
    }
}
