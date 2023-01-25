use actix_web::{get, web, HttpResponse, Responder};

#[get("/health")]
pub async fn health() -> impl Responder {
    HttpResponse::Ok().body("PONG")
}

// TODO: #[get("/api/{term: int}")]の中身を実装
#[get("/api/{term}")]
pub async fn get_term_students(path: web::Path<(u64,)>) -> impl Responder {
    let term = path.into_inner().0;
    HttpResponse::Ok().body(format!("term is {}", term))
}

// TODO: #[get("/api/{term: int}/{name: String}")]の中身を実装
#[get("/api/{term}/{name}")]
pub async fn get_student(path: web::Path<(u64, String)>) -> impl Responder {
    let path = path.clone();
    let term = path.0;
    let name = path.1;
    HttpResponse::Ok().body(format!("term is {}, name is {}", term, name))
}
