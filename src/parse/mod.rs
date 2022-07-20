use super::node::{Node, Regs, State};

pub fn parse(reg: &str) -> Regs {
    let mut nodes: Vec<Node> = Vec::new();
    let arr: Vec<char> = reg.chars().collect();
    let mut state = State {
        optional: false,
        negative: false,
        greedy: false,
        option: false,
    };
    for c in arr {
        // println!("\n{:?}  {:?}\n", c, c.peek());
        match c {
            '!' => state.negative = true,
            '?' => state.optional = true,
            '+' => state.greedy = true,
            '[' => state.option = true,
            ']' => state.option = false,
            '*' => {
                state.optional = true;
                state.greedy = true;
            }
            _ => {
                let node = Node::from_char(c, &mut state);
                nodes.push(node);
            }
        }
    }
    Regs { nodes }
}
