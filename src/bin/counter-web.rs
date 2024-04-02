use actix::prelude::*;
use actix_web::{get, web, Responder};

/// Counter アクター
#[derive(Default)]
struct Counter {
    count: u32,
}

/// IncrementAndGet メッセージ
#[derive(Message)]
#[rtype(result = "u32")]
struct IncrementAndGet;

impl Actor for Counter {
    type Context = Context<Self>;
}

impl Handler<IncrementAndGet> for Counter {
    type Result = u32;

    fn handle(&mut self, _: IncrementAndGet, _: &mut Self::Context) -> Self::Result {
        self.count += 1;
        self.count
    }
}

#[get("/")]
async fn get(counter: web::Data<Addr<Counter>>) -> impl Responder {
    let count = counter.send(IncrementAndGet).await.unwrap();
    format!("count: {}", count)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter_addr = Counter::default().start();

    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .app_data(web::Data::new(counter_addr.clone()))
            .service(get)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
