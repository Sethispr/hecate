use axum::{extract::State, Json};
use reqwest::Client;
use serde::Serialize;
use serde_json::Value;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

use crate::health::{check_sumi, check_discord, check_top_gg};

#[derive(Clone, Serialize)]
pub struct Service {
    pub name: String,
    pub id: Option<String>,
    pub status: &'static str,
    pub color: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Clone, Serialize)]
pub struct StatusResponse {
    pub main_services: Vec<Service>,
    pub other_services: Vec<Service>,
}

pub struct AppState {
    pub client: Client,
    pub cache: RwLock<Option<(Instant, StatusResponse)>>,
}

impl AppState {
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(3))
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
            .build()
            .unwrap();
        Self {
            client,
            cache: RwLock::new(None),
        }
    }
}

pub async fn status_handler(State(state): State<Arc<AppState>>) -> Json<StatusResponse> {
    // 1. Try to read from cache (read lock)
    {
        let cache_read = state.cache.read().await;
        if let Some((fetched_at, response)) = &*cache_read {
            if fetched_at.elapsed() < Duration::from_secs(10) {
                return Json(response.clone());
            }
        }
    }

    // 2. Cache is expired or empty. We fetch outside any lock so we don't block other tasks
    // from reading stale data (if we implemented stale-while-revalidate) or hitting other endpoints.
    let (sumi, discord, top_gg) = tokio::join!(
        check_sumi(&state.client),
        check_discord(&state.client),
        check_top_gg(&state.client)
    );
    
    let sumi_status = if sumi.is_online { "online" } else { "offline" };
    let sumi_color = if sumi.is_online { "48c78e" } else { "f14668" };

    let discord_status = if discord.is_online { "online" } else { "offline" };
    let discord_color = if discord.is_online { "48c78e" } else { "f14668" };

    let topgg_status = if top_gg.is_online { "online" } else { "offline" };
    let topgg_color = if top_gg.is_online { "48c78e" } else { "f14668" };

    let fresh_response = StatusResponse {
        main_services: vec![
            Service {
                name: "shizuku".into(),
                id: None,
                status: "online",
                color: "48c78e",
                metrics: None,
                description: None,
            },
            Service {
                name: "sumi".into(),
                id: None,
                status: sumi_status,
                color: sumi_color,
                metrics: sumi.metrics,
                description: None,
            },
            Service {
                name: "blair".into(),
                id: None,
                status: "offline",
                color: "f14668",
                metrics: None,
                description: None,
            },
        ],
        other_services: vec![
            Service {
                name: "discord status".into(),
                id: Some("discord".into()),
                status: discord_status,
                color: discord_color,
                metrics: None,
                description: Some(discord.description),
            },
            Service {
                name: "top.gg status".into(),
                id: Some("topgg".into()),
                status: topgg_status,
                color: topgg_color,
                metrics: None,
                description: Some(top_gg.description),
            },
        ],
    };

    // 3. Acquire write lock and store
    {
        let mut cache_write = state.cache.write().await;
        // Check once more in case another request already updated it while we were fetching
        match &*cache_write {
            Some((fetched_at, _)) if fetched_at.elapsed() < Duration::from_secs(10) => {
                // Ignore our fetch, use the latest (or just write ours anyway)
            }
            _ => {
                *cache_write = Some((Instant::now(), fresh_response.clone()));
            }
        }
    }

    Json(fresh_response)
}
