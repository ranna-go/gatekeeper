use actix_web::{web::Data, HttpRequest, HttpResponse, Responder};

use crate::Context;

pub async fn handle(req: HttpRequest, data: Data<Context>) -> impl Responder {
    if let Some(token_value) = req.headers().get("authorization") {
        let token_value = token_value.to_str().unwrap_or_default();
        if !token_value.is_empty() && token_value.to_lowercase().starts_with("basic ") {
            if let Some(ident) = data
                .basic_tokens
                .check(&token_value[6..].trim().to_string())
            {
                return HttpResponse::Ok()
                    .header("X-Gatekeeper-Ident", ident.ident.clone())
                    .finish();
            }
        }
    }

    HttpResponse::Unauthorized().finish()
}
