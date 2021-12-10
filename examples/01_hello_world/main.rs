/**
 * fn: 关键字，声明函数
 * main: 特殊函数名称，入口函数
 * 
 * 没有参数，没有返回值
 * 
 * 编译：rustc ./main.rs
 * 
 * rustc 是 rust 的编译器 compiler，类似javac
 * 编译之后会生成可执行二进制文件，在Mac系统下没有后缀，如： main，编译完成之后
 * 
 * 执行二进制文件，./main
 */

fn main() {
  // println调用内置函数，感叹号(!)表示这是一个宏(Macro)
  println!("Hello world");
}
