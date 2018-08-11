# rust_sample

rustはコンパイラがすごいのだと。でも厳しすぎてイライライするらしいので体験しとく。    
実用性を求めてはいけない。    
C++と同じく標準ライブラリ足りない問題はあると思うが、    
今風の書き方（略）はサポートされている。    


# インストール


```
curl -sSf https://static.rust-lang.org/rustup.sh | sh
```


```
cargo --version

```


# 手動でプロジェクトを作る


ビルドする


```
cd hello_world1
rustc main.rs
./main

```


設定ファイルを使う


```
Cargo.tomlを使ってsrcフォルダ以下のをビルド
cargo build
./target/debug/hello_world
#これでも一発で動かせる
cargo run
```


# コマンドで新規プロジェクトを作る：これでいい！！！


```
cargo new hello_world2 --bin
cd hello_world2
cargo build
./target/debug/hello_world2
```


# 数字当てゲームを作る


```
cargo new guess_the_number --bin
cd guess_the_number
cargo run
```

外部ライブラリを追加


```:Cargo.toml
[dependencies]
rand="0.3.0"

```

ビルド


```
cargo build
```






# 参考

https://rust-lang-ja.github.io/the-rust-programming-language-ja/1.6/book/getting-started.html#rust%E3%81%AE%E3%82%A4%E3%83%B3%E3%82%B9%E3%83%88%E3%83%BC%E3%83%AB



