use bluer;

pub async fn yeet() {
    let session = bluer::Session::new().await;

    let session = bluer::Session::new().await.expect("I hate error handling");
    let adapter_names = session.adapter_names().await.expect("I really hate error handling");
    for adapter_name in adapter_names {
        println!("{}", adapter_name);
    }
}

pub fn main() {
    let future = yeet();
    println!("Jeg hater Rust");
}