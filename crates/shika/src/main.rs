use shika_core::server::serve;

const ADDRESS: &str = "0.0.0.0:8081";

#[tokio::main]
async fn main() {
    serve(ADDRESS, index_handler).await;
}

async fn index_handler() -> () {
    // println!("HELLO FROM HANDLER");
}
