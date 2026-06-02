use reqwest::Client;
use serde_json::Value;

pub struct SumiHealth {
    pub is_online: bool,
    pub metrics: Option<Value>,
}

pub struct StatusPageHealth {
    pub is_online: bool,
    pub description: String,
}

pub async fn check_sumi(client: &Client) -> SumiHealth {
    let health_url = "http://127.0.0.1:8080/health";
    let is_online = client
        .get(health_url)
        .send()
        .await
        .map(|r| r.status().is_success())
        .unwrap_or(false);

    if !is_online {
        return SumiHealth {
            is_online: false,
            metrics: None,
        };
    }

    let metrics_url = "http://127.0.0.1:8080/metrics";
    let metrics = client
        .get(metrics_url)
        .send()
        .await
        .ok()
        .and_then(|r| r.json::<Value>().ok());

    SumiHealth { is_online, metrics }
}

pub async fn check_discord(client: &Client) -> StatusPageHealth {
    let url = "https://discordstatus.com/api/v2/summary.json";
    if let Ok(res) = client.get(url).send().await {
        if let Ok(data) = res.json::<Value>().await {
            let indicator = data["status"]["indicator"].as_str().unwrap_or("none");
            let desc = data["status"]["description"].as_str().unwrap_or("All Systems Operational").to_string();
            let is_online = indicator == "none" || indicator == "minor";
            return StatusPageHealth {
                is_online,
                description: desc,
            };
        }
    }
    StatusPageHealth {
        is_online: false,
        description: "Unable to reach Discord Status API".into(),
    }
}

pub async fn check_top_gg(client: &Client) -> StatusPageHealth {
    // Top.gg uses betterstack, so we can fetch the HTML of status.top.gg and look for keys
    let url = "https://status.top.gg/";
    if let Ok(res) = client.get(url).send().await {
        if let Ok(text) = res.text().await {
            let is_online = text.contains("All services are online") || text.contains("All systems operational");
            let desc = if is_online {
                "All services are online".to_string()
            } else {
                "Some services may be experiencing issues".to_string()
            };
            return StatusPageHealth {
                is_online,
                description: desc,
            };
        }
    }
    StatusPageHealth {
        is_online: false,
        description: "Unable to reach Top.gg Status".into(),
    }
}

