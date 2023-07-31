fn main() {
    let req = schema::Request {
        name: "Leo".to_string(),
    };
    println!("Client: hello, {}", req.name);
}
