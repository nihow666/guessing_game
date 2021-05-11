use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("請猜測一個數字！");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    loop {
        println!("請輸入你的猜測數字：");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("讀取該行失敗");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小了！\n----------------"),
            Ordering::Greater => println!("太大了！\n----------------"),
            Ordering::Equal => {
                print!("答對了！\n----------------");
                break;
            }
        }
    }
}
