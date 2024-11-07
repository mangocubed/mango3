use axum::body::Body;
use axum::extract::{Path, Query, State};
use axum::http::header::{CONTENT_DISPOSITION, CONTENT_LENGTH, CONTENT_TYPE};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use serde::Deserialize;
use tokio::net::TcpListener;
use uuid::Uuid;

use mango3_core::config::load_config;
use mango3_core::models::Blob;
use mango3_core::CoreContext;

#[derive(Deserialize)]
pub struct BlobQueryParams {
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub fill: Option<bool>,
}

async fn get_blob(
    State(core_context): State<CoreContext>,
    Path(id): Path<Uuid>,
    Query(params): Query<BlobQueryParams>,
) -> impl IntoResponse {
    let blob = Blob::get_by_id(&core_context, id, None)
        .await
        .map_err(|_| (StatusCode::NOT_FOUND, "FILE NOT FOUND"))?;

    let Some(file) = blob.read(params.width, params.height, params.fill) else {
        return Err((StatusCode::FORBIDDEN, "FORBIDDEN"));
    };

    let content_length = file.len();
    let body = Body::from(file);

    let headers = [
        (CONTENT_TYPE, blob.content_type.clone()),
        (CONTENT_LENGTH, content_length.to_string()),
        (
            CONTENT_DISPOSITION,
            format!(
                "inline; filename=\"{}\"",
                blob.variant_filename(params.width, params.height, params.fill)
            ),
        ),
    ];

    Ok((headers, body))
}

#[tokio::main]
async fn main() {
    load_config();

    let core_context = CoreContext::setup().await;

    let app = Router::new()
        .route("/blobs/:id", get(get_blob))
        .with_state(core_context);

    let listener = TcpListener::bind("127.0.0.1:3050").await.unwrap();

    axum::serve(listener, app.into_make_service()).await.unwrap();
}
