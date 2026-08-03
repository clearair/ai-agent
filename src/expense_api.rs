use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::get,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, sync::Arc};
use tokio::sync::RwLock;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Expense {
    pub id: String,
    pub description: String,
    pub amount: f64,
    pub category: String,
    pub date: String,
}

#[derive(Clone, Default)]
pub struct AppState {
    expenses: Arc<RwLock<Vec<Expense>>>,
    next_id: Arc<std::sync::atomic::AtomicU64>,
    api_key: String,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            expenses: Arc::new(RwLock::new(Vec::new())),
            next_id: Arc::new(std::sync::atomic::AtomicU64::new(1)),
            api_key: String::new(),
        }
    }

    fn with_api_key(mut self, api_key: String) -> Self {
        self.api_key = api_key;
        self
    }
}

#[derive(Debug, Deserialize)]
struct ListExpensesQuery {
    category: Option<String>,
    month: Option<String>,
}

#[derive(Debug, Deserialize)]
struct SummaryQuery {
    month: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CreateExpense {
    description: String,
    amount: f64,
    category: String,
    date: String,
}

#[derive(Debug, Deserialize)]
struct UpdateExpense {
    description: Option<String>,
    amount: Option<f64>,
    category: Option<String>,
    date: Option<String>,
}

#[derive(Debug, Serialize)]
struct Summary {
    total: f64,
    by_category: BTreeMap<String, f64>,
}

#[derive(Debug, Serialize)]
struct ErrorBody {
    error: String,
}

#[derive(Debug)]
struct ApiError(StatusCode, String);

impl ApiError {
    fn bad_request(message: impl Into<String>) -> Self {
        Self(StatusCode::BAD_REQUEST, message.into())
    }

    fn unauthorized() -> Self {
        Self(
            StatusCode::UNAUTHORIZED,
            "invalid or missing x-api-key".to_string(),
        )
    }

