use actix_web::{web, HttpResponse};
use serde_json::value::Value;
use serde_json::Map;

use crate::state::read_file;
use crate::json_serialization::to_do_items::ToDoItems;
use crate::to_do::enums::TaskStatus;

pub async fn list(info: web::Path<String>) -> HttpResponse {
    let status_str = info.into_inner().to_uppercase();
    let status = TaskStatus::from_string(status_str.clone());

    let state: Map<String, Value> = read_file("./state.json");

    let filtered_items = ToDoItems::get_items_by_status(&state, status.clone());

    if filtered_items.is_empty() {
        return HttpResponse::Ok().json(
            format!("No tasks with status: {}", status_str)
        );
    }

    return HttpResponse::Ok().json(filtered_items);
}
