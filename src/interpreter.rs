use std::io::Read;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum Ins {
    RMov, // right mov data pointer 
    LMov, // left mov data pointer
    Inc, // increase byte pointed by data pointer
    Dec, // decrease byte pointed by data pointer
    Out, // output the byte at data pointer
    In, // read one byte into the byte pointed by data pointer
    FJmpZ, // jump instruction pointer forward if zero
    BJmpNz, // jump instruction pointer backward if non-zero
}

#[derive(Debug)]
pub struct Interpreter {
    ip: usize,
    dp: usize,
    ins: Vec<Ins>,
    jmp_map: HashMap<usize, usize>,
    tape: [u8; 30_000],
}

impl Interpreter {
    pub fn default() -> Self {
        Self {
            ip: 0,
            dp: 0,
            ins: Vec::new(),
            jmp_map: HashMap::new(),
            tape: [0; 30_000],
        }
    }

    pub fn tokenize(&mut self, source: &str) {
        for c in source.as_bytes() {
            match c {
                b'>' => self.ins.push(Ins::RMov),
                b'<' => self.ins.push(Ins::LMov),
                b'+' => self.ins.push(Ins::Inc),
                b'-' => self.ins.push(Ins::Dec),
                b'.' => self.ins.push(Ins::Out),
                b',' => self.ins.push(Ins::In),
                b'[' => self.ins.push(Ins::FJmpZ),
                b']' => self.ins.push(Ins::BJmpNz),
                _ => {},
            }
        }

        let mut st = Vec::new();
        for (i, ins) in self.ins.iter().enumerate() {
            match ins {
                Ins::FJmpZ => st.push((Ins::FJmpZ, i)),
                Ins::BJmpNz => match st.pop() {
                    Some((_, id)) => {
                        self.jmp_map.insert(id, i);
                        self.jmp_map.insert(i, id);
                    }
                    None => panic!("non-matching bracket at {i}"),
                }
                _ => {}
            }
        }

        if !st.is_empty() {
            panic!("'[' must be closed");
        }
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        while self.ip < self.ins.len() {
            match self.ins[self.ip] {
                Ins::RMov => self.dp += 1,
                Ins::LMov if self.dp == 0 => panic!("out of bound"),
                Ins::LMov => self.dp -= 1,
                Ins::Inc => self.tape[self.dp] = self.tape[self.dp].wrapping_add(1),
                Ins::Dec => self.tape[self.dp] = self.tape[self.dp].wrapping_sub(1),
                Ins::Out => print!("{}", self.tape[self.dp] as char),
                Ins::In => {
                    let mut b = [0u8; 1];
                    std::io::stdin().read_exact(&mut b)?;
                    self.tape[self.dp] = b[0];
                }
                Ins::FJmpZ if self.tape[self.dp] == 0 => self.ip = self.jmp_map[&self.ip],
                Ins::BJmpNz if self.tape[self.dp] != 0 => self.ip = self.jmp_map[&self.ip],
                _ => {},
            };

            self.ip += 1;
        }

        Ok(())
    }
}
