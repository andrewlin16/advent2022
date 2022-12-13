use std::borrow::Borrow;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::io;
use std::rc::Rc;
use std::vec::Vec;

#[derive(Eq, PartialEq)]
enum PacketValue {
    List(Vec<Rc<PacketValue>>),
    Integer(u32),
}

impl PartialOrd for PacketValue {
    fn partial_cmp(&self, right: &Self) -> Option<Ordering> {
        Some(self.cmp(right))
    }
}

impl Ord for PacketValue {
    fn cmp(&self, right: &Self) -> Ordering {
        match self {
            PacketValue::List(ll) => match right {
                PacketValue::List(rl) => {
                    for i in 0.. {
                        let item_l = ll.get(i);
                        let item_r = rl.get(i);

                        if item_l.is_none() && item_r.is_none() {
                            return Ordering::Equal;
                        }

                        if item_l.is_none() && item_r.is_some() {
                            return Ordering::Less;
                        }

                        if item_l.is_some() && item_r.is_none() {
                            return Ordering::Greater;
                        }

                        // Both item_l and item_r are Some.
                        let val_l = item_l.unwrap();
                        let val_r = item_r.unwrap();

                        let val_cmp = val_l.cmp(val_r);
                        if val_cmp != Ordering::Equal {
                            return val_cmp;
                        }
                    }

                    panic!("0.. apparently has zero elements to iterate on??");
                }
                PacketValue::Integer(ri) => {
                    let temp_rl = PacketValue::List(vec![Rc::new(PacketValue::Integer(*ri))]);
                    self.cmp(&temp_rl)
                }
            },
            PacketValue::Integer(li) => match right {
                PacketValue::List(_) => {
                    let temp_ll = PacketValue::List(vec![Rc::new(PacketValue::Integer(*li))]);
                    temp_ll.cmp(right)
                }
                PacketValue::Integer(ri) => li.cmp(ri),
            },
        }
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd)]
struct Packet {
    value: PacketValue,
    divider: bool,
}

fn parse(s: &String) -> Packet {
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
        PacketValue::List(v) => Packet {
            value: PacketValue::List(v.to_vec()),
            divider: false,
        },
    };
}

fn divider(i: u32) -> Packet {
    Packet {
        value: PacketValue::List(vec![Rc::new(PacketValue::List(vec![Rc::new(
            PacketValue::Integer(i),
        )]))]),
        divider: true,
    }
}

fn main() {
    let mut packets: Vec<Packet> = io::stdin()
        .lines()
        .map(|s| s.unwrap())
        .filter(|s| !s.is_empty())
        .map(|s| parse(&s))
        .collect();

    packets.push(divider(2));
    packets.push(divider(6));

    packets.sort_unstable();

    let mut index = None;
    for (i, p) in packets.iter().enumerate() {
        if p.divider {
            match index {
                Some(f) => {
                    println!("{}", f * (i + 1));
                    return;
                }
                None => {
                    index = Some(i + 1);
                }
            }
        }
    }

    panic!("could not find 2 dividers");
}
