# Rust入門

## 所有権

何故strは不変でStringは可変か、それはメモリの使い方が違うから。

- strはバイナリにハードコードされ故に高速。
- Stringはコンパイル時に不明の量を確保する必要がある。	

また、StringはOSにメモリの量を要求し、利用し終わったら返却する必要があり、ガベージコレクションが無い場合はアロケートと解放はプログラマが行う必要がある。その場合、以下の問題が起きやすい。

- 解放のタイミングが早い
- 二重解放

Rustはメモリが所有している変数がスコープを抜けると自動で返却する。変数がスコープを抜ける時、dropと呼ばれる関数を呼ぶ。

### Move

s2にs1の参照が入っているのではなく、s1の所有権がs2にmoveしている為、s1は利用できない。

```rust
let s1 = String::from("Hello world");
let s2 = s1;
```

### 可変な参照

特定のスコープで、ある特定のデータに対しては一つしか可変な参照を持てない。データ競合を回避する利点がある。

```rust
let mut s = String::from("hello");

// 1回目
let r1 = &mut s;
// 2回目
let r2 = &mut s;

println!("{}, {}", r1, r2);

```

不変な参照をしている間は、可変な参照をすることはできない。

```rust
let mut s = String::from("hello");

let r1 = &s; // 問題なし
let r2 = &s; // 問題なし
let r3 = &mut s; // 大問題！

```

### 宙に浮いた参照

ダングリングポインタという。ある関数で参照をリターンしたにもかかわらず値がスコープから抜けて解放してしまう。

```rust
fn dangle() -> &String { // dangleはStringへの参照を返す

    let s = String::from("hello"); // sは新しいString

    &s // String sへの参照を返す
} // ここで、sはスコープを抜け、ドロップされる。そのメモリは消される。
  // 危険だ

```

## 構造体

- 構造体を定義する。
- 構造体はインスタンス化して利用する。
- 内容を更新したい構造体は`mut`を付けて可変な構造体にする。
- `..`を構造体更新記法といい、JSの...と同じ展開が可能。
- 構造体を出力するには`#[derive(Debug)]`を構造体の上に書く。
- 出力時のフォーマットは`{:?}`。

```rust

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sing_in_count: u32,
    active: bool,
}

fn main() {

    let user_0 = User {
        username: String::from("Hajime"),
        email: String::from("hajime@example.com"),
        sing_in_count: 0,
        active: true,
    };
   
    // ミュータブルな構造体
    let mut user_1 = User {
        username: String::from("Takeshi"),
        email: String::from("takeshi@example.com"),
        sing_in_count: 0,
        active: true,
    };

    user_1.sing_in_count = 1;
    
    let user_2 = User {
        username: String::from("Hibiki"),
        email: String::from("hibiki@example.com"),
        ..user_1
    };
    
    println!("{:?}", user_0);
    println!("{:?}", user_1);
    println!("{:?}", user_2);
}

```
