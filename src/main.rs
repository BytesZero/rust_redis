use redis::{Commands, Connection};
use std::{error::Error, num::NonZeroUsize};

fn main() -> Result<(), Box<dyn Error>> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;
    for i in 0..1 {
        rw_str(&mut con)?;
        rw_list(&mut con)?;
        rw_set(&mut con)?;
        rw_hash(&mut con)?;
        rw_sorted_set(&mut con)?;
        println!("第{}次", i);
    }
    Ok(())
}

/// 读写字符串、数字
fn rw_str(con: &mut Connection) -> Result<(), Box<dyn Error>> {
    con.set("my_key_num", 499)?;
    let val: i32 = con.get("my_key_num")?;
    println!("my_key_num: {}", val);

    con.set("my_key_str", "Hello, Redis!")?;
    let str: String = con.get("my_key_str")?;
    println!("my_key_str: {}", str);
    // 设置过期时间
    con.set_ex("my_key_ex", "Redis expiration", 10)?;
    // 不存在才插入
    con.set_nx("my_key_nx", "Hello, Rust!")?;
    // 删除
    con.del("my_key_nx")?;

    assert!(con.exists("my_key_num")?);
    assert!(con.exists("my_key_str")?);
    assert!(con.exists("my_key_ex")?);
    assert!(!con.exists("my_key_nx")?);
    Ok(())
}

/// 读写列表
fn rw_list(con: &mut Connection) -> Result<(), Box<dyn Error>> {
    con.rpush("my_list", vec!["a", "b", "c"])?;
    let list: Vec<String> = con.lrange("my_list", 0, -1)?;
    println!("my_list: {:?}", list);
    con.rpop("my_list", NonZeroUsize::new(1))?;
    let list: Vec<String> = con.lrange("my_list", 0, -1)?;
    println!("my_list rpop 1: {:?}", list);
    con.lpush("my_list", vec!["d", "e", "f"])?;
    let list: Vec<String> = con.lrange("my_list", 0, -1)?;
    println!("my_list lpush lpush: {:?}", list);
    con.lpop("my_list", NonZeroUsize::new(2))?;
    let list: Vec<String> = con.lrange("my_list", 0, 2)?;
    println!("my_list lpop 2: {:?}", list);
    Ok(())
}

/// 读写 set
fn rw_set(con: &mut Connection) -> Result<(), Box<dyn Error>> {
    // 添加
    con.sadd("my_set", vec!["a", "b", "c"])?;
    let set: Vec<String> = con.smembers("my_set")?;
    println!("my_set: {:?}", set);
    // 删除
    con.srem("my_set", vec!["a", "b"])?;
    let set: Vec<String> = con.smembers("my_set")?;
    println!("my_set: {:?}", set);
    // 判断是否存在
    assert!(!con.sismember("my_set", "a")?);
    assert!(con.sismember("my_set", "c")?);
    Ok(())
}

/// 读写 hash
fn rw_hash(con: &mut Connection) -> Result<(), Box<dyn Error>> {
    con.hset("my_hash", "name", "zhangsan")?;
    con.hset("my_hash", "age", 18)?;
    let name: String = con.hget("my_hash", "name")?;
    let age: i32 = con.hget("my_hash", "age")?;
    println!("name: {}, age: {}", name, age);
    Ok(())
}

/// 读写有序集合
fn rw_sorted_set(con: &mut Connection) -> Result<(), Box<dyn Error>> {
    con.zadd("my_sorted_set", "a", 1)?;
    con.zadd("my_sorted_set", "b", 2)?;
    con.zadd("my_sorted_set", "c", 3)?;
    con.zadd("my_sorted_set", "d", 4)?;
    let set: Vec<String> = con.zrange("my_sorted_set", 0, -1)?;
    println!("my_sorted_set: {:?}", set);
    let count: i32 = con.zcount("my_sorted_set", 0, 10)?;
    assert!(count == 4);
    Ok(())
}
