use std::ops::Add;
use std::sync::RwLock;
use std::sync::Arc;
use actix_web::HttpServer;
use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::{web, App};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt()]
struct Opt {
    /// Socket addresses to listen on for requests
    /// e.g. 127.0.0.1:8080
    #[structopt(short, long, required(true))]
    socket_addrs: Vec<std::net::SocketAddr>,

    /// Specifiy client_timeout. Default is `std::u64::MAX`.
    #[structopt(short,long)]
    client_timeout: Option<u64>,

    /// Specifiy client_shutdown. Default is `std::u64::MAX`.
    #[structopt(short,long)]
    client_shutdown: Option<u64>,
}

async fn noop() -> impl Responder {
    HttpResponse::Ok()
}

async fn read_the_counter(data: web::Data<Arc<RwLock<u32>>>) -> impl Responder {
    let mut result = String::new();
    {
        let handle = data.read().unwrap();
        result.push_str(&format!("{}", *handle));
    }
    HttpResponse::Ok().content_type("text/plain").body(result)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();
    let guarded_counter: Arc<RwLock<u32>> = Arc::new(RwLock::new(0));
    {
        let guarded_counter_for_background_thread = guarded_counter.clone();
        tokio::spawn(async move {
            loop {
                {
                    let mut handle = guarded_counter_for_background_thread.write().unwrap();
                    *handle += 1;
                }
                tokio::time::delay_until(
                    tokio::time::Instant::now().add(tokio::time::Duration::from_millis(250)),
                )
                .await;
            }
        });
    }
    let mut server = HttpServer::new(move || {
        App::new()
            .data(guarded_counter.clone())
            .route("/counter", web::get().to(read_the_counter))
            .route("/noop", web::get().to(noop))
    });
    for addr in opt.socket_addrs.iter() {
        server = server.bind(addr).unwrap();
    }
    server
        .client_timeout(opt.client_timeout.unwrap_or(std::u64::MAX))
        .client_shutdown(opt.client_shutdown.unwrap_or(std::u64::MAX))
        .max_connections(std::usize::MAX)
        .max_connection_rate(std::usize::MAX)
        .run()
        .await
}
