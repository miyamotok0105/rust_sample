//rustが初めから読み込む関数をプレリュードという。
//プレリュードにないと自分でuseして読み込む。
use std::io;

use rand::Rng;

use std::cmp::Ordering;
extern crate rand;


//fn mainがプログラムのエントリポイント
//()はからのタプルとして示される
fn main() {
    //println!()はスクリーンに表示するマクロ
    println!("Guess the number!");

    println!("Please input your guess.");
    //letはイミュータブル。let mutはミュータブル。
    //Stringは標準ライブラリ。::new() は関連関数。:: 構文を使用。
    //
    let mut guess = String::new();

    //io::stdin()はターミナルの標準入力へのハンドルを返す。
    //&mutとなってるのに注意
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);

    main1();
}

fn main1() {
    let x = 5;
    let y = 10;

    println!("x and y: {} and {}", x, y);
}
