const NUM_REGS: usize = 8;

// instruction décodée depuis une instruction machine
struct Instruction {
    instr_num: u32,
    imm1: u32,
    o1: u32,
    r1: u32,
    imm2: u32,
    o2: u32,
    r2: u32,
    a: u32,
    n: u32,
}

impl Instruction {
    // permet de décoder une instruction machine
    fn decode(instruction: u32) -> Self {
        Self {
            instr_num 	: (instruction & 0xF8000000) >> 27,
            imm1 		: (instruction & 0x04000000) >> 26,
            o1 			: (instruction & 0x03FFFFE0) >> 5,
            r1 			: (instruction & 0x07C00000) >> 22,
            imm2 		: (instruction & 0x00200000) >> 21,
            o2 			: (instruction & 0x001FFFE0) >> 5,
            r2 			:  instruction & 0x0000001F,
            a 			:  instruction & 0x003FFFFF,
            n 			:  instruction & 0x07FFFFFF,
        }
    }
}

// VM faisant tourner un programme donné
pub struct VM {
    regs: [u32; NUM_REGS],
    program: Vec<u32>,
    pc: u32,
}

impl VM {
    // création de la VM ainsi que ses registres
    pub fn new(program: &[u32]) -> Self {
        Self {
            regs: [0; NUM_REGS],
            program: Vec::from(program),
            pc: 0,
        }
    }

    // récupère la prochaine instruction du programme
    fn fetch(&mut self) -> u32 {
        let value = self.program[self.pc as usize];
        self.pc += 1;
        value
    }

