use std::sync::Mutex;

use cool::channel::{Receiver, Sender};
use cool::{channel, info};
use once_cell::sync::Lazy;
use tonic::{Request, Response, Status};

use crate::event::ask_pass;
use coolbox_grpc::ask_pass_server::{AskPass, AskPassServer};
use coolbox_grpc::{EmptyRequest, StringResponse};

#[allow(clippy::type_complexity)]
pub static ASK_PASS_TRIGGER_CHANNEL: Lazy<(Mutex<Sender<String>>, Mutex<Receiver<String>>)> =
    Lazy::new(|| {
        let (tx, rx) = channel::unbounded::<String>();
        (Mutex::new(tx), Mutex::new(rx))
    });

pub struct AskPassService;

#[tonic::async_trait]
impl AskPass for AskPassService {
    async fn ask_pass(&self, _: Request<EmptyRequest>) -> Result<Response<StringResponse>, Status> {
        ask_pass();
        let receiver = ASK_PASS_TRIGGER_CHANNEL.1.lock().unwrap().clone();
        let password = receiver.recv().unwrap();
        Ok(Response::new(StringResponse { value: password }))
    }
}

pub async fn start_server() {
    info!("Starting Server");
    let addr = "0.0.0.0:55051";
    let ask_pass = AskPassService {};
    let svc = AskPassServer::new(ask_pass);
    tonic::transport::Server::builder()
        .add_service(svc)
        .serve_with_shutdown(addr.parse().unwrap(), async {
            tokio::signal::ctrl_c()
                .await
                .expect("failed to install CTRL+C signal handler");
        })
        .await
        .unwrap();
}
