#[derive(Debug())]
struct linkedlist {
    head: pointer,
}

#[derive(Debug)]
struct Node {
    element: i32,
    next: pointer,
}

type pointer = Option<Box<Node>>;

fn main() {
    /* let list = Node{element: 1, next: None};

    let list = Node{element: 1, next: Some(Box::new(Node {
        element: 2, next: Some(Box::new(Node {
            element: 3, next: None
        }))
    }))};
    let list = linkedlist {head: Node{element: 1, next: None}}  */

    let list = linkedlist{head: None};
    let list = linkedlist{head: Some(Box::new(Node {element: 100, next: (
        Some(Box::new(Node  {
            element: 200, next: None
        }))
    )}))};

    println!("{:?}", list.head.unwrap().as_ref().element);
    println!("{:?}", list.head) 
}
