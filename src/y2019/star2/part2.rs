use super::part1::IntcodeProgram;

pub fn run(input: &str) -> i32 {
    for x in 0..99 {
        for y in 0..99 {
            let mut program = IntcodeProgram::new(input);
            program.tape[1] = x;
            program.tape[2] = y;
            if program.run().is_ok() && program.tape[0] == 19690720 {
                return x * 100 + y;
            }
        }
    }
    -1
}
