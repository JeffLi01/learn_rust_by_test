use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct MatchState<'a> {
    pub remain: &'a str,
    pub parent: Option<Rc<MatchState<'a>>>,
}

#[derive(Copy, Clone, Debug)]
struct PatternUnit {
    c: char,
    any: bool,
}

impl PatternUnit {
    #[allow(dead_code)]
    pub fn parse<'a>(self, node: Rc<MatchState>) -> Vec<Rc<MatchState>>
    {
        let mut match_trees = Vec::new();
        if self.any {
            match_trees.push(Rc::new(MatchState{remain: &node.remain, parent: Some(node.clone())}));
        }
        for (i, c) in node.remain.chars().enumerate() {
            if self.c == '.' || self.c == c {
                println!("{}: [{}]", self.c, &node.remain[i+1..]);
                match_trees.push(Rc::new(MatchState{remain: &node.remain[i+1..], parent: Some(node.clone())}));
                if !self.any {
                    break;
                }
            } else {
                break;
            }
        }
        match_trees
    }
}

#[allow(dead_code)]
fn parse_pattern(p: String) -> Vec<PatternUnit>
{
    let mut pattern_units: Vec<PatternUnit> = Vec::new();
    for c in p.chars() {
        match c {
            '*' => {
                let m = pattern_units.last_mut().unwrap();
                m.any = true;
            },
            c => {
                let unit = PatternUnit {c, any: false};
                pattern_units.push(unit);
            },
        }
    }
    pattern_units
}

#[allow(dead_code)]
pub fn dump_tree(leaf: Rc<MatchState>)
{
    println!("in dump_tree");
    println!("remain: {}", leaf.remain);
    let mut parent = leaf.parent.clone();
    while let Some(p) = parent {
        println!("remain: {}", p.remain);
        parent = p.parent.clone();
    }
}

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn is_match(s: String, p: String) -> bool {
        if s.len() == 0 && p.len() == 0 {
            return true;
        } else if p.len() == 0 {
            return false;
        }

        let pattern_units = parse_pattern(p);
        let root = Rc::new(MatchState { remain: &s, parent: None });
        let mut match_trees = vec![root];
        println!("match_trees:  {:?}", match_trees);
        for (i, unit) in pattern_units.iter().enumerate() {
            let mut result_match_trees: Vec<Rc<MatchState>> = Vec::new();
            for match_tree in match_trees.iter() {
                let mut trees = unit.parse(match_tree.clone());
                for tree in trees.iter_mut() {
                    // println!("unit:   {:?}", unit);
                    // println!("remain: {:?}", match_tree.remain);
                    // println!("trees:  {:?}", trees);
                    result_match_trees.push(tree.clone());
                    println!("{}\t{}{}\t{}", i, unit.c, if unit.any { '*' } else { ' ' }, tree.remain);
                }
            }
            match_trees.clear();
            match_trees.append(&mut result_match_trees);
        }
        for match_tree in match_trees {
            if match_tree.remain.len() == 0 {
                dump_tree(match_tree);
                return true;
            }
        }
        false
    }
}


#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test01 () {
        assert_eq!(Solution::is_match("mississippi".to_string(), "mis*is*p*.".to_string()), false);
    }
    #[test]
    fn test02 () {
        assert_eq!(Solution::is_match("".to_string(), "".to_string()), true);
    }

}
