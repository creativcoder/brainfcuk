
use std::io::Read;
use std::char;
use std::ascii::AsciiExt;
use std::io::Cursor;
use std::str::Chars;
use std::iter::Peekable;
use std::collections::HashMap;

const CELL_SIZE: usize = 30000;

macro_rules! map {
    () => (HashMap::new())
}

enum Instruction {
    MoveLeft,
    MoveRight,
    IncCell,
    DecCell,
    JmpFwd,
    JmpBck,
    Stdout,
    Stdin,
    Nop
}

impl Brain {
    pub fn new() -> Self {
        Brain {
            cell: [0;30000],
            ptr: 0,
            src: vec![],
            ins_ptr: 0,
            paren_map: map!()
        }
    }

    pub fn eval<R: Read>(&mut self, mut buf: R) {
        let mut inp = String::new();
        buf.read_to_string(&mut inp);
        self.src = inp.chars().collect::<Vec<char>>();
        self.ins_ptr = 0;
        loop {
            if self.ins_ptr == self.src.len() {
                break;
            }
            match self.src[self.ins_ptr] {
                a => {
                    let ins = self.parse_instruction(a).unwrap();
                    self.exec(ins);
                }
            }
            self.incr_ptr();
        }
    }

    pub fn decr_ptr(&mut self) {
        if self.ins_ptr != 0 {
            self.ins_ptr -= 1;
        }
    }

    pub fn incr_ptr(&mut self) {
        if self.ins_ptr < self.src.len() {
            self.ins_ptr += 1;
        }
    }

    pub fn exec(&mut self, ins: Instruction) {
        match ins {
            Instruction::DecCell => {
                if self.cell[self.ptr] != 0 {
                    self.cell[self.ptr] -= 1;
                }
            },
            Instruction::IncCell => {
                self.cell[self.ptr] += 1;
            },
            Instruction::JmpBck => {
                if self.cell[self.ptr] != 0 {
                    while self.src[self.ins_ptr] != '[' && self.ins_ptr != 0 {
                        self.decr_ptr();
                    }
                }
            }
            Instruction::JmpFwd => {
                println!("SRC PTR {:?}", self.src[self.ins_ptr]);
                unimplemented!();
                if self.cell[self.ptr] == 0 {
                    while self.src[self.ptr] != ']' {
                        self.incr_ptr();
                    }
                }
            }
            Instruction::MoveLeft => {
                if self.ptr != 0 {
                    self.ptr -= 1;
                }
            },
            Instruction::MoveRight => {
                self.ptr += 1;
            },
            Instruction::Stdout => {
                if let Some(c) = char::from_u32(self.cell[self.ptr] as u32) {
                    print!("{}", c);
                } else {
                    print!("Character not a valid unicode value");
                }
                },
            Instruction::Stdin => {
                self.cell[self.ptr] = read_input().unwrap();
            }
            Instruction::Nop => {}
        }
    }

    pub fn pre_process(&mut self, src:Cursor<String>) -> String {
        let mut paren_stack = vec![];
        let src = src.into_inner();
        let mut loop_idx = vec![];
        for i in src.chars().enumerate() {
            if i.1 == '[' {
                paren_stack.push(i.0);
            } else if i.1 == ']' {
                let left_brack_idx = paren_stack.pop();
                loop_idx.push((left_brack_idx, i.0));
            }
        }

        let mut first_sym = src.chars().peekable().next().unwrap();
        let mut run_length = src.chars();
        let mut encoded = String::new();
        let mut current_cnt = 0;
        loop {
            if let Some(a) = run_length.next() {
                if a == first_sym {
                    current_cnt += 1;
                } else {
                    if current_cnt <= 2 {
                        encoded.push(a);
                    } else {
                        encoded.extend((current_cnt.to_string() + &first_sym.to_string()).chars());
                    }
                    first_sym = a;
                    current_cnt = 1;
                }
            } else {
                if current_cnt <= 2 {
                    encoded.push(first_sym);
                } else {
                    encoded.extend((current_cnt.to_string()+&first_sym.to_string()).chars());
                }
                break;
            }
        }

        assert_eq!(paren_stack.len(), 0);
        encoded
    }

    pub fn expand(&mut self) -> String {
        let expanded = String::new();
        expanded
    }

    pub fn parse_instruction(&mut self, ins_buff: char) -> Result<Instruction, ()> {
        Ok(match ins_buff {
            '>' => Instruction::MoveRight,
            '<' => Instruction::MoveLeft,
            '+' => Instruction::IncCell,
            '-' => Instruction::DecCell,
            '[' => Instruction::JmpFwd,
            ']' => Instruction::JmpBck,
            '.' => Instruction::Stdout,
            ',' => Instruction::Stdin,
            a => Instruction::Nop
        })
    }

    pub fn cell_val(&self) {
        println!("{:?}", self.ptr);
    }
}

#[cfg(test)]
mod tests {
    use super::Brain;
    use std::io::Cursor;

    use super::IR;
    use engine::{elr, rle, parse_num};


    #[test]
    fn test_some() {
        let mut b = Brain::new();
        // b.optimize();
        // b.init()
        let mut v = Cursor::new(String::from("-[>[->]++[-<+]-]"));
        b.eval(v);
    }

    #[test]
    fn comp_decomp_bf_src() {
        let mut c = "++++++>>>>>>+++.";
        let encoded = rle(c);
        let decoded = elr(&encoded);
        assert_eq!(c, decoded);
    }

    // #[test]
    // fn saved_indices() {
    //     let mut b = Brain::new();
    //     let mut v = Cursor::new(String::from("["));
    //     b.eval(v);
    // }
}