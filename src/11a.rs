use std::cell::RefCell;
use std::io;
use std::rc::Rc;
use std::vec::Vec;

enum Operator {
    Add,
    Multiply,
}

enum Operand {
    Old,
    Number(u32),
}

struct Operation {
    operator: Operator,
    operand: Operand,
}

impl Operation {
    fn from(s: String) -> Operation {
        Operation {
            operator: match s.chars().nth(0).expect("operation should have operator") {
                '+' => Operator::Add,
                '*' => Operator::Multiply,
                _ => panic!("operator in operation should be + or *"),
            },
            operand: {
                let s = &s[2..];
                if s == "old" {
                    Operand::Old
                } else {
                    Operand::Number(
                        s.parse()
                            .expect("operand in operation should be number if not 'old'"),
                    )
                }
            },
        }
    }

    fn apply(&self, val: u32) -> u32 {
        let o = match self.operand {
            Operand::Old => val,
            Operand::Number(n) => n,
        };

        return match self.operator {
            Operator::Add => val + o,
            Operator::Multiply => val * o,
        };
    }
}

struct Monkey {
    items: Rc<RefCell<Vec<u32>>>,
    operation: Operation,
    test: u32,
    true_target: usize,
    false_target: usize,
    inspects: RefCell<usize>,
}

fn main() {
    let mut line = String::new();
    let mut monkeys: Vec<Rc<RefCell<Monkey>>> = Vec::new();

    loop {
        // Read "Monkey n:" line.
        let _ = io::stdin().read_line(&mut line);

        // Read and parse "Starting items: ..." line.
        line.clear();
        let _ = io::stdin().read_line(&mut line);
        let items: Vec<u32> = line[18..]
            .split(", ")
            .map(|s| s.trim().parse().expect("item should be numeric"))
            .collect();

        // Read and parse "Operation: ..." line.
        line.clear();
        let _ = io::stdin().read_line(&mut line);
        let operation = Operation::from(line[23..].trim().to_string());

        // Read and parse "Test: ..." line.
        line.clear();
        let _ = io::stdin().read_line(&mut line);
        let test = line[21..]
            .trim()
            .parse()
            .expect("test divisible by number should be numeric");

        // Read and parse "If true: ..." line.
        line.clear();
        let _ = io::stdin().read_line(&mut line);
        let true_target = line[29..]
            .trim()
            .parse()
            .expect("If true target should be numeric");

        // Read and parse "If false: ..." line.
        line.clear();
        let _ = io::stdin().read_line(&mut line);
        let false_target = line[30..]
            .trim()
            .parse()
            .expect("If true target should be numeric");

        // Create Monkey and add to vec.
        monkeys.push(Rc::new(RefCell::new(Monkey {
            items: Rc::new(RefCell::new(items)),
            operation: operation,
            test: test,
            true_target: true_target,
            false_target: false_target,
            inspects: RefCell::new(0),
        })));

        // Read blank line/check for end of input.
        let cnt = io::stdin().read_line(&mut line);
        if cnt.unwrap() == 0 {
            // Reached EOF, go to processing stage.
            break;
        }
    }

    // Run rounds on monkeys.
    for _ in 0..20 {
        for c in &monkeys {
            let m = c.borrow_mut();
            let mut items = m.items.borrow_mut();
            *m.inspects.borrow_mut() += items.len();

            let op = &m.operation;
            let test = &m.test;
            let true_target = &m.true_target;
            let false_target = &m.false_target;

            for v in items.drain(..) {
                let new_val = op.apply(v) / 3;
                let target_num = if new_val % test == 0 {
                    true_target
                } else {
                    false_target
                };
                let target = monkeys[*target_num].borrow_mut();
                target.items.borrow_mut().push(new_val);
            }
        }
    }

    // Find top 2 active monkeys to calculate monkey business.
    monkeys.select_nth_unstable_by_key(1, |m| -(*m.borrow().inspects.borrow() as i64));
    let business = *monkeys[0].borrow().inspects.borrow() * *monkeys[1].borrow().inspects.borrow();
    println!("{}", business);
}
