mod security;
mod server;
mod secrets;

//custom t-units
use security::{encryptor::{Encryptor, SourceTypes}};
use server::Server;
//externals
use std::fmt::Display;
use std::collections::HashMap;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

//---- Test Lab ------



//--------

#[actix_web::main]
fn main() {
    let mut resoruces: Vec<SourceTypes> = Vec::with_capacity(4);

    //Game On
    let ENC = Encryptor::NEW(resoruces, None);
    let server : <Server> = Server::new(HashMap::new()); // server init

 
}
