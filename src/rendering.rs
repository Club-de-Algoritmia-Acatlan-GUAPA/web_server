use axum::{extract::Request, middleware::Next, response::Response};

use crate::with_axum::Template;

#[derive(Template, Clone, Default)]
#[template(path = "navbar.html")]
pub struct Navbar {
    pub user_id: String,
}

#[derive(Template, Clone, Default)]
#[template(path = "base.html", escape = "none")]
pub struct WholePage {
    pub navbar: Navbar,
    content: String,
    pub test: String,
}

impl WholePage {
    pub fn with_content<T: askama::Template>(&mut self, content: T) -> &mut Self {
        self.content = content.render().unwrap();
        self
    }
}
pub async fn render(mut request: Request, next: Next) -> Response {
    request.extensions_mut().insert(WholePage {
        test: "test".to_string(),
        ..Default::default()
    });
    next.run(request).await
}
