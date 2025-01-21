#![warn(clippy::all, clippy::pedantic)]
mod editor;
use editor::Editor;

mod assignment;

fn main() {
    Editor::default().run();
    // assignment::tildes::test_tuple_and_loop();
    // assignment::tildes::draw_row();
}


//change
// 代码拆分

// Editor::default() 匿名变量，编译器可变借用