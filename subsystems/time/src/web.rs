use crate::commands::{execute_command, CommandOutcome};
use crate::time::{ClockAngles, Lightzone, TimeState, Timestamp};
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, Json},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tower_http::services::ServeDir;

pub type SharedTimeState = Arc<Mutex<TimeState>>;

#[derive(Debug, Serialize)]
pub struct TimestampResponse {
    pub day: usize,
    pub minute: u16,
    pub hhmm: String,
    pub lightzone: Lightzone,
}

#[derive(Debug, Serialize)]
pub struct ClockAnglesResponse {
    pub hand_15_deg: f64,
    pub dial_45_deg_total: f64,
    pub dial_45_deg_visual: f64,
}

#[derive(Debug, Serialize)]
pub struct ActionCostResponse {
    pub command: String,
    pub minutes: u32,
}

#[derive(Debug, Serialize)]
pub struct TimeResponse {
    pub success: bool,
    pub message: String,
    pub minutes_advanced: u32,
    pub timestamp: TimestampResponse,
    pub clock_angles: ClockAnglesResponse,
}

#[derive(Debug, Deserialize)]
pub struct TickRequest {
    pub minutes: u32,
}

#[derive(Debug, Deserialize)]
pub struct CommandRequest {
    pub command: String,
}

pub fn create_router(state: SharedTimeState) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/api/state", get(get_state))
        .route("/api/tick", post(post_tick))
        .route("/api/command", post(post_command))
        .route("/api/actions", get(get_actions))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(state)
}

async fn index() -> Html<&'static str> {
    Html(include_str!("../static/index.html"))
}

async fn get_state(State(state): State<SharedTimeState>) -> Result<Json<TimeResponse>, StatusCode> {
    let guard = state
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(build_response(
        &guard,
        CommandOutcome {
            success: true,
            message: "ok".to_string(),
            minutes_advanced: 0,
        },
    )))
}

async fn post_tick(
    State(state): State<SharedTimeState>,
    Json(req): Json<TickRequest>,
) -> Result<Json<TimeResponse>, StatusCode> {
    let mut guard = state
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let outcome = if req.minutes == 0 {
        CommandOutcome {
            success: true,
            message: "Ticked 0 minute(s)".to_string(),
            minutes_advanced: 0,
        }
    } else {
        execute_command(&mut guard, &format!("tick {}", req.minutes))
    };
    Ok(Json(build_response(&guard, outcome)))
}

async fn post_command(
    State(state): State<SharedTimeState>,
    Json(req): Json<CommandRequest>,
) -> Result<Json<TimeResponse>, StatusCode> {
    let mut guard = state
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let outcome = execute_command(&mut guard, &req.command);
    Ok(Json(build_response(&guard, outcome)))
}

async fn get_actions(
    State(state): State<SharedTimeState>,
) -> Result<Json<Vec<ActionCostResponse>>, StatusCode> {
    let guard = state
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let mut actions: Vec<ActionCostResponse> = guard
        .action_costs()
        .iter()
        .map(|(command, minutes)| ActionCostResponse {
            command: command.clone(),
            minutes: *minutes,
        })
        .collect();
    actions.sort_by(|a, b| a.command.cmp(&b.command));
    Ok(Json(actions))
}

fn build_response(state: &TimeState, outcome: CommandOutcome) -> TimeResponse {
    let timestamp = state.timestamp();
    let angles = state.clock_angles();

    TimeResponse {
        success: outcome.success,
        message: outcome.message,
        minutes_advanced: outcome.minutes_advanced,
        timestamp: timestamp_response(timestamp, state.hhmm()),
        clock_angles: angles_response(angles),
    }
}

fn timestamp_response(timestamp: Timestamp, hhmm: String) -> TimestampResponse {
    TimestampResponse {
        day: timestamp.day,
        minute: timestamp.minute,
        hhmm,
        lightzone: timestamp.lightzone,
    }
}

fn angles_response(angles: ClockAngles) -> ClockAnglesResponse {
    ClockAnglesResponse {
        hand_15_deg: angles.hand_15_deg,
        dial_45_deg_total: angles.dial_45_deg_total,
        dial_45_deg_visual: angles.dial_45_deg_visual,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Method, Request};
    use http_body_util::BodyExt;
    use tower::util::ServiceExt;

    fn test_app() -> Router {
        create_router(Arc::new(Mutex::new(TimeState::default())))
    }

    #[tokio::test]
    async fn get_state_initial() {
        let app = test_app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/state")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let bytes = response.into_body().collect().await.unwrap().to_bytes();
        let body = String::from_utf8(bytes.to_vec()).unwrap();
        assert!(body.contains("\"day\":0"));
        assert!(body.contains("\"minute\":0"));
    }

    #[tokio::test]
    async fn post_tick_advances_time() {
        let app = test_app();

        let response = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/api/tick")
                    .header("content-type", "application/json")
                    .body(Body::from("{\"minutes\":60}"))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let bytes = response.into_body().collect().await.unwrap().to_bytes();
        let body = String::from_utf8(bytes.to_vec()).unwrap();
        assert!(body.contains("\"minutes_advanced\":60"));
        assert!(body.contains("\"minute\":60"));
    }

    #[tokio::test]
    async fn post_command_craft_totem() {
        let app = test_app();

        let response = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/api/command")
                    .header("content-type", "application/json")
                    .body(Body::from("{\"command\":\"craft totem\"}"))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let bytes = response.into_body().collect().await.unwrap().to_bytes();
        let body = String::from_utf8(bytes.to_vec()).unwrap();
        assert!(body.contains("\"minutes_advanced\":40"));
        assert!(body.contains("\"minute\":40"));
    }

    #[tokio::test]
    async fn unknown_command_does_not_mutate() {
        let app = test_app();

        let response = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/api/command")
                    .header("content-type", "application/json")
                    .body(Body::from("{\"command\":\"nope\"}"))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let bytes = response.into_body().collect().await.unwrap().to_bytes();
        let body = String::from_utf8(bytes.to_vec()).unwrap();
        assert!(body.contains("\"success\":false"));
        assert!(body.contains("\"minute\":0"));
    }

    #[tokio::test]
    async fn actions_include_craft_totem() {
        let app = test_app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/actions")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let bytes = response.into_body().collect().await.unwrap().to_bytes();
        let body = String::from_utf8(bytes.to_vec()).unwrap();
        assert!(body.contains("craft totem"));
        assert!(body.contains("\"minutes\":40"));
    }
}
