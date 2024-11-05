use axum::response::Html;

pub async fn get_blockchain() -> Html<&'static str> {
    Html("<h1>Blockchain Data Here</h1>")
}
