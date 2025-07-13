use actix_web::{web, HttpResponse, Result};
use serde_json::json;

pub async fn info() -> Result<HttpResponse> {
    let info = json!({
        "success": true,
        "data": {
            "domain": "devforge.in",
            "technology": "Rust + Actix Web",
            "version": "1.0.0",
            "language": "Rust",
            "framework": "Actix Web 4.8",
            "database": "MySQL (SQLx)",
            "templates": "Tera",
            "timestamp": chrono::Utc::now().to_rfc3339()
        }
    });
    
    Ok(HttpResponse::Ok().json(info))
}

pub async fn contacts() -> Result<HttpResponse> {
    let contacts = json!({
        "success": true,
        "data": [
            {
                "id": 1,
                "name": "Welcome",
                "email": "admin@devforge.in",
                "message": "Welcome to your new Rust website!",
                "created_at": "2025-07-10T13:00:00Z"
            },
            {
                "id": 2,
                "name": "System",
                "email": "system@devforge.in",
                "message": "Rust + Actix Web is running perfectly!",
                "created_at": "2025-07-10T13:30:00Z"
            }
        ]
    });
    
    Ok(HttpResponse::Ok().json(contacts))
}
