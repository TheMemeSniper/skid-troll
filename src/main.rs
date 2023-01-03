use reqwest;
use clipboard::{ClipboardContext, ClipboardProvider};

//#[tokio::main]
async fn gettroll() -> String {
    let resp = reqwest::get("https://coolelectronics.me/api/generator/gen").await.unwrap();
    resp.text().await.unwrap()
}

#[tokio::main]
async fn main() {
    let text = gettroll();
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(text.await).unwrap();
}
