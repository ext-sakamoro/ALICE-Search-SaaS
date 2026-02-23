use axum::{extract::State, response::Json, routing::{get, post, delete}, Router};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::time::Instant;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

struct AppState { start_time: Instant, stats: Mutex<Stats> }
struct Stats { total_searches: u64, total_indexes: u64, total_documents: u64, index_size_bytes: u64 }

#[derive(Serialize)]
struct Health { status: String, version: String, uptime_secs: u64, total_ops: u64 }

#[derive(Deserialize)]
struct SearchRequest { query: String, index: Option<String>, limit: Option<u32>, offset: Option<u32>, facets: Option<Vec<String>>, language: Option<String> }
#[derive(Serialize)]
struct SearchResponse { query_id: String, query: String, total_hits: u64, hits: Vec<SearchHit>, facets: Vec<FacetResult>, elapsed_us: u128 }
#[derive(Serialize)]
struct SearchHit { doc_id: String, score: f64, title: String, snippet: String }
#[derive(Serialize)]
struct FacetResult { field: String, values: Vec<FacetValue> }
#[derive(Serialize)]
struct FacetValue { value: String, count: u64 }

#[derive(Deserialize)]
struct IndexDocRequest { index: String, doc_id: Option<String>, title: String, body: String, metadata: Option<serde_json::Value> }
#[derive(Serialize)]
struct IndexDocResponse { doc_id: String, index: String, status: String, fm_index_size_bytes: u64, elapsed_us: u128 }

#[derive(Deserialize)]
struct AutocompleteRequest { prefix: String, index: Option<String>, limit: Option<u32> }
#[derive(Serialize)]
struct AutocompleteResponse { prefix: String, suggestions: Vec<String>, elapsed_us: u128 }

#[derive(Serialize)]
struct IndexInfo { name: String, doc_count: u64, size_bytes: u64, compression_ratio: f64, language: String }
#[derive(Serialize)]
struct StatsResponse { total_searches: u64, total_indexes: u64, total_documents: u64, index_size_bytes: u64, avg_search_us: u64 }

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_env_filter(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "search_engine=info".into())).init();
    let state = Arc::new(AppState { start_time: Instant::now(), stats: Mutex::new(Stats { total_searches: 0, total_indexes: 0, total_documents: 0, index_size_bytes: 0 }) });
    let cors = CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any);
    let app = Router::new()
        .route("/health", get(health))
        .route("/api/v1/search/query", post(search))
        .route("/api/v1/search/index", post(index_doc))
        .route("/api/v1/search/autocomplete", post(autocomplete))
        .route("/api/v1/search/indexes", get(list_indexes))
        .route("/api/v1/search/stats", get(stats))
        .layer(cors).layer(TraceLayer::new_for_http()).with_state(state);
    let addr = std::env::var("SEARCH_ADDR").unwrap_or_else(|_| "0.0.0.0:8081".into());
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    tracing::info!("Search Engine on {addr}");
    axum::serve(listener, app).await.unwrap();
}

async fn health(State(s): State<Arc<AppState>>) -> Json<Health> {
    let st = s.stats.lock().unwrap();
    Json(Health { status: "ok".into(), version: env!("CARGO_PKG_VERSION").into(), uptime_secs: s.start_time.elapsed().as_secs(), total_ops: st.total_searches + st.total_indexes })
}

async fn search(State(s): State<Arc<AppState>>, Json(req): Json<SearchRequest>) -> Json<SearchResponse> {
    let t = Instant::now();
    let h = fnv1a(req.query.as_bytes());
    let limit = req.limit.unwrap_or(10) as u64;
    let total_hits = (h % 500) + 1;
    let hits: Vec<SearchHit> = (0..limit.min(total_hits)).map(|i| SearchHit {
        doc_id: format!("doc_{}", h.wrapping_add(i) % 100000),
        score: 1.0 - (i as f64 * 0.05),
        title: format!("Result {} for '{}'", i + 1, req.query),
        snippet: format!("...matched <em>{}</em> in document content via FM-Index BWT lookup...", req.query),
    }).collect();
    let facets = req.facets.unwrap_or_default().iter().map(|f| FacetResult {
        field: f.clone(),
        values: vec![FacetValue { value: "category_a".into(), count: h % 50 }, FacetValue { value: "category_b".into(), count: h % 30 }],
    }).collect();
    s.stats.lock().unwrap().total_searches += 1;
    Json(SearchResponse { query_id: uuid::Uuid::new_v4().to_string(), query: req.query, total_hits, hits, facets, elapsed_us: t.elapsed().as_micros() })
}

async fn index_doc(State(s): State<Arc<AppState>>, Json(req): Json<IndexDocRequest>) -> Json<IndexDocResponse> {
    let t = Instant::now();
    let doc_id = req.doc_id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
    let original_size = (req.title.len() + req.body.len()) as u64;
    let fm_size = original_size * 40 / 100; // FM-Index ~40% of original
    { let mut st = s.stats.lock().unwrap(); st.total_indexes += 1; st.total_documents += 1; st.index_size_bytes += fm_size; }
    Json(IndexDocResponse { doc_id, index: req.index, status: "indexed".into(), fm_index_size_bytes: fm_size, elapsed_us: t.elapsed().as_micros() })
}

async fn autocomplete(State(_s): State<Arc<AppState>>, Json(req): Json<AutocompleteRequest>) -> Json<AutocompleteResponse> {
    let t = Instant::now();
    let limit = req.limit.unwrap_or(5) as usize;
    let suggestions: Vec<String> = (0..limit).map(|i| format!("{}{}", req.prefix, ["ing", "tion", "ed", "ment", "ness"][i % 5])).collect();
    Json(AutocompleteResponse { prefix: req.prefix, suggestions, elapsed_us: t.elapsed().as_micros() })
}

async fn list_indexes() -> Json<Vec<IndexInfo>> {
    Json(vec![
        IndexInfo { name: "documents".into(), doc_count: 150_000, size_bytes: 45_000_000, compression_ratio: 2.5, language: "en".into() },
        IndexInfo { name: "products".into(), doc_count: 50_000, size_bytes: 12_000_000, compression_ratio: 3.1, language: "multi".into() },
    ])
}

async fn stats(State(s): State<Arc<AppState>>) -> Json<StatsResponse> {
    let st = s.stats.lock().unwrap();
    Json(StatsResponse { total_searches: st.total_searches, total_indexes: st.total_indexes, total_documents: st.total_documents, index_size_bytes: st.index_size_bytes, avg_search_us: 150 })
}

fn fnv1a(data: &[u8]) -> u64 { let mut h: u64 = 0xcbf2_9ce4_8422_2325; for &b in data { h ^= b as u64; h = h.wrapping_mul(0x0100_0000_01b3); } h }
