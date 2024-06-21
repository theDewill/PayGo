mod security::{encryptor::{Encryptorm, SourceTypes}};
mod server;


use std::fmt::Display;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

//---- Test Lab ------


//--------

fn main() {

   



    let mut resoruces: Vec<SourceTypes> = Vec::with_capacity(4);
    //Game On
    let ENC = Encryptor::NEW(resoruces, None);
}
