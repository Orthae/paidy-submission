use axum::body::Body;
use axum::http::{HeaderName, Request};
use tower::layer::util::{Identity, Stack};
use tower::ServiceBuilder;
use tower_http::request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer};
use tower_http::trace::{HttpMakeClassifier, TraceLayer};
use tracing::{info_span, warn, Span};
use tracing_subscriber::EnvFilter;
use uuid::Uuid;

const REQUEST_ID_HEADER: HeaderName = HeaderName::from_static("x-request-id");

pub type RequestIdMiddlewareLayer = ServiceBuilder<
    Stack<PropagateRequestIdLayer, Stack<SetRequestIdLayer<MakeRequestUuid>, Identity>>,
>;

pub struct RequestIdMiddleware;

impl RequestIdMiddleware {
    pub fn create() -> RequestIdMiddlewareLayer {
        ServiceBuilder::new()
            .layer(SetRequestIdLayer::new(REQUEST_ID_HEADER, MakeRequestUuid))
            .layer(PropagateRequestIdLayer::new(REQUEST_ID_HEADER))
    }
}

pub type TraceMiddlewareLayer =
    ServiceBuilder<Stack<TraceLayer<HttpMakeClassifier, fn(&Request<Body>) -> Span>, Identity>>;

pub struct TraceMiddleware;

impl TraceMiddleware {
    pub fn create() -> TraceMiddlewareLayer {
        ServiceBuilder::new().layer(TraceLayer::new_for_http().make_span_with(trace_handler))
    }

    pub fn filter() -> EnvFilter {
        EnvFilter::try_from_default_env().unwrap_or(EnvFilter::from(format!(
            "{}=info",
            env!("CARGO_CRATE_NAME")
        )))
    }
}

fn trace_handler(request: &Request<Body>) -> Span {
    match request.headers().get(REQUEST_ID_HEADER) {
        Some(request_id) => {
            info_span!("HTTP", "{:#?}", request_id)
        }
        None => {
            let request_id = Uuid::new_v4();
            warn!(
                "Request ID not found in headers, generating new one: {:?}",
                request_id
            );
            info_span!("HTTP", "{:#?}", request_id)
        }
    }
}
