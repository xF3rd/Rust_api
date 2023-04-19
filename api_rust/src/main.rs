use actix_web::*;

mod routes;
use routes::bubble::*;
use routes::quick::*;
use routes::insertion::*;


#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let api = HttpServer::new(|| {
        App::new()
        .route("/bubble", web::get().to(bubble))
        .route("/quick", web::get().to(quick))
        .route("/insertion", web::get().to(insertion))
    });

    let porta = 9091;
    let api = api.bind(format!("127.0.0.1:{}", porta))
    .expect("Nao conectado");

    println!("Conectado \n http://localhost:{}/quick", porta);

    api.run()
    .await
}
