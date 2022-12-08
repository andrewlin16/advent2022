use std::cell::RefCell;
use std::collections::HashMap;
use std::io;
use std::rc::Rc;
use std::vec::Vec;

struct Filesystem {
    entities: Vec<Rc<RefCell<Entity>>>,
}

impl Filesystem {
    fn new() -> Filesystem {
        let root = Entity::Dir(Directory::new(0));
        return Filesystem {
            entities: vec![Rc::new(RefCell::new(root))],
        };
    }

    fn add_entity(&mut self, entity: Rc<RefCell<Entity>>) -> usize {
        let index = self.entities.len();
        self.entities.push(Rc::clone(&entity));
        return index;
    }

    fn get(&self, index: usize) -> Rc<RefCell<Entity>> {
        Rc::clone(&self.entities[index])
    }

    fn dirs(&self) -> impl Iterator<Item = Rc<RefCell<Directory>>> + '_ {
        return self
            .entities
            .iter()
            .filter(|e| matches!(&*e.borrow(), Entity::Dir(_)))
            .map(|e| Rc::clone(&e.borrow().as_dir()));
    }
}

enum Entity {
    File(u64),
    Dir(Rc<RefCell<Directory>>),
}

impl Entity {
    fn as_dir(&self) -> Rc<RefCell<Directory>> {
        match self {
            Entity::File(_) => panic!("entity should be Dir"),
            Entity::Dir(d) => Rc::clone(&d),
        }
    }
}

struct Directory {
    parent: usize,
    contents: HashMap<String, usize>,
}

impl Directory {
    fn new(parent: usize) -> Rc<RefCell<Directory>> {
        return Rc::new(RefCell::new(Directory {
            parent: parent,
            contents: HashMap::new(),
        }));
    }

    fn insert(&mut self, name: String, index: usize) {
        self.contents.insert(name, index);
    }

    fn get(&self, name: String) -> Option<usize> {
        self.contents.get(&name).map(|v| *v)
    }

    fn size(&self, fs: &Filesystem) -> u64 {
        self.contents.values().fold(0, |acc, i| {
            return acc
                + match &*fs.get(*i).borrow() {
                    Entity::File(s) => *s,
                    Entity::Dir(d) => d.borrow().size(fs),
                };
        })
    }
}

fn main() {
    let mut fs = Filesystem::new();
    let mut cd = 0;

    for line in io::stdin().lines() {
        let line = line.unwrap();

        if line.chars().nth(0).unwrap() == '$' {
            // Parse out shell command.
            let cmd = &line[2..];
            if cmd.starts_with("cd") {
                let target = &cmd[3..];
                if target == "/" {
                    // Set current directory to root FS directory.
                    cd = 0;
                } else if target == ".." {
                    // Go up one level via current directory's parent.
                    let dir = fs.get(cd).borrow().as_dir();
                    cd = dir.borrow().parent;
                } else {
                    // Go down one level by getting named directory from contents.
                    let dir = fs.get(cd).borrow().as_dir();
                    cd = dir
                        .borrow()
                        .get(target.to_string())
                        .expect("cd target should exist");
                }
            }
            // If command is not cd, then it must be ls, which will then go to
            // the else case below.
        } else {
            // Add entry from ls into current directory.
            let dir = fs.get(cd).borrow_mut().as_dir();

            let mut entry_split = line.splitn(2, ' ');
            let meta = entry_split.next().expect("entry should have dir or size");
            let name = entry_split
                .next()
                .expect("entry should have name")
                .to_string();

            if meta == "dir" {
                let ent = Rc::new(RefCell::new(Entity::Dir(Directory::new(cd))));
                let new_dir = fs.add_entity(ent);
                dir.borrow_mut().insert(name, new_dir);
            } else {
                let ent = Rc::new(RefCell::new(Entity::File(
                    meta.parse().expect("size field should be a number"),
                )));
                let file = fs.add_entity(ent);
                dir.borrow_mut().insert(name, file);
            }
        }
    }

    let cur_free = 70000000 - fs.get(0).borrow().as_dir().borrow().size(&fs);
    let to_delete = 30000000 - cur_free;

    let size_min: u64 = fs
        .dirs()
        .map(|d| d.borrow().size(&fs))
        .filter(|&s| s >= to_delete)
        .min()
        .expect("should have at least one directory eligible for removal");
    println!("{}", size_min);
}