    fn not_found() -> Self {
        Self(StatusCode::NOT_FOUND, "expense not found".to_string())
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        (self.0, Json(ErrorBody { error: self.1 })).into_response()
    }
}

pub fn app(state: AppState, api_key: String) -> Router {
    let state = state.with_api_key(api_key);

    Router::new()
        .route("/expenses", get(list_expenses).post(create_expense))
        .route("/expenses/summary", get(get_summary))
        .route(
            "/expenses/{id}",
            get(get_expense).put(update_expense).delete(delete_expense),
        )
        .with_state(state)
}

async fn authorize(headers: &HeaderMap, state: &AppState) -> Result<(), ApiError> {
    let provided = headers
        .get("x-api-key")
        .and_then(|value| value.to_str().ok());

    if provided == Some(state.api_key.as_str()) && !state.api_key.is_empty() {
        Ok(())
    } else {
        Err(ApiError::unauthorized())
    }
}

fn validate_date(date: &str) -> Result<(), ApiError> {
    NaiveDate::parse_from_str(date, "%Y-%m-%d")
        .map(|_| ())
        .map_err(|_| ApiError::bad_request("date must use YYYY-MM-DD format"))
}

fn validate_month(month: &str) -> Result<(), ApiError> {
    NaiveDate::parse_from_str(&format!("{month}-01"), "%Y-%m-%d")
        .map(|_| ())
        .map_err(|_| ApiError::bad_request("month must use YYYY-MM format"))
}

fn validate_amount(amount: f64) -> Result<(), ApiError> {
    if amount.is_finite() && amount >= 0.0 {
        Ok(())
    } else {
        Err(ApiError::bad_request(
            "amount must be a finite non-negative number",
        ))
    }
}

async fn list_expenses(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(query): Query<ListExpensesQuery>,
) -> Result<Json<Vec<Expense>>, ApiError> {
    authorize(&headers, &state).await?;
    if let Some(month) = &query.month {
        validate_month(month)?;
    }

    let category = query.category.map(|value| value.to_lowercase());
    let expenses = state.expenses.read().await;
    let result = expenses
        .iter()
        .filter(|expense| {
            category
                .as_deref()
                .is_none_or(|value| expense.category.to_lowercase() == value)
        })
        .filter(|expense| {
            query
                .month
                .as_deref()
                .is_none_or(|value| expense.date.starts_with(value))
        })
        .cloned()
        .collect();

    Ok(Json(result))
}

async fn get_expense(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Result<Json<Expense>, ApiError> {
    authorize(&headers, &state).await?;
    state
        .expenses
        .read()
        .await
        .iter()
        .find(|expense| expense.id == id)
        .cloned()
        .map(Json)
        .ok_or_else(ApiError::not_found)
}

async fn create_expense(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(input): Json<CreateExpense>,
) -> Result<(StatusCode, Json<Expense>), ApiError> {
    authorize(&headers, &state).await?;
    validate_date(&input.date)?;
    validate_amount(input.amount)?;

    let id = state
        .next_id
        .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        .to_string();
    let expense = Expense {
        id,
        description: input.description,
        amount: input.amount,
        category: input.category,
        date: input.date,
    };

    state.expenses.write().await.push(expense.clone());
    Ok((StatusCode::CREATED, Json(expense)))
}

async fn update_expense(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<String>,
    Json(input): Json<UpdateExpense>,
) -> Result<Json<Expense>, ApiError> {
    authorize(&headers, &state).await?;
    if let Some(amount) = input.amount {
        validate_amount(amount)?;
    }
    if let Some(date) = &input.date {
        validate_date(date)?;
    }

    let mut expenses = state.expenses.write().await;
    let expense = expenses
        .iter_mut()
        .find(|expense| expense.id == id)
        .ok_or_else(ApiError::not_found)?;

    if let Some(description) = input.description {
        expense.description = description;
    }
    if let Some(amount) = input.amount {
        expense.amount = amount;
    }
    if let Some(category) = input.category {
        expense.category = category;
    }
    if let Some(date) = input.date {
        expense.date = date;
    }

    Ok(Json(expense.clone()))
}

async fn delete_expense(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Result<StatusCode, ApiError> {
    authorize(&headers, &state).await?;
    let mut expenses = state.expenses.write().await;
    let old_len = expenses.len();
    expenses.retain(|expense| expense.id != id);

    if expenses.len() == old_len {
        Err(ApiError::not_found())
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}

async fn get_summary(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(query): Query<SummaryQuery>,
) -> Result<Json<Summary>, ApiError> {
    authorize(&headers, &state).await?;
    if let Some(month) = &query.month {
        validate_month(month)?;
    }

    let expenses = state.expenses.read().await;
    let matching = expenses.iter().filter(|expense| {
        query
            .month
            .as_deref()
            .is_none_or(|value| expense.date.starts_with(value))
    });

    let mut total = 0.0;
    let mut by_category = BTreeMap::new();
    for expense in matching {
        total += expense.amount;
        *by_category.entry(expense.category.clone()).or_insert(0.0) += expense.amount;
    }

    Ok(Json(Summary { total, by_category }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, http::Request};
    use serde_json::{Value, json};
    use tower::ServiceExt;

    async fn body_json(response: axum::response::Response) -> Value {
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        serde_json::from_slice(&body).unwrap()
    }

    fn request(method: &str, uri: &str, body: Option<Value>) -> Request<Body> {
        let mut builder = Request::builder()
            .method(method)
            .uri(uri)
            .header("x-api-key", "test-key");
        if body.is_some() {
            builder = builder.header("content-type", "application/json");
        }
        builder
            .body(Body::from(
                body.map_or_else(String::new, |value| value.to_string()),
            ))
            .unwrap()
    }

    #[tokio::test]
    async fn rejects_requests_without_the_api_key() {
        let response = app(AppState::new(), "test-key".to_string())
            .oneshot(
                Request::builder()
                    .uri("/expenses")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn supports_create_list_get_update_delete_and_summary() {
        let app = app(AppState::new(), "test-key".to_string());
        let create = app
            .clone()
            .oneshot(request(
                "POST",
                "/expenses",
                Some(json!({
                    "description": "Lunch",
                    "amount": 12.5,
                    "category": "Food",
                    "date": "2026-08-03"
                })),
            ))
            .await
            .unwrap();
        assert_eq!(create.status(), StatusCode::CREATED);
        let created: Expense = serde_json::from_value(body_json(create).await).unwrap();
        assert_eq!(created.id, "1");

        let list = app
            .clone()
            .oneshot(request(
                "GET",
                "/expenses?category=food&month=2026-08",
                None,
            ))
            .await
            .unwrap();
        assert_eq!(list.status(), StatusCode::OK);
        assert_eq!(body_json(list).await.as_array().unwrap().len(), 1);

        let get = app
            .clone()
            .oneshot(request("GET", "/expenses/1", None))
            .await
            .unwrap();
        assert_eq!(get.status(), StatusCode::OK);

        let update = app
            .clone()
            .oneshot(request("PUT", "/expenses/1", Some(json!({"amount": 15.0}))))
            .await
            .unwrap();
        assert_eq!(update.status(), StatusCode::OK);
        assert_eq!(body_json(update).await["amount"], 15.0);

        let summary = app
            .clone()
            .oneshot(request("GET", "/expenses/summary?month=2026-08", None))
            .await
            .unwrap();
        assert_eq!(summary.status(), StatusCode::OK);
        assert_eq!(body_json(summary).await["total"], 15.0);

        let delete = app
            .clone()
            .oneshot(request("DELETE", "/expenses/1", None))
            .await
            .unwrap();
        assert_eq!(delete.status(), StatusCode::NO_CONTENT);

        let missing = app
            .oneshot(request("GET", "/expenses/1", None))
            .await
            .unwrap();
        assert_eq!(missing.status(), StatusCode::NOT_FOUND);
    }
}
