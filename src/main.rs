use yari::entry;


#[tokio::main]
async fn main() {
    env_logger::init();
    entry().await;
}