    // évalue une instruction décodée
    fn eval(&mut self, instr: Instruction) -> bool {
        match instr.instr_num {
            0 => {
                // halt
                println!("halt");
                return false;
            }
            1 => {
                // registre sinon immediate
                if instr.imm2 == 0 {
                    println!("add r{} r{} r{}", instr.r1, instr.o2, instr.r2);
                    self.regs[instr.r2 as usize] = self.regs[instr.r1 as usize] + self.regs[instr.o2 as usize];
                } else {
                    println!("add r{} #{} r{}", instr.r1, instr.o2, instr.r2);
                    self.regs[instr.r2 as usize] = self.regs[instr.r1 as usize] + instr.o2;
                }
            }
            2 => {
                // registre sinon immediate
                if instr.imm2 == 0 {
                    println!("sub r{} r{} r{}", instr.r1, instr.o2, instr.r2);
                    self.regs[instr.r2 as usize] = self.regs[instr.r1 as usize] - self.regs[instr.o2 as usize];
                } else {
                    println!("sub r{} #{} r{}", instr.r1, instr.o2, instr.r2);
                    self.regs[instr.r2 as usize] = self.regs[instr.r1 as usize] - instr.o2;
                }
            }
            3 => {
                // registre sinon immediate
                if instr.imm2 == 0 {
                    println!("mul r{} r{} r{}", instr.r1, instr.o2, instr.r2);
                    self.regs[instr.r2 as usize] = self.regs[instr.r1 as usize] * self.regs[instr.o2 as usize];
                } else {
                    println!("mul r{} #{} r{}", instr.r1, instr.o2, instr.r2);
                    self.regs[instr.r2 as usize] = self.regs[instr.r1 as usize] * instr.o2;
                }
            }
            4 => {
                // registre sinon immediate
                if instr.imm2 == 0 {
                    println!("div r{} r{} r{}", instr.r1, instr.o2, instr.r2);
                    self.regs[instr.r2 as usize] = self.regs[instr.r1 as usize] / self.regs[instr.o2 as usize];
                } else {
                    println!("div r{} #{} r{}", instr.r1, instr.o2, instr.r2);
                    self.regs[instr.r2 as usize] = self.regs[instr.r1 as usize] / instr.o2;
                }
            }
            5 => {
                // registre sinon immediate
                if instr.imm2 == 0 {
                    println!("and r{} r{} r{}", instr.r1, instr.o2, instr.r2);
                    self.regs[instr.r2 as usize] = self.regs[instr.r1 as usize] & self.regs[instr.o2 as usize];
                } else {
                    println!("and r{} #{} r{}", instr.r1, instr.o2, instr.r2);
                    self.regs[instr.r2 as usize] = self.regs[instr.r1 as usize] & instr.o2;
                }
            }
            6 => {
                // registre sinon immediate
                if instr.imm2 == 0 {
                    println!("or r{} r{} r{}", instr.r1, instr.o2, instr.r2);
                    self.regs[instr.r2 as usize] = self.regs[instr.r1 as usize] | self.regs[instr.o2 as usize];
                } else {
                    println!("or r{} #{} r{}", instr.r1, instr.o2, instr.r2);
                    self.regs[instr.r2 as usize] = self.regs[instr.r1 as usize] | instr.o2;
                }
            }
            7 => {
                // registre sinon immediate
                if instr.imm2 == 0 {
                    println!("xor r{} r{} r{}", instr.r1, instr.o2, instr.r2);
                    self.regs[instr.r2 as usize] = self.regs[instr.r1 as usize] ^ self.regs[instr.o2 as usize];
                } else {
                    println!("xor r{} #{} r{}", instr.r1, instr.o2, instr.r2);
                    self.regs[instr.r2 as usize] = self.regs[instr.r1 as usize] ^ instr.o2;
                }
            }
            8 => {
                // registre sinon immediate
                if instr.imm2 == 0 {
                    println!("shl r{} r{} r{}", instr.r1, instr.o2, instr.r2);
                    self.regs[instr.r2 as usize] = self.regs[instr.r1 as usize] << self.regs[instr.o2 as usize];
                } else {
                    println!("shl r{} #{} r{}", instr.r1, instr.o2, instr.r2);
                    self.regs[instr.r2 as usize] = self.regs[instr.r1 as usize] << instr.o2;
                }
            }
            9 => {
                // registre sinon immediate
                if instr.imm2 == 0 {
                    println!("shr r{} r{} r{}", instr.r1, instr.o2, instr.r2);
                    self.regs[instr.r2 as usize] = self.regs[instr.r1 as usize] >> self.regs[instr.o2 as usize];
                } else {
                    println!("shr r{} #{} r{}", instr.r1, instr.o2, instr.r2);
                    self.regs[instr.r2 as usize] = self.regs[instr.r1 as usize] >> instr.o2;
                }
            }
            10 => {
                // registre sinon immediate
                if instr.imm2 == 0 {
                    println!("slt r{} r{} r{}", instr.r1, instr.o2, instr.r2);
                    self.regs[instr.r2 as usize] = if self.regs[instr.r1 as usize] < self.regs[instr.o2 as usize] { 1 } else { 0 };
                } else {
                    println!("slt r{} #{} r{}", instr.r1, instr.o2, instr.r2);
                    self.regs[instr.r2 as usize] = if self.regs[instr.r1 as usize] < instr.o2 { 1 } else { 0 };
                }
            }
            11 => {
                // registre sinon immediate
                if instr.imm2 == 0 {
                    println!("sle r{} r{} r{}", instr.r1, instr.o2, instr.r2);
                    self.regs[instr.r2 as usize] = if self.regs[instr.r1 as usize] <= self.regs[instr.o2 as usize] { 1 } else { 0 };
                } else {
                    println!("sle r{} #{} r{}", instr.r1, instr.o2, instr.r2);
                    self.regs[instr.r2 as usize] = if self.regs[instr.r1 as usize] <= instr.o2 { 1 } else { 0 };
                }
            }
            12 => {
                // registre sinon immediate
                if instr.imm2 == 0 {
                    println!("seq r{} r{} r{}", instr.r1, instr.o2, instr.r2);
                    self.regs[instr.r2 as usize] = if self.regs[instr.r1 as usize] == self.regs[instr.o2 as usize] { 1 } else { 0 };
                } else {
                    println!("seq r{} #{} r{}", instr.r1, instr.o2, instr.r2);
                    self.regs[instr.r2 as usize] = if self.regs[instr.r1 as usize] == instr.o2 { 1 } else { 0 };
                }
            }
            13 => { println!("TODO: loading from cache"); }
            14 => { println!("TODO: storing to cache"); }
            15 => {
                self.regs[instr.r2 as usize] = self.pc + 1;

                // registre sinon immediate
                if instr.imm1 == 0 {
                    println!("jmp r{} r{}", instr.o1, instr.r2);
                    self.pc = self.regs[instr.o1 as usize];
                } else {
                    println!("seq r{} #{} r{}", instr.r1, instr.o2, instr.r2);
                    self.pc = instr.o1;
                }
                // cette opération vaut 2 cycles (TODO)
            }
            16 => {
                println!("braz r{} #{}", instr.r1, instr.a);
                if self.regs[instr.r1 as usize] == 0 { self.pc = instr.a; }
                // cette opération vaut 2 cycles (TODO)
            }
            17 => {
                println!("branz r{} #{}", instr.r1, instr.a);
                if self.regs[instr.r1 as usize] != 0 { self.pc = instr.a; }
                // cette opération vaut 2 cycles (TODO)
            }
            18 => {
                println!("scall {}", instr.n);
                // todo: handle syscall
            }
            _ => {
                println!("instruction unknown");
                return false;
            }
        }
        true
    }

    // affiche les registres de façon lisible
    fn show_regs(&self) {
        let mut repr = String::from("regs = ");
        self.regs
            .iter()
            .for_each(|reg| repr.push_str(&format!("{:04X} ", reg)));
        println!("{}", repr);
    }

    // fait tourner la machine virtuelle
    pub fn run(&mut self) {
        loop {
            self.show_regs();
            let instr = self.fetch();
            let instr_dec = Instruction::decode(instr);
            if !self.eval(instr_dec) {
                break;
            }
        }
    }
}