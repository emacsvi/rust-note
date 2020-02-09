use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("we will play game with guest number");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    loop {
        println!("Please input your guess.");

        // 调用String的类型函数来创建一个可变的guess变量
        let mut guess = String::new();

        // 接收输入，如果报错则panic失败的内容
        io::stdio().read_line(&mut guess)
            .except("Failed to read line.");

        // 这里不用except()来panic,
        // 而是如果报错了，则continue继续下一次
        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num, // 成功则返回Ok返回的num
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // 成功则退出游戏
            }
        }

    }
}
