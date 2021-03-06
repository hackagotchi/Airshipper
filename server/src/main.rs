// How to send manual webhooks:
// curl --header "Content-Type: application/json" --request POST --data "@<FILE_WITH_WEBHOOK_DATA>" --header "X-Gitlab-Event: Pipeline Hook" --header "X-Gitlab-Token: <TOKEN>" http://<ADDRESS>
mod logger;

fn main() {
    dotenv::from_filename(".airshipper-env").ok();
    logger::init();
    server::rocket().launch().expect("Server failed!");
}
