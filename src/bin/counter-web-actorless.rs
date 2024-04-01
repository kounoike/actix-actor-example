// code base from: https://github.com/actix/examples/blob/4a3e27488262dee56f48503f08c68327af6313f0/basics/state/src/main.rs
// modified by KOUNOIKE.

use std::{io, sync::Mutex};

use actix_web::{
    web::{self, Data},
    App, HttpRequest, HttpResponse, HttpServer,
};

async fn index(counter: Data<Mutex<usize>>, _req: HttpRequest) -> HttpResponse {
    let mut counter = counter.lock().unwrap();
    *counter += 1;
    HttpResponse::Ok().body(format!("Request number: {}", counter))
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    // Create some global state prior to building the server
    #[allow(clippy::mutex_atomic)] // it's intentional.
    let counter_mutex = Data::new(Mutex::new(0usize));

    // move is necessary to give closure below ownership of counter1
    HttpServer::new(move || {
        App::new()
            .app_data(counter_mutex.clone()) // add shared state
            // register simple handler
            .service(web::resource("/").to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
