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
use mango3_core::{text_icon, CoreContext};

#[derive(Deserialize)]
pub struct BlobQueryParams {
    pub width: Option<u16>,
    pub height: Option<u16>,
    pub fill: Option<bool>,
}

#[derive(Deserialize)]
pub struct TextIconQueryParams {
    pub size: Option<u16>,
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

async fn get_text_icon(Path(text): Path<String>, Query(params): Query<TextIconQueryParams>) -> impl IntoResponse {
    let size = params.size.unwrap_or(32);

    if text.len() > 2 || text.contains(|c: char| !c.is_ascii_alphanumeric()) || size > 512 {
        return Err((StatusCode::BAD_REQUEST, "BAD REQUEST"));
    }

    let image = text_icon(text.to_uppercase(), size).map_err(|_| (StatusCode::NOT_FOUND, "FILE NOT FOUND"))?;

    let content_length = image.len();
    let body = Body::from(image);

    let headers = [
        (CONTENT_TYPE, "image/png".to_owned()),
        (CONTENT_LENGTH, content_length.to_string()),
        (
            CONTENT_DISPOSITION,
            format!("inline; filename=\"text-icon-{}-{}x{}.png\"", text, size, size),
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
        .route("/text-icons/:text", get(get_text_icon))
        .with_state(core_context);

    let listener = TcpListener::bind("127.0.0.1:3050").await.unwrap();

    axum::serve(listener, app.into_make_service()).await.unwrap();
}
