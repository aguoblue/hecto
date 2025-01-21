use core::panic;
use crossterm::event::{read, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn default() -> Self {            
        Editor{should_quit: false}            
    }
    pub fn run(&mut self){
        if let Err(err) = self.repl() {
            panic!("{err:#?}");
        }
        print!("Goodbye.\r\n");
    }

    fn repl(&mut self) -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        loop {
            if let Key(KeyEvent {
                code, modifiers, kind, state
            }) = read()?
            {
                println!("Code: {code:?} Modifiers: {modifiers:?} Kind: {kind:?} State: {state:?} \r");
                match code {
                    Char('c') if modifiers == KeyModifiers::CONTROL => {
                        self.should_quit = true;
                    }
                    _ => (),
                }
            }
            if self.should_quit {
                break;
            }
        }
        disable_raw_mode()?;
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