
use std::collections::HashMap;
use std::ascii::AsciiExt;
use std::io::Read;

#[derive(Debug)]
pub enum Instruction {
    MoveLeft,
    MoveRight,
    IncCell,
    DecCell,
    LBrace,
    RBrace,
    Stdout,
    Stdin,
    Nop,
    Zero
}

pub trait ToInstruction {
    fn to_ir(self) -> Instruction;
}

impl ToInstruction for char {
    fn to_ir(self) -> Instruction {
        match self {
            '+' => Instruction::IncCell,
            '-' => Instruction::DecCell,
            '[' => Instruction::LBrace,
            ']' => Instruction::RBrace,
            '.' => Instruction::Stdout,
            ',' => Instruction::Stdin,
            '<' => Instruction::MoveLeft,
            '>' => Instruction::MoveRight,
            '|' => Instruction::Zero,
            _ => Instruction::Nop
        }
    }
}

#[derive(Debug)]
pub enum IR {
    Sym(Instruction, usize),
    CntSym(usize, Instruction, usize),
    Eof
}

pub struct BrainFuck {
    src: String,
    paren_map: HashMap<usize, usize>,
    cell: Vec<u8>,
    ptr: usize,
    ir_ptr: usize
}

impl BrainFuck {
    pub fn new(src: &str) -> Self {
        let cell = vec![0u8; 30_000];
        // Run length encdoing
        let src_encoded = rle(src);
        // Perform optimize zero loop
        let src_encoded = optimize_zero_loop(&src_encoded);
        // Build paren indices
        let paren_map = process_parens(&src_encoded);

        BrainFuck {
            // contains the source code
            src: src_encoded,//.to_string(),
            // contains the paren map
            paren_map: paren_map,
            // contains the data cell
            cell: cell,
            // points to the optmized IR
            ir_ptr: 0,
            // points into the cell
            ptr: 0,
        }
    }

    pub fn eval(&mut self) -> Result<String, ()> {
        let vec_src = self.src.chars().collect::<Vec<char>>();
        let mut out = String::new();
        loop {
            match parse_num_idx(&vec_src, &mut self.ir_ptr) {
                IR::CntSym(count,symbol, updated_ptr) => {
                    self.ir_ptr = updated_ptr;
                    match symbol {
                        Instruction::IncCell => self.cell[self.ptr] += count as u8,
                        Instruction::DecCell => self.cell[self.ptr] -= count as u8,
                        Instruction::MoveRight => self.ptr += count,
                        Instruction::MoveLeft => self.ptr -= count,
                        Instruction::Stdin => self.cell[self.ptr] = read_input().unwrap() as u8,
                        _ => unreachable!()
                    }
                }
                IR::Sym(symbol, updated_ptr) => {
                    self.ir_ptr = updated_ptr;
                    match symbol {
                        Instruction::Stdout => out.push(self.cell[self.ptr] as u8 as char),
                        Instruction::Zero => self.cell[self.ptr] = 0,
                        Instruction::IncCell => self.cell[self.ptr] += 1,
                        Instruction::DecCell => self.cell[self.ptr] -= 1,
                        Instruction::MoveRight => self.ptr += 1,
                        Instruction::MoveLeft => self.ptr -= 1,
                        Instruction::LBrace => {
                            if self.cell[self.ptr] == 0 {
                                let past_ptr = self.paren_map.get(&(self.ir_ptr-1));
                                self.ir_ptr = *past_ptr.unwrap() + 1;
                            }
                        },
                        Instruction::RBrace => {
                            if self.cell[self.ptr] != 0 {
                                let prev_ptr = self.paren_map.get(&(self.ir_ptr-1));
                                self.ir_ptr = *prev_ptr.unwrap() + 1;
                            }
                        },
                        Instruction::Stdin => {
                            self.cell[self.ptr] = read_input().unwrap();
                        }
                        Instruction::Nop => {},
                    }
                }
                IR::Eof => break,
            }
        }
        Ok(out)
    }
}

pub fn process_parens(c: &str) -> HashMap<usize, usize> {
    let mut paren_map = HashMap::new();
    let mut chars = c.chars().enumerate();
    let mut paren_stack = vec![];
    loop {
        match chars.next() {
            Some((idx, '[')) => paren_stack.push(idx),
            Some((idx, ']')) => {
                let left_idx = paren_stack.pop().unwrap();
                let right_idx = idx;
                paren_map.insert(left_idx, right_idx);
                paren_map.insert(right_idx, left_idx);
            },
            Some((_, _)) => {}
            None => break
        }
    }
    assert_eq!(paren_stack.len(), 0);
    paren_map
}

