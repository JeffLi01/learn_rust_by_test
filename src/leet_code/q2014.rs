use std::collections::HashSet;

#[derive(Debug)]
struct Combinations<'a> {
    items: &'a Vec<char>,
    count: usize,
    start: usize,
    replace: bool,
}

impl<'a> Iterator for Combinations<'a> {
    type Item = Vec<char>;
    fn next(&mut self) -> Option<Vec<char>> {
        let end = self.items.len().pow(self.count as u32);
        'total:
        loop {
            if self.start >= end {
                return None;
            }
            
            if !self.replace {
                let mut used: HashSet<usize> = HashSet::new();
                let mut m = self.start;
                for _ in 0..self.count {
                    if !used.insert(m % self.items.len()) {
                        self.start += 1;
                        continue 'total;
                    }
                    m /= self.items.len();
                }
            }
            let mut v = vec![];
            let mut m = self.start;
            for _ in 0..self.count {
                v.insert(0, self.items[m % self.items.len()].clone());
                m /= self.items.len();
            }
            self.start += 1;
            let result = Some(v);
            return result
        }
    }
}

#[allow(dead_code)]
fn combinations_with_replacement(s: &Vec<char>, n: u32) -> Combinations {
    let comb = Combinations { items: s, count: n as usize, start: 0, replace: true };
    comb
}

#[allow(dead_code)]
fn combinations(s: &Vec<char>, n: u32) -> Combinations {
    let mut start = 0usize;
    for i in 0..n {
        start *= s.len();
        start += i as usize;
    }
    let comb = Combinations { items: s, count: n as usize, start: start, replace: false };
    comb
}

#[allow(dead_code)]
struct Solution;
impl Solution {
    #[allow(dead_code)]
    fn contains_seq(chars: &[char], seq: &Vec<char>) -> bool {
        if seq.len() == 0 {
            return false;
        }
        let mut found = 0usize;
        for i in 0..chars.len() {
            // if (chars.len()) - i < (seq.len() - found) {
            //     return false;
            // }
            if chars[i] == seq[found] {
                found += 1;
            }
            if found == seq.len() {
                return true;
            }
        }
        false
    }

    #[allow(dead_code)]
    pub fn longest_subsequence_repeated_k(s: String, k: i32) -> String {
        use std::collections::HashMap;
        let chars: Vec<char> = s.chars().collect();
        let mut counter: HashMap<char, i32> = HashMap::new();
        for c in &chars {
            let count = counter.entry(*c).or_insert(0);
            *count += 1;
        }
        println!("{:?}", counter);
        // let mut candidates: Vec<char> = counter.drain().filter(|(_, count)| count >= &k).collect::<HashMap<char, i32>>().into_keys().collect();
        let counter_possible = counter.drain().filter(|(_, count)| count >= &k).collect::<HashMap<char, i32>>();
        let mut candidates: Vec<char> = Vec::new();
        for (c, count) in counter_possible {
            let _: Vec<_> = (0..(count / k)).map(|_| candidates.push(c)).collect();
        }
        candidates.sort();
        candidates.reverse();
        println!("candidates: {:?}", candidates);
        let mut max_seq: Vec<char> = vec![];
        let mut len: usize = s.len() / k as usize;
        let mut found = false;
        while !found && len > 0 {
            // println!("len: {}", len);
            // for combination in Solution::combinations_with_replacement(&candidates, len) {
            for combination in combinations(&candidates, len as u32) {
                // println!("combination: {:?}", combination);
                let seq = combination.repeat(k as usize);
                if Solution::contains_seq(&chars, &seq) {
                    max_seq = combination;
                    found = true;
                    break;
                }
            }
            len -= 1;
        }
        max_seq.into_iter().collect::<String>()
    }
}

#[test]
fn test_solution0 () {
    let s = "vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuutttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttsssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp".to_string();
    let k = 251;
    let expected = "v".to_string();
    let result = Solution::longest_subsequence_repeated_k(s.to_string(), k);
    assert_eq!(expected, result);
}

#[test]
fn test_solution () {
    let s = "bbabbabbbbabaababab".to_string();
    let k = 3;
    let expected = "bbbb".to_string();
    let result = Solution::longest_subsequence_repeated_k(s, k);
    assert_eq!(expected, result);
}
#[test]
fn test_solution2 () {
    let s = "letsleetcode".to_string();
    let k = 2;
    let expected = "let".to_string();
    let result = Solution::longest_subsequence_repeated_k(s, k);
    assert_eq!(expected, result);
}
#[test]
fn test_solution3 () {
    let s = "ab".to_string();
    let k = 2;
    let expected = "".to_string();
    let result = Solution::longest_subsequence_repeated_k(s, k);
    assert_eq!(expected, result);
}
#[test]
fn test_solution4 () {
    let s = "nhsbbeonhsbbeonhsbbeo".to_string();
    let k = 3;
    let expected = "nhsbbeo".to_string();
    let result = Solution::longest_subsequence_repeated_k(s, k);
    assert_eq!(expected, result);
}
#[test]
fn test_solution5 () {
    let s = "vptwvlvcmptwvlvptwvlvptrwvlvptwvlvptwrvlvptwvlvpktwivlvptwovlcpvptwvlvptwvlvptwvlvputwvlvfptwvlvptwkvlvptwvlvptwvlvhptwavlqvptswvlwvptwvlvptwvlvpotowvl".to_string();
    let k = 22;
    let expected = "vptwvl".to_string();
    let result = Solution::longest_subsequence_repeated_k(s, k);
    assert_eq!(expected, result);
}
#[test]
fn test_solution6 () {
    let s = "tsrkimbtsrkimbtsrkimbtsrkimbtsrkimbtsrkimbtsrkimbtsrkimbtsrkimbtsrkimbtsrkimbtsrkimbtsrkimbtsrkimbtsrkimbtsrkimbtsrkimbtsrkimbtsrkimbtsrkimbtsrkimbtsrkimbtsrkimbtsrkimbtsrkimbtsrkimbtsrkimb".to_string();
    let k = 27;
    let expected = "tsrkimb".to_string();
    let result = Solution::longest_subsequence_repeated_k(s, k);
    assert_eq!(expected, result);
}
#[test]
fn test_solution7 () {
    let s = "ororobrorororororhororosrorordorowrororororsororyororororororosrorororojjrorororororoprorvorxorororororoqrhoriorozrorcorzororkororokrororornorororororortorosroyrohrorwhorovrorxoro".to_string();
    let k = 74;
    let expected = "ro".to_string();
    let result = Solution::longest_subsequence_repeated_k(s, k);
    assert_eq!(expected, result);
}
