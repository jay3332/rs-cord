use rs_cord::{Client, EventHandler};

#[tokio::main]
async fn main() {
    println!("{:?}", rs_cord::intents!(MESSAGES, PRESENCES, MEMBERS));

    if let Err(e) =
        Client::new_with_token("OTE0MjgzMDU5NTAxNzM1OTc3.YaKycw.f6l3Mqbz_CEpOJdCgXaCGQCUSmo")
            .with_intents(rs_cord::intents!(MESSAGES))
            .with_event_handler(EventHandler::new().on_ready(|s| async move {
                println!("suspiciousness {}", s.user().await.tag());
            }))
            .start()
            .await
    {
        println!("{:?}", e);
    }
}
