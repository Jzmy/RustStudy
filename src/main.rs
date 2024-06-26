use core::num;
use std::io;//prelude
use rand::Rng;//trait 定义很多方法相当于接口
use std::cmp::Ordering;
fn main() {
    println!("猜数!");
    let secret_number =  rand::thread_rng().gen_range(1..101);//包括1不包括101
    // println!("神秘数是 {}",secret_number);
    loop{
        println!("猜测一个数!");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("无法读取输入!");   
    //出错直接退出了健壮性不好 故在处理一下用match表达式
   // let guess:u32 = guess.trim().parse().expect("字符串有字母无法转换");
    let guess:u32 = match guess.trim().parse(){
        Ok(num)=>num,
        Err(_) => continue,
    };
    println!("你猜测得数是 {}",guess);
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("小了!"),
        Ordering::Greater => println!("大了!"),
       // Ordering::Equal => return println!("猜对了!"),自己测试可以用return
        Ordering::Equal => {
            println!("猜对了!");
        break;
    },
    }
    }
    //{}相当于占位符
}
