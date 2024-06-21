use actix_web::{get, web, App, HttpServer, Responder};

use std::collections::HashMap;

pub struct SecPolicy {
    pub policy : String
}


pub struct Server {
    pub security_policies : HashMap<String , &SecPolicy>,
    pub server : HttpServer
}


pub struct GatewayState {
    pub stateData : Mutex<HashMap<String, String>>
}

impl Server {
    pub async fn new (policy_bundle : HashMap<String, &SecPolicy>) -> Self {

        let mut GtState = web::Data::new(GatewayState {
           stateData : Mutext::new(HashMap::new())
        });
    
        let tmp_server = HttpServer::new(move || {
            // move counter into the closure
            App::new()
                .app_data(GtState.clone()) // <- register the created data
                .route("/", web::get().to(index))
        })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await

        Server {
            security_policies : policy_bundle,
            server : tmp_server
        }
    }

    pub async fn start (&self) {
        self.server
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
    }
}