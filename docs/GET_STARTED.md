# Rust入門

## 所有権

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
