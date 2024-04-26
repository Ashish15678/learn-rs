use actix_web::{web, App, HttpServer};

pub mod actix;
use actix::{echo,hello,manual_hello};

#[actix_web::main]
async fn main()->std::io::Result<()> {
    println!("Hello, world!");
    HttpServer::new(||{
        App::new().service(echo).service(hello).route("/hey", web::get().to(manual_hello))
    }).bind(("127.0.0.1",8080))?.run().await
}
