use axum::response::Html;

// The handler function for the `/public` endpoint
pub async fn index() -> Html<&'static str> {
    Html("<h1>Welcome to the Blockchain Public API</h1>")
}
