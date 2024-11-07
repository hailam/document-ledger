use axum::response::Html;

pub async fn get_chain() -> Html<&'static str> {
    Html("<h1>Blockchain Data Here</h1>")
}

pub async fn add_transaction() -> Html<&'static str> {
    Html("<h1>Blockchain Data Here</h1>")
}
