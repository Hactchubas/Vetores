use std::path::PathBuf;

use actix_cors::Cors;
use actix_files::NamedFile;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
mod structs;
use structs::vector::Vector;

#[get("/")]
async fn app_home() -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open(PathBuf::from("./static/index.html"))?)
}

// Endpoint para soma de vetores
#[post("/soma")]
async fn soma_vetores(data: web::Json<VectorOperationRequest>) -> impl Responder {
    let resultado = data.v1.clone() + data.v2.clone();
    HttpResponse::Ok().json(resultado)
}

// Endpoint para subtração de vetores
#[post("/subtracao")]
async fn subtracao_vetores(data: web::Json<VectorOperationRequest>) -> impl Responder {
    let resultado = data.v1.clone() - data.v2.clone();
    HttpResponse::Ok().json(resultado)
}

// Endpoint para multiplicação por escalar
#[post("/redimensionar")]
async fn redimensionar(data: web::Json<VectorOperationRequest>) -> impl Responder {
    if let Some(k) = data.scalar {
        let resultado = data.v1.scale(k);
        HttpResponse::Ok().json(resultado)
    } else {
        HttpResponse::BadRequest().body("Escalar não fornecido")
    }
}

// Endpoint para produto escalar
#[post("/produto-escalar")]
async fn produto_escalar(data: web::Json<VectorOperationRequest>) -> impl Responder {
    let resultado = data.v1.dot_product(&data.v2);
    match resultado {
        Some(v) => HttpResponse::Ok().json(v),
        None => HttpResponse::BadRequest().body("Não foi possível calcular o produto escalar"),
    }
}

// Endpoint para produto vetorial
#[post("/produto-vetorial")]
async fn produto_vetorial(data: web::Json<VectorOperationRequest>) -> impl Responder {
    let resultado = data.v1.cross_product(&data.v2);
    match resultado {
        Some(v) => HttpResponse::Ok().json(v),
        None => HttpResponse::BadRequest().body("Não foi possível calcular o produto vetorial"),
    }
}

// Endpoint para projeção
#[post("/projecao")]
async fn projecao_vetores(data: web::Json<VectorOperationRequest>) -> impl Responder {
    let resultado = data.v1.projected_at(&data.v2);
    match resultado {
        Some(v) => HttpResponse::Ok().json(v),
        None => HttpResponse::BadRequest().body("Não foi possível projetar o vetor"),
    }
}

// Endpoint para decomposição
#[post("/decomposicao")]
async fn decomposicao_vetores(data: web::Json<VectorOperationRequest>) -> impl Responder {
    let resultado = data.v1.decompose(&data.v2);
    match resultado {
        Some((v1, v2)) => HttpResponse::Ok().json((v1, v2)),
        None => HttpResponse::BadRequest().body("Não foi possível decompor o vetor"),
    }
}

// Endpoint para reação
#[post("/reacao")]
async fn reacao_vetores(data: web::Json<VectorReactionRequest>) -> impl Responder {
    let resultado = data
        .v1
        .parameterized_reaction(data.alpha, &data.v2, data.beta);
    match resultado {
        Some((vn, vp)) => HttpResponse::Ok().json((vn, vp)),
        None => HttpResponse::BadRequest().body("Não foi possível refletir o vetor"),
    }
}

// Endpoint para visualização da soma
#[get("/soma")]
async fn view_sum() -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open(PathBuf::from(
        "./static/vector-visualization.html",
    ))?)
}

// Endpoint para visualização da reação
#[get("/reacao")]
async fn view_reaction() -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open(PathBuf::from(
        "./static/reaction-visualization.html",
    ))?)
}

// Configuração das rotas
fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(soma_vetores)
            .service(subtracao_vetores)
            .service(redimensionar)
            .service(produto_escalar)
            .service(produto_vetorial)
            .service(projecao_vetores)
            .service(reacao_vetores)
            .service(decomposicao_vetores),
    )
    .service(view_sum)
    .service(view_reaction)
    .service(app_home);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Abertura da página no navegador
    println!("Starting server at http://127.0.0.1:8080");
    if let Err(e) = webbrowser::open("http://127.0.0.1:8080") {
        println!("Failed to open browser: {}", e);
    }

    // Configuração do servidor
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            .configure(configure_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[derive(Deserialize)]
struct VectorOperationRequest {
    v1: Vector,
    v2: Vector,
    scalar: Option<f64>,
}
#[derive(Deserialize)]
struct VectorReactionRequest {
    v1: Vector,
    v2: Vector,
    alpha: f64,
    beta: f64,
}
