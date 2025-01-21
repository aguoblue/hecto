
pub fn test_tuple_and_loop() {
    let somesize = (10, 20);
    dbg!(somesize);
    dbg!(somesize.0);
    dbg!(somesize.1);
    
    for number in 0..10 {
        dbg!(number);
    }
}

pub fn draw_row(){
    let (width, height) = crossterm::terminal::size().unwrap();
    for pos in 0..height {
        crossterm::cursor::MoveTo(0, pos);
        println!("~");
    }
    crossterm::cursor::MoveTo(0, 0);
}

