use std::rc::Rc;

#[allow(dead_code)]
struct TreeNode<'a> {
    remain: &'a str,
    parent: Option<Rc<TreeNode<'a>>>,
}


#[test]
fn test_reference() {
    let a = "Hello world".to_string();
    let node1 = TreeNode { remain: &a[..], parent: None };
    assert_eq!(node1.remain, "Hello world".to_string());
}

#[test]
fn test_rc() {
    let a = "Hello world".to_string();
    let node1 = Rc::new(TreeNode { remain: &a[..], parent: None });
    assert_eq!(node1.remain, "Hello world".to_string());
    let node2 = TreeNode { remain: &node1.remain[1..], parent: Some(node1.clone()) };
    assert_eq!(node2.remain, "ello world".to_string());
    assert_eq!(node2.parent.unwrap().remain, "Hello world".to_string());

    let mut node3 = TreeNode { remain: &node1.remain[2..], parent: None };
    node3.parent = Some(node1.clone());
    assert_eq!(node3.remain, "llo world".to_string());
    assert_eq!(node3.parent.unwrap().remain, "Hello world".to_string());
}


#[test]
fn test_rc_vector() {
    let a = "Hello world".to_string();
    let node1 = Rc::new(TreeNode { remain: &a[..], parent: None });
    assert_eq!(node1.remain, "Hello world".to_string());

    let mut v: Vec<Rc<TreeNode>> = Vec::new();
    let node2 = Rc::new(TreeNode { remain: &node1.remain[1..], parent: Some(node1.clone()) });
    v.push(node2.clone());

    let node3 = Rc::new(TreeNode { remain: &node1.remain[2..], parent: Some(node1.clone()) });
    v.push(node3.clone());

    assert_eq!(v.get(0).unwrap().remain, "ello world".to_string());
    assert_eq!(v.get(1).unwrap().remain, "llo world".to_string());

    let mut v2: Vec<Rc<TreeNode>> = Vec::new();
    for item in v {
        let node = Rc::new(TreeNode { remain: &item.remain[1..], parent: Some(item.clone()) });
        v2.push(node);
    }

    assert_eq!(v2.len(), 2);
}
