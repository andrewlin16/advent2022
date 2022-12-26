use std::collections::HashMap;
use std::io;

enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operator {
    fn from(c: char) -> Operator {
        match c {
            '+' => Operator::Add,
            '-' => Operator::Subtract,
            '*' => Operator::Multiply,
            '/' => Operator::Divide,
            _ => panic!("expected +, -, *, or / for operator"),
        }
    }

    fn operate(&self, op1: u64, op2: u64) -> u64 {
        match self {
            Operator::Add => op1 + op2,
            Operator::Subtract => op1 - op2,
            Operator::Multiply => op1 * op2,
            Operator::Divide => op1 / op2,
        }
    }
}

struct Operation {
    op: Operator,
    operands: [String; 2],
}

impl Operation {
    fn from(s: &str) -> Operation {
        Operation {
            op: Operator::from(
                s.chars()
                    .nth(5)
                    .expect("operation should have operator at index 5"),
            ),
            operands: [String::from(&s[0..4]), String::from(&s[7..11])],
        }
    }
}

enum Job {
    Number(u64),
    Operation(Operation),
}

impl Job {
    fn from(s: &str) -> Job {
        match s.parse::<u64>() {
            Ok(v) => Job::Number(v),
            Err(_) => Job::Operation(Operation::from(s)),
        }
    }

    fn evaluate(&self, monkeys: &HashMap<String, Job>) -> u64 {
        match self {
            Job::Number(v) => *v,
            Job::Operation(o) => {
                let op1 = monkeys
                    .get(&o.operands[0])
                    .expect("monkey should exist for left operand")
                    .evaluate(monkeys);
                let op2 = monkeys
                    .get(&o.operands[1])
                    .expect("monkey should exist for left operand")
                    .evaluate(monkeys);
                return o.op.operate(op1, op2);
            }
        }
    }
}

fn main() {
    let monkeys: HashMap<String, Job> = io::stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            return (String::from(&line[0..4]), Job::from(&line[6..]));
        })
        .collect();

    let root_job = monkeys.get("root").expect("monkey named root should exist");
    println!("{}", root_job.evaluate(&monkeys));
}
