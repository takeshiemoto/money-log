# Rust入門

## 所有権

何故strは不変でStringは可変か、それはメモリの使い方が違うから。

- strはバイナリにハードコードされ故に高速。
- Stringはコンパイル時に不明の量を確保する必要がある。	

また、StringはOSにメモリの量を要求し、利用し終わったら返却する必要があり、ガベージコレクションが無い場合はアロケートと解放はプログラマが行う必要がある。その場合、以下の問題が起きやすい。

- 解放のタイミングが早い
- 二重解放

Rustはメモリが所有している変数がスコープを抜けると自動で返却する。変数がスコープを抜ける時、dropと呼ばれる関数を呼ぶ。

### MoveとCopy

- Copyトレイトを実装していたら値はCopy。
- Copyトレイト実装していない場合は所有権がMoveする。

```rust
// Copy
let a = 100;
let b = a;

println!("{}", b);

// Move
let a = String::from("FooBar");
let b = a;

println!("{}", str); // Error!
```

### 借用

- 引数に値の参照を渡すことを借用という。
- 所有権をMoveさせたくない時に使う。
- 所有権を必要としない関数への引数など。

```rust

let world = String::from("world");
hello_print(world);

// worldはhello_printに所有権を奪われているため利用できない
println!("{}", world); // Error!

// hello_printは所有権を奪う
fn hello_print(message: String) {
    println!("Hello {}", message);
}

let world = String::from("world");
hello_print( & world);

// 所有権はMoveしていないので利用できる
println!("{}", world);

// 参照だけを受け取る
fn hello_print(message: &str) {
    println!("Hello {}", message);
}
```
