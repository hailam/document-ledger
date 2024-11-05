use crate::models::Node;
use axum::{extract::State, Json};
use std::sync::{Arc, Mutex};

pub async fn get_all_nodes(State(nodes): State<Arc<Mutex<Vec<Node>>>>) -> Json<Vec<Node>> {
    let nodes = nodes.lock().unwrap();
    Json(nodes.clone())
}

#[axum::debug_handler]
pub async fn register_node(
    State(nodes): State<Arc<Mutex<Vec<Node>>>>,
    Json(new_node): Json<Node>,
) -> Json<String> {
    let mut nodes = nodes.lock().unwrap();
    nodes.push(new_node);
    Json("Node registered successfully".to_string())
}
