use std::io;
use lotto::lotto_paper::LottoPaper;
use crate::user::LottoBuyer::LottoBuyer;


fn main() {

    let mut lottoBuyer = LottoBuyer::new();
    lottoBuyer.plus_amount(20000);

    let mut money = String::new();
    println!("로또 구입 금액을 입력해주세요.");
    io::stdin().read_line(&mut money).expect("Failed");
    let money = money.trim().parse::<i64>().unwrap();

    if lottoBuyer.minus_amount(money) > 0 {

    } else {
        println!("그 만큼 돈이 없습니다.");
    }

    // let LottoPaper = LottoPaper {
    //     numbers: vec![1,2,3,4,5,6],
    // };

}

mod lotto;
mod shop;
mod statistics;
mod user;