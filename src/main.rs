#[allow(unused_macros)]
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

#[allow(unused_macros)]
macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };

    ($next:expr, mut $var:ident : $t:tt $($r:tt)*) => {
        let mut $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

#[allow(unused_macros)]
macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, [ $t:tt ]) => {
        {
            let len = read_value!($next, usize);
            (0..len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
        }
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, bytes) => {
        read_value!($next, String).into_bytes()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

#[derive(Debug, Copy, Clone, Eq, PartialOrd, PartialEq, Ord)]
enum Instruction {
    Increment,
    Decrement,
    PointerIncrement,
    PointerDecrement,
    Put,
    GetChar,
    Begin,
    End,
    Etc,
}

struct Interpreter {
    memory: [i32; 30000],
    pointer: usize,
    index: usize,
    instructions: Vec<Instruction>,
}

impl Interpreter {
    fn new(input: &str) -> Self {
        let mut process: Vec<Instruction> = Vec::new();
        for i in input.chars() {
            match i {
                '+' => process.push(Instruction::Increment),
                '-' => process.push(Instruction::Decrement),
                '>' => process.push(Instruction::PointerIncrement),
                '<' => process.push(Instruction::PointerDecrement),
                '.' => process.push(Instruction::Put),
                ',' => process.push(Instruction::GetChar),
                '[' => process.push(Instruction::Begin),
                ']' => process.push(Instruction::End),
                _ => process.push(Instruction::Etc),
            }
        }
        Interpreter {
            memory: [0; 30000],
            pointer: 0,
            index: 0,
            instructions: process,
        }
    }

    fn run(&mut self) -> () {
        while self.index < self.instructions.len() {
            let inst = self.instructions[self.index];
            self.execute(inst);
            self.index += 1;
        }
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Increment => self.memory[self.pointer] += 1,
            Instruction::Decrement => self.memory[self.pointer] -= 1,
            Instruction::PointerIncrement => self.pointer += 1,
            Instruction::PointerDecrement => self.pointer -= 1,
            Instruction::Put => print!("{}", (self.memory[self.pointer] as u8) as char),

            Instruction::GetChar => {
                input! {
                    n: i32,
                }
                self.memory[self.pointer] = n;
            }

            Instruction::Begin => {
                match self.memory[self.pointer] {
                    0 => {
                        let mut loop_counter = 1;
                        while loop_counter != 0 {
                            self.index += 1;
                            match self.instructions[self.index] {
                                Instruction::Begin => loop_counter += 1,
                                Instruction::End => loop_counter -= 1,
                                _ => (),
                            }
                        }
                    }
                    _ => ()
                }
            }

            Instruction::End => {
                match self.memory[self.pointer] {
                    0 => (),
                    _ => {
                        let mut loop_counter = 1;
                        while loop_counter > 0 {
                            self.index -= 1;
                            match self.instructions[self.index] {
                                Instruction::Begin => loop_counter -= 1,
                                Instruction::End => loop_counter += 1,
                                _ => (),
                            }
                        }
                    }
                }
            }

            _ => (),
        }
    }
}


fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn reads () -> String{
    let mut process: String = String::new();

    loop{

        let a: String = read();
        let b: &str = &a;
        match b{
            "end" => break,
            _ => process += &b,
        }
    }
    process
}
fn main() {
    println!();
    println!("Welcome to BrainF**k Interpreter with Rust!");
    println!("if your code is finished, Please enter 'end' nextLine.");
    println!();
    println!("Example)");
    println!("        input) +++++++++[->++++++++>+++++++++++>+++++<<<]>.>++.+++++");
    println!("               ++..+++.>-.------------.<++++++++.--------.+++.------.--------.>+.");
    println!("               end");
    println!("        output) Hello, world!");
    println!();
    println!("Please enter your code.");
    let process:String = reads();
    let mut inter = Interpreter::new(&process);
    inter.run();
    println!();
}



