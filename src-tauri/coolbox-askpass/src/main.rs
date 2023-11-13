use coolbox_grpc::ask_pass_client::AskPassClient;
use coolbox_grpc::EmptyRequest;
use std::io;
use std::io::Write;

#[tokio::main]
async fn main() {
    let mut client = AskPassClient::connect("http://localhost:55051")
        .await
        .unwrap();
    let pass = client
        .ask_pass(tonic::Request::new(EmptyRequest {}))
        .await
        .unwrap()
        .into_inner()
        .value;
    io::stdout().write_all(pass.as_bytes()).unwrap();
}
