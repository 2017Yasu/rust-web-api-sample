use std::{sync::Mutex, time::Duration};

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

struct AppState {
    app_name: String,    // shared state
    counter: Mutex<i32>, // shared mutable state
}

// /
#[get("/")]
async fn index(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name;
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;

    HttpResponse::Ok().body(format!("Welcome to {app_name}!"))
}

// /api/count
#[get("/count")]
async fn count(data: web::Data<AppState>) -> impl Responder {
    let counter = data.counter.lock().unwrap();
    HttpResponse::Ok().body(format!("Request number: {counter}"))
}

// /echo
#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

// /heavy
#[get("/heavy")]
async fn heaver_process_handler() -> impl Responder {
    tokio::time::sleep(Duration::from_secs(5)).await;
    "heavy process done!"
}

// /api
fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        // /api/test
        web::resource("/test")
            .route(web::get().to(|| async { HttpResponse::Ok().body("test") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    )
    .service(count);
}

fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        // /app
        web::resource("/app")
            .route(web::get().to(|| async { HttpResponse::Ok().body("app") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(AppState {
        app_name: String::from("web-api-sample"),
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .configure(config)
            .service(web::scope("/api").configure(scoped_config))
            .service(index)
            .service(echo)
            .service(heaver_process_handler)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
