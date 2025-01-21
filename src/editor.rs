use crossterm::event::{read, Event::Key, KeyCode::Char};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

pub struct Editor {}

impl Editor {
    pub fn default() -> Self {            
        Editor{}            
    }
    pub fn run(&self){
        enable_raw_mode().unwrap();
        loop {
            match read() {
                Ok(Key(event)) => {
                    println!("{event:?} \r");
                    if let Char(c) = event.code {
                        if c == 'q' {
                            break;
                        }
                    }
                },
                Err(err) => println!("Error: {err}"),
                _ => ()
            }
        }
        disable_raw_mode().unwrap(); 
    }
}

// note
// 定义一个空结构体 editor
// 实现default方法
// 将main函数里的逻辑提取到run方法
// 在main函数声明mod，改为使用整体逻辑

// 更专注于crossterm

// 使用 clippy 进行代码检测
// 调整了格式化
// 使用 if let 简化