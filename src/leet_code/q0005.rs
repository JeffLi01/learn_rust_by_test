#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn longest_palindrome(s: String) -> String {
        let s = s.as_bytes();
        let mut lens: Vec<usize> = Vec::with_capacity(s.len() * 2 - 1);

        for i in 0..lens.capacity() {
            if (i & 1) == 0 {
                lens.push(1);
                let mut left;
                let mut right;
                for len in 0..(i / 2) {
                    if len == i / 2 {
                        break;
                    }
                    if (i / 2 + 1 + len) >= s.len() {
                        break;
                    }
                    left = i / 2 - 1 - len;
                    right = i / 2 + 1 + len;
                    print!("left: {}, right: {}", left, right);
                    if s[left] == s[right] {
                        if let Some(elem) = lens.get_mut(i) {
                            *elem += 2;
                            print!(" {}", *elem);
                        }
                        println!("\t√");
                    } else {
                        println!("\tx");
                        break;
                    }
                }
            } else {
                lens.push(0);
                let mut left;
                let mut right;
                for len in 0..((i + 1) / 2) {
                    // if i / 2 < len {
                    //     break;
                    // }
                    if (i / 2 + 1 + len) >= s.len() {
                        break;
                    }
                    left = i / 2 - len;
                    right = i / 2 + 1 + len;
                    print!("left: {}, right: {}", left, right);
                    if s[left] == s[right] {
                        if let Some(elem) = lens.get_mut(i) {
                            *elem += 2;
                            print!(" {}", *elem);
                        }
                        println!("\t√");
                    } else {
                        println!("\tx");
                        break;
                    }
                    if left == 0 || right == (s.len() - 1) {
                        break;
                    }
                }
            }
        }

        // lens[2].to_string()
        let max_len = lens.iter().max().unwrap().to_owned();
        let max_index = lens.iter().position(|r| *r == max_len).unwrap();
        let start;
        if (max_index & 1) == 0 {
            start = max_index / 2 - max_len / 2;
        } else {
            start = (max_index + 1) / 2 - max_len / 2;
        }
        println!("{}, {}, {}", max_index, start, max_len);
        String::from_utf8(s[start..(start + max_len)].to_vec()).unwrap()
    }
}

#[test]
fn test_solution() {
    assert_eq!("bb", Solution::longest_palindrome("bb".to_string()));
}
