use actix_http::Request;
use actix_service::{IntoServiceFactory, ServiceFactory};
use actix_web::{
    body::MessageBody,
    dev::{AppConfig, Response},
    error::Error,
    get,
    web::{get, Data},
    App, HttpServer,
};

use std::{
    any::Any,
    collections::HashMap,
    fmt,
    sync::{Arc, Mutex},
    thread,
};

use crate::secrets::PayProcessor;

pub struct SecPolicy {
    pub policy: String,
}

pub struct Server<'a, F, I, S, B>
where
    F: Fn() -> I + Send + Clone + 'static,
    I: IntoServiceFactory<S, Request>,
    S: ServiceFactory<Request, Config = AppConfig>,
    S::Error: Into<Error>,
    S::InitError: fmt::Debug,
    S::Response: Into<Response<B>>,
    B: MessageBody,
{
    pub security_policies: HashMap<String, &'a SecPolicy>,
    pub server: HttpServer<F, I, S, B>, //TODO: add the specific type later
}

pub struct GatewayState {
    pub threat: bool, //TODO: add enum for threats and auto fixes later
    pub stateData: Mutex<HashMap<String, String>>,
}

fn mut_test() -> () {
    let mut gt = Arc::new(GatewayState {
        threat: false,
        stateData: Mutex::new(HashMap::new()),
    });

    let t1 = thread::spawn(move || {
        let mut gt2: Arc<GatewayState> = Arc::clone(&gt);
        let mut data = gt2.stateData.lock().unwrap();
    });
}

impl<'a, F, I, S, B: Any> Server<'a, F, I, S, B>
where
    F: Fn() -> I + Send + Clone + 'static,
    I: IntoServiceFactory<S, Request>,
    S: ServiceFactory<Request, Config = AppConfig>,
    S::Error: Into<Error>,
    S::InitError: fmt::Debug,
    S::Response: Into<Response<B>>,
    B: MessageBody,
{
    pub async fn new(policy_bundle: HashMap<String, &'a SecPolicy>) -> Self {
        let mut GtState: Data<GatewayState> = Data::new(GatewayState {
            threat: false,
            stateData: Mutex::new(HashMap::new()),
        });

        let tmp_server = HttpServer::new(move || {
            // move counter into the closure
            App::new()
                .app_data(GtState.clone()) // <- register the created data
                .route("/pay", get().to(PayProcessor))
        });

        Server {
            security_policies: policy_bundle,
            server: tmp_server,
        }
    }

    pub async fn start(&self, port: u16, threads: Option<u8>) {
        match threads {
            Some(t) => {
                self.server
                    .workers(t)
                    .bind(("127.0.0.1", port))?
                    .run()
                    .await
            }

            None => self.server.bind(("127.0.0.1", port))?.run().await,
        }
    }
}
