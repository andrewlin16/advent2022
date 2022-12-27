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
    operator: Operator,
    operands: [String; 2],
}

impl Operation {
    fn from(s: &str) -> Operation {
        Operation {
            operator: Operator::from(
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
                    .expect("monkey should exist for left operand in evaluate")
                    .evaluate(monkeys);
                let op2 = monkeys
                    .get(&o.operands[1])
                    .expect("monkey should exist for left operand in evaluate")
                    .evaluate(monkeys);
                return o.operator.operate(op1, op2);
            }
        }
    }

    // Unwraps Operation from Job.
    fn op(&self) -> &Operation {
        match self {
            Job::Number(_) => panic!("operation should have operation"),
            Job::Operation(o) => o,
        }
    }

    // Splits Job::Operation into (human operand key, non-human operand value).
    fn split(&self, monkeys: &HashMap<String, Job>) -> (&String, u64) {
        let op = self.op();
        if has_human(&monkeys, &op.operands[0]) {
            return (
                &op.operands[0],
                monkeys
                    .get(&op.operands[1])
                    .expect("monkey should exist for right operand in split")
                    .evaluate(&monkeys),
            );
        } else {
            return (
                &op.operands[1],
                monkeys
                    .get(&op.operands[0])
                    .expect("monkey should exist for left operand in split")
                    .evaluate(&monkeys),
            );
        }
    }
}

fn has_human(monkeys: &HashMap<String, Job>, key: &String) -> bool {
    if key == "humn" {
        return true;
    }

    let job = monkeys
        .get(key)
        .expect("monkey should exist for has_human key");
    if let Job::Operation(op) = job {
        let left = has_human(monkeys, &op.operands[0]);
        let right = has_human(monkeys, &op.operands[1]);

        return left || right;
    } else {
        return false;
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

    let root = monkeys.get("root").expect("monkey named root should exist");
    let (mut human_side, mut match_value) = root.split(&monkeys);

    loop {
        if human_side == "humn" {
            break;
        }

        let job = monkeys.get(human_side).expect("monkey should exist");
        let op = job.op();

        let split = job.split(&monkeys);
        human_side = split.0;
        let operand = split.1;

        match_value = match op.operator {
            Operator::Add => match_value - operand,
            Operator::Subtract => {
                if *human_side == op.operands[0] {
                    match_value + operand
                } else {
                    operand - match_value
                }
            }
            Operator::Multiply => match_value / operand,
            Operator::Divide => {
                if *human_side == op.operands[0] {
                    match_value * operand
                } else {
                    operand / match_value
                }
            }
        };
    }

    println!("{}", match_value);
}
