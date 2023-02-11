use pwd_utility_bak::{decrypt_conn,encrypt_conn,encypt_first};
use actix_web::{get,post, App, HttpResponse, HttpServer,Responder,middleware::Logger};
use actix_cors::Cors;
use actix_web_validator::Json;
use serde_json;
#[allow(non_snake_case)]
mod StartServer;
use StartServer::{ConnStruct,RecieveResponse,RecData};

#[get("/")]
async fn home()->impl Responder{
    HttpResponse::Ok().body(include_str!("../templates/home.html"))
}
#[post("/decrypt")]
async fn decrypt_values(req_body:Json<ConnStruct>) -> impl Responder {
    let b= serde_json::to_string(req_body.as_ref()).expect("Unable to deserialize");
    let a:String=decrypt_conn(b);
    HttpResponse::Ok().body(a)
    
}
#[post("/encrypt")]
async fn encrypt_values(req_body: Json<RecieveResponse>) -> impl Responder {
    let b= serde_json::to_string(req_body.as_ref()).expect("Unable to deserialize");
    let a:String=encrypt_conn(b);
    HttpResponse::Ok().body(a)
}

#[post("/encrypt_first")]
async fn encrypt_first(req_body: Json<RecData>) -> impl Responder {
    let b= serde_json::to_string(req_body.as_ref()).expect("Unable to deserialize");
    let a:String=encypt_first(b);
    HttpResponse::Ok().body(a)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main()->std::io::Result<()>{
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    HttpServer::new(move||{

    //let cors= Cors::default().allow_any_origin().send_wildcard();
    let cors= Cors::permissive();

	let logger=Logger::default();
        App::new()
	    .wrap(cors)
        .wrap(logger)
        .service(home)
        .service(decrypt_values)
        .service(encrypt_values)
        .service(encrypt_first)
        .service(echo)
        
    })
    .bind(("localhost", 8080)).expect("Unable to start server")
    .run()
    .await
    
}

