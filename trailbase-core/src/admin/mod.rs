mod config;
mod error;
mod info;
mod jwt;
mod list_logs;
mod oauth_providers;
mod parse;
mod query;
pub(crate) mod rows;
mod schema;
mod table;
pub(crate) mod user;

pub use error::AdminError;

use crate::app_state::AppState;
use axum::{
  routing::{delete, get, patch, post},
  Router,
};

pub fn router() -> Router<AppState> {
  Router::new()
    // Row actions.
    .route("/table/{table_name}/rows", get(rows::list_rows_handler))
    .route("/table/{table_name}/files", get(rows::read_files_handler))
    .route(
      "/table/{table_name}/rows",
      delete(rows::delete_rows_handler),
    )
    .route("/table/{table_name}", patch(rows::update_row_handler))
    .route("/table/{table_name}", post(rows::insert_row_handler))
    .route("/table/{table_name}", delete(rows::delete_row_handler))
    // Index actions.
    .route("/index", post(table::create_index_handler))
    .route("/index", patch(table::alter_index_handler))
    .route("/index", delete(table::drop_index_handler))
    // Table actions.
    .route(
      "/table/{table_name}/schema.json",
      get(table::get_table_schema_handler),
    )
    .route("/table", post(table::create_table_handler))
    .route("/table", delete(table::drop_table_handler))
    .route("/table", patch(table::alter_table_handler))
    // Table & Index actions.
    .route("/tables", get(table::list_tables_handler))
    // Config actions
    .route("/config", get(config::get_config_handler))
    .route("/config", post(config::update_config_handler))
    // User actions
    .route("/user", get(user::list_users_handler))
    .route("/user", post(user::create_user_handler))
    .route("/user", patch(user::update_user_handler))
    // Schema actions
    .route("/schema", get(schema::list_schemas_handler))
    .route("/schema", post(schema::update_schema_handler))
    // Logs
    .route("/logs", get(list_logs::list_logs_handler))
    // Query execution handler for the UI editor
    .route("/query", post(query::query_handler))
    // Parse handler for UI validation.
    .route("/parse", post(parse::parse_handler))
    // List available oauth providers
    .route(
      "/oauth_providers",
      get(oauth_providers::available_oauth_providers_handler),
    )
    .route("/public_key", get(jwt::get_public_key))
    .route("/info", get(info::info_handler))
}
