use std::{
    fmt,
    io::{self, prelude::*},
};

enum Instruction {
    noop,
    addx(i32),
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Instruction::noop => write!(f, "noop"),
            Instruction::addx(value) => write!(f, "addx {}", value),
        }
    }
}

impl Instruction {
    fn get_cycle_count(&self) -> usize {
        match self {
            Instruction::noop => 1,
            Instruction::addx(_) => 2,
        }
    }
}

struct Cpu {
    x_register: i32,
    cycle: usize,
}

impl Cpu {
    fn new() -> Self {
        Self {
            x_register: 1,
            cycle: 0,
        }
    }

    fn run(&mut self, instructions: &Vec<Instruction>, monitoring: &mut CpuMonitoring) {
        for instruction in instructions {
            self.execute(instruction, monitoring);
        }
    }

    fn execute(&mut self, instruction: &Instruction, monitoring: &mut CpuMonitoring) {
        monitoring.add(self, instruction.get_cycle_count(), instruction.to_string());

        self.cycle += instruction.get_cycle_count();

        match instruction {
            Instruction::noop => {}
            Instruction::addx(value) => {
                self.x_register += value;
            }
        }

        monitoring.update_current_state(self);
    }
}

#[derive(Debug)]
struct CpuSnapshot {
    x_state: i32,
    action: String,
}

#[derive(Debug)]
struct CpuMonitoring {
    monitor: Vec<CpuSnapshot>,
    current_state: CpuSnapshot,
}

impl CpuMonitoring {
    fn new() -> Self {
        Self {
            monitor: vec![],
            current_state: CpuSnapshot {
                x_state: 0,
                action: "Ready for CPU instructions".to_string(),
            },
        }
    }

    fn add(&mut self, cpu: &Cpu, cycle_count: usize, action: String) {
        for _ in 0..cycle_count {
            self.monitor.push(CpuSnapshot {
                x_state: cpu.x_register,
                action: format!("{} (cycle {})", action, cpu.cycle),
            });
        }
        self.current_state = CpuSnapshot {
            x_state: cpu.x_register,
            action: format!("{} (cycle {})", action, cpu.cycle),
        };
    }

    fn update_current_state(&mut self, cpu: &Cpu) {
        self.current_state = CpuSnapshot {
            x_state: cpu.x_register,
            action: format!("Ready for CPU instructions (cycle {})", cpu.cycle),
        };
    }

    fn calculate_signal_strength(&self, cycle: usize) -> i32 {
        let signal = self.monitor.get(cycle - 1);

        match signal {
            Some(signal) => signal.x_state * cycle as i32,
            None => self.current_state.x_state * cycle as i32,
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let instructions = lines
        .map(|l| {
            let line = l.unwrap();
            if line == "noop" {
                return Instruction::noop;
            }

            let (command, value) = line.split_once(' ').unwrap();
            let value = value.parse::<i32>().unwrap();
            match command {
                "addx" => Instruction::addx(value),
                _ => panic!("Unknown instruction: {}", command),
            }
        })
        .collect::<Vec<_>>();

    let mut cpu = Cpu::new();
    let mut cpu_monitoring = CpuMonitoring::new();
    cpu.run(&instructions, &mut cpu_monitoring);

    let total_signal_strength: i32 = [20, 60, 100, 140, 180, 220]
        .map(|cycle| cpu_monitoring.calculate_signal_strength(cycle))
        .iter()
        .sum();

    println!(
        "The sum of the six signal strengths is {}.",
        total_signal_strength
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(1, 1 * 1)]
    #[case(2, 1 * 2)]
    #[case(3, 1 * 3)]
    #[case(4, 4 * 4)]
    #[case(5, 4 * 5)]
    #[case(6, -1 * 6)]
    fn test_signals(#[case] cycle: usize, #[case] expected: i32) {
        let mut cpu = Cpu::new();
        let mut cpu_monitoring = CpuMonitoring::new();
        let instructions = vec![
            Instruction::noop,
            Instruction::addx(3),
            Instruction::addx(-5),
        ];
        cpu.run(&instructions, &mut cpu_monitoring);

        println!("{:?}", cpu_monitoring);

        assert_eq!(expected, cpu_monitoring.calculate_signal_strength(cycle));
    }

    // #[rstest]
    // #[case(20, 420)]
    // #[case(60, 1140)]
    // #[case(100, 1800)]
    // #[case(140, 2940)]
    // #[case(180, 2880)]
    // #[case(220, 3960)]
    // fn test_signals(#[case] cycle: usize, #[case] expected: usize) {
    //     assert_eq!(expected, find_first_marker(4, input).unwrap())
    // }
}
