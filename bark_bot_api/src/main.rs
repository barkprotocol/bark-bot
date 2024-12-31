use axum::{
    extract::{Json, Path, State},
    response::{IntoResponse, Result as AxumResult},
    routing::{get, patch, post},
    Router,
};
use rusqlite::{Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use std::{sync::Arc, net::SocketAddr};
use tokio::sync::Mutex;
use tracing::{info, error};

#[derive(Debug, Serialize, Deserialize)]
struct Transaction {
    id: i64,
    transaction_index: i32,
    user_id: String,
    signature: String,
    status: u8,
    message_id: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct CreateTransactionBody {
    transaction_index: i32,
    user_id: String,
    signature: String,
}

#[derive(Deserialize)]
struct UpdateTransactionBody {
    message_id: String,
}

struct AppState {
    conn: Mutex<Connection>,
}

const DB_PATH: &str = "./db.sqlite";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let conn = Connection::open(DB_PATH).expect("Failed to open database");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS entries (
            id INTEGER PRIMARY KEY,
            transaction_index INTEGER NOT NULL,
            user_id TEXT NOT NULL,
            signature TEXT NOT NULL,
            status INTEGER NOT NULL,
            message_id TEXT
        )",
        (),
    )
    .expect("Failed to create table");

    let app_state = Arc::new(AppState {
        conn: Mutex::new(conn),
    });

    let app = Router::new()
        .route("/transactions", post(create_transaction))
        .route("/transactions/:transaction_id", get(get_transaction))
        .route("/transactions/:transaction_id", patch(update_transaction))
        .with_state(app_state);

    let addr: SocketAddr = "127.0.0.1:3000".parse().expect("Invalid address");
    info!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn create_transaction(
    State(state): State<Arc<AppState>>,
    Json(body): Json<CreateTransactionBody>,
) -> AxumResult<impl IntoResponse> {
    let conn = state.conn.lock().await;

    match conn.execute(
        "INSERT INTO entries (transaction_index, user_id, signature, status, message_id) VALUES (?1, ?2, ?3, ?4, ?5)",
        (&body.transaction_index, &body.user_id, &body.signature, 0, None::<String>),
    ) {
        Ok(_) => {
            let id = conn.last_insert_rowid();
            info!("Transaction created: ID={}", id);
            Ok((
                axum::http::StatusCode::CREATED,
                Json(Transaction {
                    id,
                    transaction_index: body.transaction_index,
                    user_id: body.user_id.clone(),
                    signature: body.signature.clone(),
                    status: 0,
                    message_id: None,
                }),
            ))
        }
        Err(e) => {
            error!("Error creating transaction: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR.into())
        }
    }
}

async fn get_transaction(
    State(state): State<Arc<AppState>>,
    Path(transaction_id): Path<i64>,
) -> AxumResult<impl IntoResponse> {
    let conn = state.conn.lock().await;

    match conn
        .prepare(
            "SELECT id, transaction_index, user_id, signature, status, message_id FROM entries WHERE id = ?1",
        )
        .and_then(|mut stmt| {
            stmt.query_row([transaction_id], |row| {
                Ok(Transaction {
                    id: row.get(0)?,
                    transaction_index: row.get(1)?,
                    user_id: row.get(2)?,
                    signature: row.get(3)?,
                    status: row.get(4)?,
                    message_id: row.get(5)?,
                })
            })
        }) {
        Ok(transaction) => Ok(Json(Some(transaction))),
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            Ok((axum::http::StatusCode::NOT_FOUND, Json(None)))
        }
        Err(e) => {
            error!("Error fetching transaction: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR.into())
        }
    }
}

async fn update_transaction(
    State(state): State<Arc<AppState>>,
    Path(transaction_id): Path<i64>,
    Json(body): Json<UpdateTransactionBody>,
) -> AxumResult<impl IntoResponse> {
    let conn = state.conn.lock().await;

    match conn.execute(
        "UPDATE entries SET status = 1, message_id = ?1 WHERE id = ?2",
        (&Some(body.message_id.clone()), transaction_id),
    ) {
        Ok(_) => {
            info!("Transaction updated: ID={}", transaction_id);
            let updated_transaction = get_transaction(State(state), Path(transaction_id))
                .await?
                .into_inner();
            Ok((axum::http::StatusCode::OK, Json(updated_transaction)))
        }
        Err(e) => {
            error!("Error updating transaction: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR.into())
        }
    }
}
