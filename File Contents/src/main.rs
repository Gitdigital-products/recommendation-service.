use axum::{
    extract::{Json, State},
    routing::post,
    Router,
};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct RecommendationRequest {
    user_id: String,
    context: String, // e.g. "shopping", "articles", "videos"
}

#[derive(Debug, Serialize, Clone)]
struct Recommendation {
    id: String,
    user_id: String,
    context: String,
    suggestion: String,
}

type RecStore = Arc<Mutex<Vec<Recommendation>>>;

#[tokio::main]
async fn main() {
    let store: RecStore = Arc::new(Mutex::new(Vec::new()));

    let app = Router::new()
        .route("/recommend", post(make_recommendation))
        .with_state(store.clone());

    let addr = SocketAddr::from(([127, 0, 0, 1], 4701));
    println!("🤖 Recommendation Service running at http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_state(store))
        .await
        .unwrap();
}

async fn make_recommendation(
    State(store): State<RecStore>,
    Json(req): Json<RecommendationRequest>,
) -> Json<Recommendation> {
    let pool = match req.context.as_str() {
        "shopping" => vec!["Wireless Headphones", "Smartwatch", "Gaming Mouse", "E-Book Reader"],
        "articles" => vec!["Rust Async Guide", "Microservices with Axum", "AI Ethics in 2025"],
        "videos" => vec!["Rust Crash Course", "Scaling Web Services", "Intro to AI Agents"],
        _ => vec!["No specific recommendations"],
    };

    let suggestion = pool.choose(&mut rand::thread_rng()).unwrap_or(&"Try something new");

    let rec = Recommendation {
        id: Uuid::new_v4().to_string(),
        user_id: req.user_id,
        context: req.context,
        suggestion: suggestion.to_string(),
    };

    let mut store = store.lock().await;
    store.push(rec.clone());

    Json(rec)
}
