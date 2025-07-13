use actix_web::{web, HttpResponse, Result};
use tera::{Tera, Context};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ContactForm {
    name: String,
    email: String,
    message: String,
}

pub async fn index(tera: web::Data<Tera>) -> Result<HttpResponse> {
    let mut context = Context::new();
    context.insert("title", "Welcome to devforge.in");
    context.insert("domain", "devforge.in");
    
    let rendered = tera.render("index.html", &context)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    
    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}

pub async fn about(tera: web::Data<Tera>) -> Result<HttpResponse> {
    let mut context = Context::new();
    context.insert("title", "About - devforge.in");
    context.insert("domain", "devforge.in");
    
    let rendered = tera.render("about.html", &context)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    
    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}

pub async fn contact(tera: web::Data<Tera>) -> Result<HttpResponse> {
    let mut context = Context::new();
    context.insert("title", "Contact - devforge.in");
    context.insert("domain", "devforge.in");
    
    let rendered = tera.render("contact.html", &context)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    
    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}

pub async fn contact_post(
    form: web::Form<ContactForm>,
    tera: web::Data<Tera>
) -> Result<HttpResponse> {
    println!("📧 Contact form submitted:");
    println!("Name: {}", form.name);
    println!("Email: {}", form.email);
    println!("Message: {}", form.message);
    
    let mut context = Context::new();
    context.insert("title", "Contact - devforge.in");
    context.insert("domain", "devforge.in");
    context.insert("success", "Thank you for your message! We'll get back to you soon.");
    
    let rendered = tera.render("contact.html", &context)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    
    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}
