use std::collections::HashSet;
use shared::args::read_file_from_arg;


type Value = u64;
type Program = Vec<Value>;

#[derive(Debug, Clone)]
struct Computer {
    a: Value,
    b: Value,
    c: Value,
    instruction: Value,
    output: Vec<Value>,
}

impl Computer {
    fn new(a: Value, b: Value, c: Value) -> Self {
        Self {
            a,
            b,
            c,
            instruction: 0,
            output: Vec::new(),
        }
    }

    fn execute(&mut self, program: &Program) {
        self.instruction = 0;

        loop {
            if self.instruction as usize > program.len() - 2 {
                break;
            }

            let op = program[self.instruction as usize];
            let value = program[(self.instruction + 1) as usize];

            match op {
                0 => { self.adv(value); self.instruction += 2 },
                1 => { self.bxl(value); self.instruction += 2 },
                2 => { self.bst(value); self.instruction += 2 },
                3 => { if !self.jnz(value) { self.instruction += 2 } },
                4 => { self.bxc(value); self.instruction += 2 },
                5 => { self.out(value); self.instruction += 2 },
                6 => { self.bdv(value); self.instruction += 2 },
                7 => { self.cdv(value); self.instruction += 2 },
                _ => { panic!("unknown command") }
            }
        }
    }

    fn format_output(&self) -> String {
        self.output
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }

    fn combo_operand(&self, value: Value) -> Value {
        if value <= 3 {
            value
        } else if value == 4 {
            self.a
        } else if value == 5 {
            self.b
        } else if value == 6 {
            self.c
        } else {
            panic!("wtf?")
        }
    }

    fn adv(&mut self, value: Value) {
        let base: Value = 2;
        self.a = self.a / (base.pow(self.combo_operand(value).try_into().unwrap()))
    }

    fn bxl(&mut self, value: Value) {
        self.b = self.b ^ value
    }

    fn bst(&mut self, value: Value) {
        self.b = self.combo_operand(value) % 8
    }

    fn jnz(&mut self, value: Value) -> bool {
        if self.a != 0 {
            self.instruction = value;
            return true;
        }

        false
    }

    fn bxc(&mut self, _value: Value) {
        self.b = self.b ^ self.c
    }

    fn out(&mut self, value: Value) {
        self.output.push(self.combo_operand(value) % 8)
    }

    fn bdv(&mut self, value: Value) {
        let base: Value = 2;
        self.b = self.a / (base.pow(self.combo_operand(value).try_into().unwrap()))
    }
    
    fn cdv(&mut self, value: Value) {
        let base: Value = 2;
        self.c = self.a / (base.pow(self.combo_operand(value).try_into().unwrap()))
    }
}


fn parse_data(data: String) -> (Computer, Program) {
    let lines: Vec<&str> = data.lines().collect();

    let computer = Computer::new(
        lines[0].split_once(": ").expect("could not parse reg A").1.parse().expect("could not parse reg A value"),
        lines[1].split_once(": ").expect("could not parse reg B").1.parse().expect("could not parse reg B value"),
        lines[2].split_once(": ").expect("could not parse reg C").1.parse().expect("could not parse reg C value"),
    );

    let program = lines[4]
        .split_once(": ").expect("could not parse program")
        .1
        .split(",")
        .map(|v| v.parse().expect("could not parse op"))
        .collect()
    ;

    (computer, program)
}

fn part2(computer: &Computer, program: &Program, new_a: Value, program_index: usize) -> Value {
    let mut possibles: HashSet<Value> = HashSet::new();

    for i in 0..8 {
        let newer_a = (new_a << 3) | i;
        let mut test_computer = computer.clone();
        test_computer.a = newer_a;
        test_computer.execute(program);

        if test_computer.output == program[(program.len() - program_index)..] {
            if test_computer.output == *program {
                possibles.insert(newer_a);
            } else {
                let p = part2(computer, program, newer_a, program_index + 1);
                if p > 0 {
                    possibles.insert(p);
                }
            }
        }
    }

    *possibles.iter().min().unwrap_or(&0)
}


fn main() {
    let (computer, program) = parse_data(read_file_from_arg(1));

    let mut p1_computer = computer.clone();
    p1_computer.execute(&program);
    println!("Part 1 Output: {}", p1_computer.format_output());

    let new_a = part2(&computer, &program, 0, 1);
    println!("Part 2 Register A: {}", new_a);
}
