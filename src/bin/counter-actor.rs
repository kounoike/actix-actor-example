use actix::prelude::*;

/// Counter アクター
#[derive(Default)]
struct Counter {
    count: u32,
}

/// Get メッセージ
#[derive(Message)]
#[rtype(result = "u32")]
struct Get;

/// Increment メッセージ
#[derive(Message)]
#[rtype(result = "()")]
struct Increment;

impl Actor for Counter {
    type Context = Context<Self>;
}

impl Handler<Get> for Counter {
    type Result = u32;

    fn handle(&mut self, _: Get, _: &mut Self::Context) -> Self::Result {
        self.count
    }
}

impl Handler<Increment> for Counter {
    type Result = ();

    fn handle(&mut self, _: Increment, _: &mut Self::Context) {
        self.count += 1;
    }
}

#[actix::main]
async fn main() {
    let counter_addr = Counter::default().start();

    println!("Counter: {}", counter_addr.send(Get).await.unwrap());
    counter_addr.send(Increment).await.unwrap();
    println!("Counter: {}", counter_addr.send(Get).await.unwrap());
    counter_addr.send(Increment).await.unwrap();
    println!("Counter: {}", counter_addr.send(Get).await.unwrap());
}
