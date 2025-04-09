use actix_web::{web, HttpResponse};
use serde_json::Value;
use serde_json::Map;

use crate::state::read_file;
use crate::json_serialization::to_do_items::ToDoItems;
use crate::to_do::{ItemTypes, to_do_factory, enums::TaskStatus};

pub async fn list(path: web::Path<String>) -> HttpResponse {
    let input_status = path.into_inner().to_lowercase();

    let state: Map<String, Value> = read_file("./state.json");
    let mut array_buffer = Vec::new();

    for (key, value) in state {
        let status_string = value.as_str().unwrap().to_lowercase();

        if status_string == input_status {
            let status = TaskStatus::from_string(status_string.clone());
            let item = to_do_factory(&key, status);
            array_buffer.push(item);
        }
    }

    let response = ToDoItems::new(array_buffer);
    return HttpResponse::Ok().json(response);
}
