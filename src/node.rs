#[derive(Debug)]
pub struct State {
    pub optional: bool,
    pub negative: bool,
    pub greedy: bool,
    pub option: bool,
}

#[derive(Debug)]
pub struct Node {
    pub char: Option<char>,
    pub optional: bool,
    pub greedy: bool,
    pub negative: bool,
    pub next: Option<Box<Node>>, // children: Option<Box<Node>>,
}

impl Node {
    pub fn new() -> Self {
        Node {
            char: None,
            optional: false,
            greedy: false,
            negative: false,
            next: None,
        }
    }
    //
    pub fn from_char(c: char, state: &mut State) -> Self {
        let n = Node {
            char: Some(c),
            optional: state.optional,
            greedy: state.greedy,
            negative: state.negative,
            next: None,
        };
        // consume state
        if state.negative {
            state.negative = false;
        }
        n
    }
    //
    pub fn test(str: &str) -> bool {
        false
    }
}

#[derive(Debug)]
pub struct Regs {
    pub nodes: Vec<Node>,
}
impl Regs {
    pub fn test(&self, str: &str) {
        for n in &self.nodes {
            println!("\n{:?}\n", n);
        }
    }

    pub fn print(&self) {
        println!("[",);
        for n in &self.nodes {
            if n.char.is_some() {
                let mut out = "".to_string();
                if n.negative {
                    out.push('!');
                }
                out.push(n.char.unwrap());
                if n.greedy {
                    out.push('*');
                }
                if n.optional {
                    out.push('?');
                }

                println!("  {:?},", out);
            }
        }
        println!("]",);
    }
}
