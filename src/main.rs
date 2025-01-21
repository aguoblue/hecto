#![warn(clippy::all, clippy::pedantic)]
mod editor;
use editor::Editor;

fn main() {
    Editor::default().run();
}


//change
// 代码拆分

// Editor::default() 匿名变量，编译器可变借用