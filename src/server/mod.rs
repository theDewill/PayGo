use actix_web::{get, web, App, HttpServer, Responder};
use std::{collections::HashMap, thread};


use super::Secrets::{PayProcessor};

pub struct SecPolicy {
    pub policy : String
}


pub struct Server {
    pub security_policies : HashMap<String , &SecPolicy>,
    pub server : HttpServer
}


pub struct GatewayState {
    pub threat : bool,
    pub stateData : Mutex<HashMap<String, String>>
}


fn mut_test () -> () {
    let mut gt = Arc::new(GateWayState {
        threat : false,
        stateData : Mutex::new(HashMap::new())
    });

    let t1 = Thread::new(|| {
        let mut gt2 = Arc::clone(&mut gt).lock().unwrap();

    });
}


impl Server {
    pub async fn new (policy_bundle : HashMap<String, &SecPolicy>) -> Self {

        let mut GtState : Data<GatewayState> =  web::Data::new(GatewayState {

           stateData : Mutext::new(HashMap::new())
        });
    
        let tmp_server = HttpServer::new(move || {
            // move counter into the closure
            App::new()
                .app_data(GtState.clone()) // <- register the created data
                .route("/pay", web::get().to(PayProcessor))
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