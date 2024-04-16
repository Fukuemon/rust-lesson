pub mod sub_a; // サブモジュールを読み込む（pubをつけることで呼び出し先から使用可能）
mod sub_b;

// サブモジュールの関数を呼び出す（pubをつけることで使用可能）
pub fn run() {
    println!("Called run() from vars.rs");
    sub_a::func_a();
    sub_b::func_b();
}
