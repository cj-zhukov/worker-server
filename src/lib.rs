use color_eyre::Result;
use axum::serve::Serve;
use axum::{routing::get, Router};
use routes::{ping, work};
use tokio::net::TcpListener;

pub mod routes;

#[derive(Debug, Clone)]
pub struct AppState {
    pub server_name: String,
}

impl AppState {
    pub fn new(server_name: String) -> Self {
        Self { server_name }
    }
}

pub struct Application {
    server: Serve<Router, Router>,
    pub address: String,
}

impl Application {
    fn new(server: Serve<Router, Router>, address: String) -> Self {
        Self { server, address }
    }

    pub async fn build(address: String, name: String) -> Result<Self> {   
        let app_state = AppState::new(name);

        let router = Router::new()
            .route("/alive", get(ping))
            .route("/work", get(work))
            .with_state(app_state);

        let listener = TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        Ok(Application::new(server, address))
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on {}", &self.address);
        self.server.await?;

        Ok(())
    }
}