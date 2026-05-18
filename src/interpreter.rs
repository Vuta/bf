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
    ip: usize,
    dp: usize,
    ins: Vec<Ins>,
    jmp_map: HashMap<usize, usize>,
    tape: [u8; 30_000],
    done: bool,
    output_buf: Vec<u8>,
}

impl Interpreter {
    pub fn default() -> Self {
        Self {
            ip: 0,
            dp: 0,
            ins: Vec::new(),
            jmp_map: HashMap::new(),
            tape: [0; 30_000],
            done: false,
            output_buf: Vec::new(),
        }
    }

    pub fn reset(&mut self) {
        let n = Interpreter::default();
        *self = n;
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
                _ => {}
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
    }

    pub fn step(&mut self) -> std::io::Result<()> {
        if self.ip >= self.ins.len() {
            return Ok(());
        }

        match self.ins[self.ip] {
            Ins::RMov => self.dp += 1,
            Ins::LMov if self.dp == 0 => panic!("out of bound"),
            Ins::LMov => self.dp -= 1,
            Ins::Inc => self.tape[self.dp] = self.tape[self.dp].wrapping_add(1),
            Ins::Dec => self.tape[self.dp] = self.tape[self.dp].wrapping_sub(1),
            Ins::Out => {
                self.output_buf.push(self.tape[self.dp]);
                // print!("{}", self.tape[self.dp] as char),
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

        Ok(())
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        while self.ip < self.ins.len() {
            match self.ins[self.ip] {
                Ins::RMov => self.dp += 1,
                Ins::LMov if self.dp == 0 => panic!("out of bound"),
                Ins::LMov => self.dp -= 1,
                Ins::Inc => self.tape[self.dp] = self.tape[self.dp].wrapping_add(1),
                Ins::Dec => self.tape[self.dp] = self.tape[self.dp].wrapping_sub(1),
                Ins::Out => {
                    self.output_buf.push(self.tape[self.dp]);
                    // print!("{}", self.tape[self.dp] as char),
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
        }

        self.done = true;

        Ok(())
    }

    pub fn current_cell(&self) -> usize {
        self.dp
    }

    pub fn get_cell_value(&self, i: usize) -> u8 {
        self.tape[i]
    }

    pub fn is_finish(&self) -> bool {
        self.done
    }

    pub fn output(&self) -> &[u8] {
        &self.output_buf[..]
    }
}
