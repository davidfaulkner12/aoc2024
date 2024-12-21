#[derive(Clone, Debug)]
struct Computer {
    program: Vec<u8>,
    ar: i64,
    br: i64,
    cr: i64,
    ip: usize,
    output: Vec<i64>,
}

impl Computer {
    fn new(program: Vec<u8>, ar: i64, br: i64, cr: i64) -> Self {
        Computer {
            program,
            ar,
            br,
            cr,
            ip: 0,
            output: Vec::new(),
        }
    }

    fn is_done(&self) -> bool {
        self.ip >= self.program.len() - 1
    }

    fn step(&self) -> Self {
        let mut res = self.clone();
        if self.ip >= self.program.len() - 1 {
            return res;
        }
        let inst = self.program[self.ip];
        let op = self.program[self.ip + 1];

        res.ip += 2;

        match inst {
            0 => res.ar = self.ar / (1 << self.combo(op)),
            1 => res.br = op as i64 ^ self.br,
            2 => res.br = self.combo(op) % 8,
            3 => {
                if self.ar != 0 {
                    res.ip = op as usize
                }
            }
            4 => res.br = self.br ^ self.cr,
            5 => res.output.push(self.combo(op) % 8),
            6 => res.br = self.ar / (1 << self.combo(op)),
            7 => res.cr = self.ar / (1 << self.combo(op)),
            _ => todo!(),
        }

        res
    }

    fn combo(&self, op: u8) -> i64 {
        match op {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.ar,
            5 => self.br,
            6 => self.cr,
            _ => panic!("Invalid combo operand"),
        }
    }
}

#[cfg(test)]
mod tests {
    use core::time;
    use std::{fs, num, thread};

    use crate::day4::find_char_in_puzzle;

    use super::*;

    const TEST_DATA: &str = "";

    #[test]
    fn test_example1() {
        let c = Computer::new(vec![2, 6], 0, 0, 9);

        let c = c.step();

        assert_eq!(c.br, 1);
    }

    #[test]
    fn test_example2_17() {
        let mut c = Computer::new(vec![5, 0, 5, 1, 5, 4], 10, 0, 0);

        while !c.is_done() {
            c = c.step();
        }

        assert_eq!(c.output, vec![0, 1, 2])
    }

    #[test]
    fn test_example3_17() {
        let mut c = Computer::new(vec![0, 1, 5, 4, 3, 0], 2024, 0, 0);

        while !c.is_done() {
            c = c.step();
        }

        assert_eq!(c.output, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(c.ar, 0);
    }

    #[test]
    fn test_example4_17() {
        let mut c = Computer::new(vec![1, 7], 0, 29, 0);

        while !c.is_done() {
            c = c.step();
        }

        assert_eq!(c.br, 26);
    }

    #[test]
    fn test_example5_17() {
        let mut c = Computer::new(vec![4, 0], 0, 2024, 43690);

        while !c.is_done() {
            c = c.step();
        }

        assert_eq!(c.br, 44354);
    }

    #[test]
    fn test_example6_17() {
        let mut c = Computer::new(vec![0, 1, 5, 4, 3, 0], 729, 0, 0);

        while !c.is_done() {
            c = c.step();
        }

        assert_eq!(c.output, vec![4, 6, 3, 5, 6, 3, 5, 2, 1, 0]);
    }

    #[test]
    fn test_actual_prob_17_part1() {
        let mut c = Computer::new(
            vec![2, 4, 1, 5, 7, 5, 1, 6, 0, 3, 4, 1, 5, 5, 3, 0],
            44374556,
            0,
            0,
        );
        while !c.is_done() {
            c = c.step();
        }

        // 1,5,0,3,7,3,0,3,1

        assert_eq!(c.output, vec![1, 5, 0, 3, 7, 3, 0, 3, 1]);
    }

    #[test]
    fn test_example_17_prob2() {
        let program = vec![0, 3, 5, 4, 3, 0];

        let mut c = Computer::new(program.clone(), 117440, 0, 0);

        while !c.is_done() {
            c = c.step();
        }

        assert_eq!(
            c.output.iter().map(|i| *i as u8).collect::<Vec<_>>(),
            program
        );
    }

    #[test]
    fn test_actual_17_prob2() {
        let program = vec![2, 4, 1, 5, 7, 5, 1, 6, 0, 3, 4, 1, 5, 5, 3, 0];
        //let program = vec![0, 3, 5, 4, 3, 0];
        let program_i64: Vec<_> = program.clone().into_iter().map(|i| i as i64).collect();

        //let mut c = Computer::new(program.clone(), 0b100001100001001, 0, 0);
        //let mut c = Computer::new(program.clone(), 0b10101001111001100001001, 0, 0);
        //let mut c = Computer::new(program.clone(), 281474976710655, 0, 0);
        let mut c = Computer::new(program.clone(), 105981155568026, 0, 0);

        while !c.is_done() {
            c = c.step();
        }

        assert_eq!(c.output, program_i64);
    }

    #[test]
    fn test_find_sequence_17() {
        let program = vec![2, 4, 1, 5, 7, 5, 1, 6, 0, 3, 4, 1, 5, 5, 3, 0];
        //let program = vec![0, 3, 5, 4, 3, 0];
        let program_i64: Vec<_> = program.clone().into_iter().map(|i| i as i64).collect();

        fn find_a(
            program_i64: &Vec<i64>,
            program: &Vec<u8>,
            ar: i64,
            target_idx: usize,
        ) -> Option<i64> {
            if target_idx == usize::MAX {
                return Some(ar);
            }
            for i in 0..8 {
                let next_a = ar * 8 + i;
                let mut c = Computer::new(program.clone(), next_a, 0, 0);
                while !c.is_done() {
                    c = c.step();
                }
                if c.output[0] == program_i64[target_idx] {
                    println!("{:?} {:?}", next_a, c.output);
                    let best_a = find_a(program_i64, program, next_a, target_idx.wrapping_sub(1));
                    if best_a.is_some() {
                        return best_a;
                    }
                    println!("Backtracking from {next_a}");
                }
            }

            None
        }

        println!("{:?}", program);

        println!("{:?}", find_a(&program_i64, &program, 0, program.len() - 1));
    }
}
