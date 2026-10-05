use configcat::*;
mod truck;

const STANDARD_TIME: f64 = 10.0;
const TURBO_TIME: f64 = 3.5;

#[tokio::main]
async fn main() {
    let client =
        Client::new("#YOUR-SDK-KEY#").unwrap();

    let is_turbo_mode_enabled = client.get_value("isTurboModeEnabled", false, None).await;

    if is_turbo_mode_enabled {
        println!("⚡️ TURBO MODE");
        truck::accelerate(TURBO_TIME).await;
    } else {
        println!("🚚 STANDARD MODE");
        truck::accelerate(STANDARD_TIME).await;
    }
}
