use std::io;
use std::io::Read;
use std::fs::OpenOptions;
const CELL_SIZE:usize = 30000;
fn main() {
    let mut cell:[u8;CELL_SIZE] = [0;CELL_SIZE];
    let mut pointer = 0;
    let mut buffer = [0;1];
    let mut file = OpenOptions::new().read(true).open("src.fuck").unwrap();
    loop {
    match file.read(&mut buffer) {
        Ok(ins) => {
            if(ins==0){break;};
            let token = buffer[0] as char;
            match token {
                '+' => cell[pointer]+=1,
                '-' => cell[pointer]-=1,
                '>' => pointer+=1,
                '<' => pointer-=1,
                '[' => {let mut skip_buf = [0;1];
                    if cell[pointer] == 0 {
                    let _ = file.read(&mut skip_buf);
                    while( skip_buf[0] as char !=']' ) {
                        let _ = file.read(&mut skip_buf);
                    }
                } else { /* Todo*/ }}
                '.' => {print!("{:?}",cell[pointer] as char);},
                ',' => {let input: Option<u8> = std::io::stdin()
                        .bytes() 
                        .next()
                        .and_then(|result| result.ok())
                        .map(|byte| byte as u8);println!("{:?}",input);}
                _ => {},
            }
        },
        Err(why) => {print!("{:?}",why );},
    }
}
}