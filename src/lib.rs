#[no_mangle]
pub extern fn hello(word: String) -> String {
    format!("Hello, {}!", word)
}
