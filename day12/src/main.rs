use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    io::BufRead,
    rc::Rc,
};

struct Cave {
    name: String,
    big: bool,
    next: Vec<Rc<RefCell<Cave>>>,
}

struct CaveGraph {
    caves: HashMap<String, Rc<RefCell<Cave>>>,
}

impl CaveGraph {
    fn new() -> Self {
        Self {
            caves: HashMap::new(),
        }
    }

    fn parse(f: impl BufRead) -> Self {
        let mut g = Self::new();
        for l in f.lines().flatten() {
            let mut l = l.split('-');
            g.add_edge(l.next().unwrap(), l.next().unwrap());
        }
        g
    }

    fn add_edge(&mut self, a: &str, b: &str) {
        let a = self.node(a);
        let b = self.node(b);
        a.borrow_mut().next.push(b.clone());
        b.borrow_mut().next.push(a);
    }

    fn node(&mut self, name: &str) -> Rc<RefCell<Cave>> {
        self.caves
            .entry(name.to_string())
            .or_insert_with(|| {
                Rc::new(RefCell::new(Cave {
                    name: name.to_string(),
                    big: name.chars().next().unwrap().is_uppercase(),
                    next: vec![],
                }))
            })
            .clone()
    }

    fn count_paths(&self, start: &str, end: &str) -> i32 {
        self.dfs(
            &self.caves.get(start).unwrap().borrow(),
            end,
            &mut HashSet::<String>::from_iter([start.to_string()]),
            true,
        )
    }

    fn count_paths_with_extra(&self, start: &str, end: &str) -> i32 {
        self.dfs(
            &self.caves.get(start).unwrap().borrow(),
            end,
            &mut HashSet::<String>::from_iter([start.to_string()]),
            false,
        )
    }

    fn dfs(
        &self,
        curr: &Cave,
        end: &str,
        small_visited: &mut HashSet<String>,
        has_extra: bool,
    ) -> i32 {
        if curr.name == end {
            return 1;
        }

        let mut tot = 0;
        for c in curr.next.iter().map(|c| c.borrow()) {
            if c.big {
                tot += self.dfs(&c, end, small_visited, has_extra);
            } else if small_visited.get(&c.name).is_none() {
                small_visited.insert(c.name.clone());
                tot += self.dfs(&c, end, small_visited, has_extra);
                small_visited.remove(&c.name);
            } else if c.name != "start" && !has_extra {
                tot += self.dfs(&c, end, small_visited, true);
            }
        }
        tot
    }
}

impl Drop for CaveGraph {
    fn drop(&mut self) {
        for v in self.caves.values_mut() {
            let mut m = v.borrow_mut();
            m.next.clear();
        }
    }
}

fn main() {
    let g = CaveGraph::parse(std::io::BufReader::new(std::io::stdin()));
    println!("{}", g.count_paths("start", "end"));
    println!("{}", g.count_paths_with_extra("start", "end"));
}

#[cfg(test)]
mod tests {
    use crate::CaveGraph;

    #[test]
    fn test_count_path() {
        let mut g = CaveGraph::new();
        g.add_edge("start", "A");
        g.add_edge("start", "b");
        g.add_edge("A", "c");
        g.add_edge("A", "b");
        g.add_edge("b", "d");
        g.add_edge("A", "end");
        g.add_edge("b", "end");
        assert_eq!(g.count_paths("start", "end"), 10);
        assert_eq!(g.count_paths_with_extra("start", "end"), 36);
    }

    #[test]
    fn test_parse_count_path() {
        let g = CaveGraph::parse(
            "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW"
                .as_bytes(),
        );
        assert_eq!(g.count_paths("start", "end"), 226);
        assert_eq!(g.count_paths_with_extra("start", "end"), 3509);
    }
}