fn read_input() -> Result<u8, ()> {
    use io::stdin;
    let mut stdin = stdin();
    let mut buf = [0u8;1];
    let _ = stdin.read_exact(&mut buf);
    Ok(buf[0].to_ascii_lowercase())
}

pub fn optimize_zero_loop(s: &str) -> String {
    s.replace("[-]", "|")
}

pub fn parse_num_idx(s: &Vec<char>, ir_ptr: &mut usize) -> IR {
    let mut ir_ptr = *ir_ptr;
    let mut num = String::new();

    loop {
        if ir_ptr == s.len() {
            return IR::Eof;
        }
        while let true = s[ir_ptr].is_numeric() {
            num.push(s[ir_ptr]);
            ir_ptr += 1;
        }
        if !s[ir_ptr].is_numeric() {
            let raw_ir = if let Ok(n) = num.parse() {
                IR::CntSym(n, s[ir_ptr].to_ir(), ir_ptr+1)
            } else {
                IR::Sym(s[ir_ptr].to_ir(), ir_ptr+1)
            };
            return raw_ir;
        }
    }
}

// Run Length decoding
// pub fn elr(encoded: &str) -> String {
//         let en_chars = encoded.chars();
//         let mut decoded = String::new();
//         let mut peekable = en_chars.peekable();
//         let mut ir_ptr = 0;
//         loop {
//             match parse_num_idx(&mut peekable, &mut ir_ptr) {
//                     IR::Eof => break,
//                     IR::CntSym(count, symbol, _) => {
//                         for _ in 0..count {
//                             decoded.push(symbol);
//                         }
//                     },
//                     IR::Sym(symbol, _) => decoded.push(symbol)
//             }
//         }
//         decoded
// }

pub fn rle(e: &str) -> String {
    if e.len() == 0 {
        e.to_string()
    } else {
        let mut first_sym = e.chars().peekable().next().unwrap();
        let mut run_length = e.chars();
        let mut encoded = String::with_capacity(e.len());
        let mut current_cnt = 0;
        loop {
            if let Some(a) = run_length.next() {
                if a == first_sym {
                    current_cnt += 1;
                } else {
                    if current_cnt <= 2 || first_sym == '[' || first_sym == ']' {
                        for _ in 0..current_cnt {
                            encoded.push(first_sym);
                        }
                    } else {
                        encoded.extend((current_cnt.to_string() + &first_sym.to_string()).chars());
                    }
                    first_sym = a;
                    current_cnt = 1;
                }
            } else {
                if current_cnt <= 2 || first_sym == '[' || first_sym == ']' {
                    for _ in 0..current_cnt {
                        encoded.push(first_sym);
                    }
                } else {
                    encoded.extend((current_cnt.to_string()+&first_sym.to_string()).chars());
                }
                break;
            }
        }
        encoded
    }
}

#[cfg(test)]
mod tests {
    use engine::{process_parens, rle, optimize_zero_loop, parse_num, parse_num_idx};
    use super::BrainFuck;
    use cell::IR;
    #[test]
    fn paren_idx() {
        let src = "++[.[-]]";
        let map = process_parens(src);
        assert_eq!(map.get(&6).unwrap(), &4);
        assert_eq!(map.get(&2).unwrap(), &7);
        assert_eq!(map.get(&4).unwrap(), &6);
        assert_eq!(map.get(&7).unwrap(), &2);
    }

    #[test]
    fn cell_clear() {
        let a = "[[-]]";
        let res = optimize_zero_loop(a);
        assert_eq!(res, "[|]");
    }

    #[test]
    fn hello_world_program() {
        let src = "++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.";
        let mut b = BrainFuck::new(src);
        assert_eq!(Ok("Hello World!\n".to_string()), b.eval());
    }

    #[test]
    fn test_parse_num() {
        let mut s = "3+[----]".chars().collect::<Vec<char>>();
        let mut ir_ptr = 0;
        let mut parsed = String::new();
        loop {
            match parse_num_idx(&mut s, &mut ir_ptr) {
                IR::Eof => break,
                IR::CntSym(count,symbol, ptr) => {
                    ir_ptr = ptr;
                    parsed.push_str(&format!("{}{}", count, symbol));
                    },
                IR::Sym(symbol, ptr) => {
                    ir_ptr = ptr;
                    parsed.push_str(&format!("{}", symbol));
                }
            }
        }
        assert_eq!(parsed, s.iter().collect::<String>());
    }

}
