fn run_loop() {
    let var = 0;

    loop {
        let var = var + 1;
        // let var2 = var + 1;
        // 等价于重新申明了一个新的变量 var2，
        // 因此 var 的值一直为 1；
        println!("var is {}", var);
        if var == 10 {
            // 永远不会执行到这里
            println!("Loop done! {}", var);
            break;
        }
    }
}

fn main() {
    println!(
        "Shadowing(隐藏)，一个 Rust 的新概念
用于使用 **相同的变量名** **重复声明** 变量
这一点在 JAVA|JS 中是不允许的
目前感觉 Shadowing 的作用是 复用变量名称，
反而，在使用中要警惕变量作用域(Scope)的问题，如 run_loop"
    );

    // run_loop(); // 这是个错误的示例
}
