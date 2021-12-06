use std::{collections::HashSet, fmt::Debug, marker::PhantomData};

pub trait Instruction<RegisterType, ExtraState, const NR: usize> where RegisterType: Debug, ExtraState: Debug + State<RegisterType, NR> {
    fn from_asm(s: &str) -> Self;
    fn execute(&self, comp: &mut Computer<RegisterType, ExtraState, NR>);
}

pub trait State<RegisterType, const NR: usize> where RegisterType: Debug, Self: Debug + Sized {
    fn before_ins(_comp: &mut Computer<RegisterType, Self, NR>) {}
    fn after_ins(_comp: &mut Computer<RegisterType, Self, NR>) {}
}

pub struct InstructionSet<I, R, E, const NR: usize> where I: Instruction<R, E, NR>, R: Debug, E: Debug + State<R, NR> {
    instructions: Vec<I>,
    register: PhantomData<R>,
    extra_data: PhantomData<E>
}

impl<I, R, E, const NR: usize> InstructionSet<I, R, E, NR> where I: Instruction<R, E, NR>, R: Debug, E: Debug + State<R, NR> {
    pub fn new_from_asm(s: &str) -> Self {
        let instructions = s.split("\n").map(|s| s.trim()).filter(|s| s.len() > 0).map(|s| I::from_asm(s)).collect();
        return Self {
            instructions,
            register: PhantomData,
            extra_data: PhantomData
        }
    }
    
    pub fn get(&self, i: usize) -> Option<&I> {
        return self.instructions.get(i);
    }
}

#[derive(Debug)]
pub struct Computer<RegisterType, ExtraState, const NR: usize> where RegisterType: Debug, ExtraState: Debug + State<RegisterType, NR> {
    jumped: bool,
    pub registers: [RegisterType; NR],
    pub instruction_ptr: usize,
    pub extra_state: ExtraState,
    pub break_points: HashSet<usize>,
    pub halted: bool
}

impl<RegisterType, ExtraState, const NR: usize>  Computer<RegisterType, ExtraState, NR> where RegisterType: Debug, ExtraState: Debug + State<RegisterType, NR> {
    pub fn new(registers: [RegisterType;NR], extra_state: ExtraState) -> Self {
        return Computer {
            jumped: false,
            registers,
            instruction_ptr: 0,
            extra_state,
            break_points: HashSet::new(),
            halted: false
        }
    }

    pub fn add_breakpoint(&mut self, n: usize) {
        self.break_points.insert(n);
    }

    pub fn rel_jump(&mut self, jmp: isize) {
        if jmp < 0 && jmp.abs() as usize > self.instruction_ptr {
            panic!();
        }

        if jmp == 0 {
            return;
        }

        self.jumped = true;
        self.instruction_ptr = (self.instruction_ptr as isize + jmp) as usize;
    }

    pub fn abs_jump(&mut self, jmp: usize) {
        if jmp == self.instruction_ptr {
            return;
        }

        self.instruction_ptr = jmp;
        self.jumped = true;
    }

    pub fn halt(&mut self) {
        self.halted = true;
    }

    pub fn is_halted(&self) -> bool {
        return self.halted;
    }

    pub fn get_register(&self, reg: usize) -> &RegisterType {
        if reg >= NR {
            panic!();
        }

        return &self.registers[reg];
    }

    pub fn set_register(&mut self, reg: usize, val: RegisterType) {
        if reg >= NR {
            panic!();
        }

        self.registers[reg] = val;
    }

    pub fn step<I: Instruction<RegisterType, ExtraState, NR>>(&mut self, instructions: &InstructionSet<I, RegisterType, ExtraState, NR>) {
        if self.break_points.contains(&self.instruction_ptr) {
            println!("BREAK! PC: {}, Registers: {:?}", self.instruction_ptr, self.registers);
        }

        let instruction = instructions.get(self.instruction_ptr);
        if instruction.is_none() || self.halted {
            self.halt();
            return;
        }

        let instruction = instruction.unwrap();
        ExtraState::before_ins(self);
        instruction.execute(self);
        ExtraState::after_ins(self);

        if !self.jumped {
            self.instruction_ptr += 1;
        }
        self.jumped = false;
    }

    pub fn run_to_completion<I: Instruction<RegisterType, ExtraState, NR>>(&mut self, instructions: &InstructionSet<I, RegisterType, ExtraState, NR>) {
        while !self.halted {
            self.step(instructions);
        }
    }

    pub fn run_to_completion_with_timeout<I: Instruction<RegisterType, ExtraState, NR>>(&mut self, instructions: &InstructionSet<I, RegisterType, ExtraState, NR>, timeout: usize) -> Option<usize> {
        let mut steps = 0;
        while !self.halted {
            self.step(instructions);
            steps += 1;
            if steps >= timeout {
                return None;
            }
        }

        return Some(steps);
    }
}