use redis::Commands;

fn main() -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1:6379/")?;
    let mut connection = client.get_connection()?;

    connection.hset("user:1", "id", "1")?;
    connection.hset("user:1", "name", "Ziggy")?;
    connection.hset("user:1", "age", "29")?;

    let sections: Vec<String> = connection.hgetall("user:1")?;

    let mut is_key = true;
    for (i, s) in sections.iter().enumerate() {
        if is_key {
            println!("{}: {}", s, sections.get(i + 1).unwrap());
            is_key = false;
        } else {
            is_key = true;
        }
    }

    let name: String = connection.hget("user:1", "name")?;
    println!("{:?}", name);

    Ok(())
}
