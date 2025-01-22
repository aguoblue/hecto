use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use std::io::Error;

mod terminal;
use terminal::{Terminal, Size, Position};

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn default() -> Self {
        Editor { should_quit: false }
    }

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event);
        }
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            match code {
                Char('c') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                _ => (),
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::hide_cursor()?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye.\r\n")?;
        } else {
            Self::draw_rows()?;
            Terminal::move_cursor_to(Position{x:0,y:0})?;
        }
        Terminal::show_cursor()?;
        Terminal::execute()?;
        Ok(())
    }

    fn draw_rows() -> Result<(), Error> {
        let Size{height, ..} = Terminal::size()?;
        for current_row in 0..height {
            Terminal::clear_line()?;
            Terminal::print("~")?;
            if current_row + 1 < height {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }

}

// note
// 定义一个空结构体 editor
// 实现default方法
// 将main函数里的逻辑提取到run方法
// 在main函数声明mod，改为使用整体逻辑

// 更专注于crossterm

// 使用 clippy 进行代码检测 cargo clippy -- -W clippy::all  -W clippy::pedantic
// 调整了格式化
// 使用 if let 简化

// 错误向上传递
// 使用?传递

// 使用 control + c 来控制退出
// 增加一个标识
// 可变mut self

// 初始化，raw mode，清屏
// 循环读入，评估，更新屏幕
// 结尾，禁用 raw mode，清屏，handle errors， goodbye
// Self 代指 impl 后面的类型
// 关联函数
// 效果：屏幕干净，ctrl + c 退出，打印goodbye

// queue!代替execute!，需手动flush
// 