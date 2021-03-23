use std::io::{self, prelude::*, BufReader, Write};//导入基本包
use std::net::TcpStream;//导入tcp包
use std::str;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;//链接tcp sever端口
    for _ in 0..10 {
        let mut input = String::new(); //new 一个字符串给input
        io::stdin()
            .read_line(&mut input)  //获取输入字符串
            .expect("Failed to read from stdin");
        stream
            .write(input.as_bytes()) //获取的字符串写入到stream
            .expect("Failed to write to stream");
        
        let mut reader = BufReader::new(&stream); //将stream的结果通过buffreader对象给到reader
        let mut buffer: Vec<u8> = Vec::new(); //new 一个动态数组
        reader 
            .read_until(b'\n', &mut buffer) //读 buffer这个对象
            .expect("Could not read into buffer");//如果异常则输出异常信息
        println!("{}", 
            str::from_utf8(&buffer).expect("Could not write buffer as string"));
        println!("");
    }
    Ok(())
}
