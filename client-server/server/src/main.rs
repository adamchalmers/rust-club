fn main() {
    let req = schema::Request {
        name: "Leo".to_string(),
    };
    println!("Server: hello, {}", req.name);
}
