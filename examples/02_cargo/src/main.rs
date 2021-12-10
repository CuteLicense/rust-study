/**
 * Cargo, rust 的编译+包管理工具
 *
 * cargo --version 查看版本
 * cargo new PROJECT_NAME 生成一个项目
 *
 * cargo build 可编译 src 目录下代码，并存放在 target 目录，debug 目录表示当前为开发调试模式
 * 如需要编译生产版本需指定 --release，cargo build -h 查看详细命令
 *
 * cargo run 等价于 build + run 的操作，先编译再执行
*/

fn main() {
    println!("Hello, world!");
}
