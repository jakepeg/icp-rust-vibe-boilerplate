#[ic_cdk::query]
pub fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}
