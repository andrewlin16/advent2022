use std::borrow::Borrow;
use std::cell::RefCell;
use std::io;
use std::rc::Rc;
use std::vec::Vec;

enum PacketValue {
    List(Vec<Rc<PacketValue>>),
    Integer(u32),
}

fn parse(s: &String) -> PacketValue {
    let mut stack = Vec::new();
    let mut cur_list: Vec<Rc<PacketValue>> = Vec::new();

    let mut i = 0;
    while i < s.len() {
        let c = s
            .chars()
            .nth(i)
            .expect("line should still have input to parse");

        if c == '[' {
            stack.push(Rc::new(RefCell::new(cur_list)));
            cur_list = Vec::new();
        } else if c >= '0' && c <= '9' {
            let mut value = 0;
            'int_parse: loop {
                let vc = s.chars().nth(i).unwrap();
                let d = vc.to_digit(10).unwrap();
                value = value * 10 + d;

                i += 1;
                let nc = s
                    .chars()
                    .nth(i)
                    .expect("line should still have input to parse");
                if nc == ',' || nc == ']' {
                    cur_list.push(Rc::new(PacketValue::Integer(value)));
                    i -= 1;
                    break 'int_parse;
                }
            }
        } else if c == ']' {
            let top_c = stack.pop().expect("parse stack should not be empty");
            let mut top = top_c.borrow_mut();
            top.push(Rc::new(PacketValue::List(cur_list)));
            cur_list = top.to_vec();
        }

        i += 1;
    }

    return match &*cur_list.pop().unwrap().borrow() {
        PacketValue::Integer(_) => panic!("inner item should be list"),
        PacketValue::List(v) => PacketValue::List(v.to_vec()),
    };
}

fn compare(left: &PacketValue, right: &PacketValue) -> Option<bool> {
    match left {
        PacketValue::List(ll) => match right {
            PacketValue::List(rl) => {
                for i in 0.. {
                    let item_l = ll.get(i);
                    let item_r = rl.get(i);

                    if item_l.is_none() && item_r.is_none() {
                        return None;
                    }

                    if item_l.is_none() && item_r.is_some() {
                        return Some(true);
                    }

                    if item_l.is_some() && item_r.is_none() {
                        return Some(false);
                    }

                    // Both item_l and item_r are Some.
                    let val_l = item_l.unwrap();
                    let val_r = item_r.unwrap();

                    let val_cmp = compare(val_l, val_r);
                    if val_cmp.is_some() {
                        return val_cmp;
                    }
                }

                panic!("0.. apparently has zero elements to iterate on??");
            }
            PacketValue::Integer(ri) => {
                let temp_rl = PacketValue::List(vec![Rc::new(PacketValue::Integer(*ri))]);
                compare(left, &temp_rl)
            }
        },
        PacketValue::Integer(li) => match right {
            PacketValue::List(_) => {
                let temp_ll = PacketValue::List(vec![Rc::new(PacketValue::Integer(*li))]);
                compare(&temp_ll, right)
            }
            PacketValue::Integer(ri) => {
                if li > ri {
                    Some(false)
                } else if li == ri {
                    None
                } else {
                    Some(true)
                }
            }
        },
    }
}

fn main() {
    let mut sum = 0;
    let mut index = 1;
    let mut line = String::new();

    loop {
        line.clear();
        let _ = io::stdin().read_line(&mut line);
        let left = parse(&line);

        line.clear();
        let _ = io::stdin().read_line(&mut line);
        let right = parse(&line);

        if compare(&left, &right).unwrap_or(true) {
            sum += index;
        }

        let cnt = io::stdin().read_line(&mut line);
        if cnt.unwrap() == 0 {
            break;
        }

        index += 1;
    }

    println!("{}", sum);
}
