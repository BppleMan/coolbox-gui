use std::sync::Mutex;

use once_cell::sync::Lazy;
use tokio::task::JoinHandle;
use tonic::{Request, Response, Status};

use cool::channel::{Receiver, Sender};
use cool::result::CoolResult;
use cool::{channel, info};
use coolbox_grpc::ask_pass_server::{AskPass, AskPassServer};
use coolbox_grpc::{EmptyRequest, StringResponse};

use crate::event::EventLoop;

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
        EventLoop::ask_pass();
        let receiver = ASK_PASS_TRIGGER_CHANNEL.1.lock().unwrap().clone();
        let password = receiver.recv().unwrap();
        Ok(Response::new(StringResponse { value: password }))
    }
}

pub fn start_server() -> (
    tokio::sync::mpsc::UnboundedSender<()>,
    JoinHandle<CoolResult<()>>,
) {
    info!("Starting Server");
    let addr = "0.0.0.0:55051";
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
    let handle = tokio::spawn(async move {
        let svc = AskPassServer::new(AskPassService);
        tonic::transport::Server::builder()
            .add_service(svc)
            .serve_with_shutdown(addr.parse().unwrap(), async {
                let _ = rx.recv().await;
                info!("Shutting down server");
            })
            .await?;
        Ok(())
    });
    (tx, handle)
}
