use std::collections::HashMap;
use std::io::Read;

#[derive(Debug, PartialEq)]
enum Ins {
    RMov,   // right mov data pointer
    LMov,   // left mov data pointer
    Inc,    // increase byte pointed by data pointer
    Dec,    // decrease byte pointed by data pointer
    Out,    // output the byte at data pointer
    In,     // read one byte into the byte pointed by data pointer
    FJmpZ,  // jump instruction pointer forward if zero
    BJmpNz, // jump instruction pointer backward if non-zero
}

#[derive(Debug)]
pub struct Interpreter {
    pub status: Status,
    ip: usize,
    dp: usize,
    ins: Vec<Ins>,
    positions: Vec<(usize, usize)>,
    jmp_map: HashMap<usize, usize>,
    tape: [u8; 30_000],
    output_buf: Vec<u8>,
    current_row: usize,
    current_col: usize,
}

impl Interpreter {
    pub fn default() -> Self {
        Self {
            ip: 0,
            dp: 0,
            ins: Vec::new(),
            positions: Vec::new(),
            jmp_map: HashMap::new(),
            tape: [0; 30_000],
            status: Status::New,
            output_buf: Vec::new(),
            current_row: 0,
            current_col: 0,
        }
    }

    pub fn reset(&mut self) {
        let n = Interpreter::default();
        *self = n;
    }

    pub fn tokenize(&mut self, source: &str) {
        for c in source.as_bytes() {
            match c {
                b'>' => {
                    self.ins.push(Ins::RMov);
                    self.store_ins_position();
                }
                b'<' => {
                    self.ins.push(Ins::LMov);
                    self.store_ins_position();
                }
                b'+' => {
                    self.ins.push(Ins::Inc);
                    self.store_ins_position();
                }
                b'-' => {
                    self.ins.push(Ins::Dec);
                    self.store_ins_position();
                }
                b'.' => {
                    self.ins.push(Ins::Out);
                    self.store_ins_position();
                }
                b',' => {
                    self.ins.push(Ins::In);
                    self.store_ins_position();
                }
                b'[' => {
                    self.ins.push(Ins::FJmpZ);
                    self.store_ins_position();
                }
                b']' => {
                    self.ins.push(Ins::BJmpNz);
                    self.store_ins_position();
                }
                b';' => {
                    self.current_row += 1;
                    self.current_col = 0;
                }
                _ => self.current_col += 1,
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
                },
                _ => {}
            }
        }

        if !st.is_empty() {
            panic!("'[' must be closed");
        }

        if self.ins.len() == 0 {
            self.status = Status::Done;
        }
    }

    pub fn step(&mut self) -> std::io::Result<()> {
        self.status = Status::InProgress;

        match self.ins[self.ip] {
            Ins::RMov => self.dp += 1,
            Ins::LMov if self.dp == 0 => panic!("out of bound"),
            Ins::LMov => self.dp -= 1,
            Ins::Inc => self.tape[self.dp] = self.tape[self.dp].wrapping_add(1),
            Ins::Dec => self.tape[self.dp] = self.tape[self.dp].wrapping_sub(1),
            Ins::Out => {
                self.output_buf.push(self.tape[self.dp]);
            }
            Ins::In => {
                let mut b = [0u8; 1];
                std::io::stdin().read_exact(&mut b)?;
                self.tape[self.dp] = b[0];
            }
            Ins::FJmpZ if self.tape[self.dp] == 0 => self.ip = self.jmp_map[&self.ip],
            Ins::BJmpNz if self.tape[self.dp] != 0 => self.ip = self.jmp_map[&self.ip],
            _ => {}
        };

        self.ip += 1;
        if self.ip >= self.ins.len() {
            self.status = Status::Done;
        }

        Ok(())
    }

    pub fn current_cell(&self) -> usize {
        self.dp
    }

    pub fn current_position(&self) -> (u16, u16) {
        let i = if self.ip >= self.ins.len() {
            self.ins.len() - 1
        } else {
            self.ip
        };

        let p = self.positions[i];
        (p.0 as u16, p.1 as u16)
    }

    pub fn get_cell_value(&self, i: usize) -> u8 {
        self.tape[i]
    }

    pub fn output(&self) -> &[u8] {
        &self.output_buf[..]
    }

    fn store_ins_position(&mut self) {
        self.positions.push((self.current_row, self.current_col));
        self.current_col += 1;
    }
}

#[derive(Debug, PartialEq)]
pub enum Status {
    New,
    InProgress,
    Done,
}
