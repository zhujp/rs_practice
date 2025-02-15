use redis::{Commands, RedisResult};
use dotenv::dotenv;
use std::env;

fn main() -> RedisResult<()> {
    // 加载 .env 文件
    dotenv().ok();

    // 从环境变量中读取 Redis 连接配置
    let redis_url = env::var("REDIS_URL")
        .expect("REDIS_URL must be set in .env file"); //密码如果特殊字符多有可能解析失败

    println!("Connecting to Redis server at {}", redis_url);
    // 连接到 Redis 服务器
    let client = redis::Client::open(redis_url)?;
    let mut con = client.get_connection()?;

    // 设置键值对
    let _: () = con.set("uname", "vilay")?;
    println!("Key 'my_key' set with value 'my_value'.");

    // 获取键值对
    let value: String = con.get("uname")?;
    println!("Value for 'my_key': {}", value);


    //let _: () = con.set_ex("my_key", "my_value", 10)?; // 10 秒后过期
    //命令参考 https://docs.rs/redis/0.28.2/redis/trait.Commands.html
    Ok(())
}