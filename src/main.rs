use redis::Commands;

fn main() {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_connection().unwrap();
    let _: () = con.set("my_key", 42).unwrap();
    let val: i32 = con.get("my_key").unwrap();
    println!("my_key: {}", val);
}
