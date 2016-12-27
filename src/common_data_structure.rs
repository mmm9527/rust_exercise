//栈
#[derive(Debug)]
struct Stack<T> {
    top: Option<Box<StackNode<T>>>
}

#[derive(Debug, Clone)]
struct StackNode<T> {
    val: T,
    next: Option<Box<StackNode<T>>>
}

impl<T> StackNode<T> {
    fn new(val: T) -> StackNode<T> {
        StackNode {
            val: val,
            next: None
        }
    }
}

impl<T> Stack<T> {
    fn new() -> Stack<T> {
        Stack {
            top: None
        }
    }

    fn push(&mut self, val: T) {
        let mut node = StackNode::new(val);
        let next = self.top.take();
        node.next = next;
        self.top = Some(Box::new(node));
    }

    fn pop(&mut self) -> Option<T> {
        let val = self.top.take();
        match val {
            None => None,
            Some(mut x) => {
                self.top = x.next.take();
                Some(x.val)
            }
        }
    }
}
//队列
#[derive(Debug)]
struct Queue<T> {
    qdata: Vec<T>,
}

impl<T> Queue<T> {
    fn new() -> Queue<T> {
        Queue {
            qdata: Vec::new(),
        }
    }

    fn push(&mut self, item: T) {
        self.qdata.push(item)
    }

    fn pop(&mut self) -> T {
        self.qdata.remove(0)
    }
}
//二叉树
type TreeNode<K, V> = Option<Box<Node<K, V>>>;

#[derive(Debug)]
struct Node<K, V: ::std::fmt::Display> {
    left: TreeNode<K, V>,
    right: TreeNode<K, V>,
    key: K,
    value: V,
}

trait BinaryTree<K, V> {
    fn pre_order(&self);
    fn in_order(&self);
    fn pos_order(&self);
}

trait BinarySearchTree<K: PartialOrd, V>: BinaryTree<K, V> {
    fn insert(&mut self, key: K, value: V);
}

impl<K, V: ::std::fmt::Display> Node<K, V> {
    fn new(key: K, value: V) -> Self {
        Node {
            left: None,
            right: None,
            value: value,
            key: key,
        }
    }
}

impl<K: PartialOrd, V: ::std::fmt::Display> BinarySearchTree<K, V> for Node<K, V> {
    fn insert(&mut self, key: K, value: V) {
        if self.key < key {
            if let Some(ref mut right) = self.right {
                right.insert(key, value);
            } else {
                self.right = Some(Box::new(Node::new(key, value)));
            }
        } else {
            if let Some(ref mut left) = self.left {
                left.insert(key, value);
            } else {
                self.left = Some(Box::new(Node::new(key, value)));
            }
        }
    }
}

impl<K: PartialOrd, V: ::std::fmt::Display> BinaryTree<K, V> for Node<K, V> {
    fn pre_order(&self) {
        println!("{}", self.value);
        if let Some(ref left) = self.left {
            left.pre_order();
        }
        if let Some(ref right) = self.right {
            right.pre_order();
        }
    }

    fn in_order(&self) {
        if let Some(ref left) = self.left {
            left.pre_order();
        }
        println!("{}", self.value);
        if let Some(ref right) = self.right {
            right.pre_order();
        }
    }

    fn pos_order(&self) {
        if let Some(ref left) = self.left {
            left.pre_order();
        }
        if let Some(ref right) = self.right {
            right.pre_order();
        }
        println!("{}", self.value);
    }
}

type BST<K, V> = Node<K, V>;

//优先队列
#[derive(Debug)]
struct PriorityQueue<T> where T: PartialOrd + Clone {
    pq: Vec<T>
}

impl<T> PriorityQueue<T> where T: PartialOrd + Clone {
    fn new() -> PriorityQueue<T> {
        PriorityQueue { pq: Vec::new() }
    }

    fn len(&self) -> usize {
        self.pq.len()
    }

    fn is_empty(&self) -> bool {
        self.pq.len() == 0
    }

    fn insert(&mut self, value: T) {
        self.pq.push(value);
    }

    fn max(&mut self) -> Option<T> {
        if self.is_empty() { return None }
        let max = self.max_index();
        Some(self.pq[max].clone())
    }

    fn min(&mut self) -> Option<T> {
        if self.is_empty() { return None }
        let min = self.min_index();
        Some(self.pq[min].clone())
    }

    fn delete_max(&mut self) -> Option<T> {
        if self.is_empty() { return None; }
        let max = self.max_index();
        Some(self.pq.remove(max).clone())
    }

    fn delete_min(&mut self) -> Option<T> {
        if self.is_empty() { return None; }
        let min = self.min_index();
        Some(self.pq.remove(min).clone())
    }

    fn max_index(&self) -> usize {
        let mut max = 0;
        for i in 1..self.pq.len() - 1 {
            if self.pq[max] < self.pq[i] {
                max = i;
            }
        }
        max
    }

    fn min_index(&self) -> usize {
        let mut min = 0;
        for i in 0..self.pq.len() - 1 {
            if self.pq[i] < self.pq[i + 1] {
                min = i;
            }
        }
        min
    }
}

//链表
use self::List::*;

enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    // 创建一个空链表
    fn new() -> List {
        // `Nil` 是 `List`类型的。因为前面我们使用了 `use List::*;`
        // 所以不需要 List::Nil 这样使用
        Nil
    }

    // 在前面加一个元素节点，并且链接旧的链表和返回新的链表
    fn prepend(self, elem: u32) -> List {
        // `Cons` 也是 List 类型的
        Cons(elem, Box::new(self))
    }

    // 返回链表的长度
    fn len(&self) -> u32 {
        // `self` 的类型是 `&List`, `*self` 的类型是 `List`,
        // 匹配一个类型 `T` 好过匹配一个引用 `&T`
        match *self {
            // 因为`self`是借用的，所以不能转移 tail 的所有权
            // 因此使用 tail 的引用
            Cons(_, ref tail) => 1 + tail.len(),
            // 基本规则：所以空的链表长度都是0
            Nil => 0
        }
    }
    // 返回连链表的字符串表达形式
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                // `format!` 和 `print!` 很像
                // 但是返回一个堆上的字符串去替代打印到控制台
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}

//图
#[derive(Debug)]
struct GNode {
    nodeid: usize,
    nodename: String,
}

#[derive(Debug, Clone)]
struct Edge {
    edge: bool,
}

#[derive(Debug)]
struct Graphadj {
    nodenums: usize,
    graphadj: Vec<Vec<Edge>>,
}

impl GNode {
    fn new(nodeid: usize, nodename: String) -> GNode {
        GNode {
            nodeid: nodeid,
            nodename: nodename,
        }
    }
}

impl Edge {
    fn new() -> Edge {
        Edge {
            edge: false,
        }
    }
    fn have_edge() -> Edge {
        Edge {
            edge: true,
        }
    }
}

impl Graphadj {
    fn new(nums: usize) -> Graphadj {
        Graphadj {
            nodenums: nums,
            graphadj: vec![vec![Edge::new(); nums]; nums],
        }
    }

    fn insert_edge(&mut self, v1: GNode, v2: GNode) {
        match v1.nodeid < self.nodenums && v2.nodeid < self.nodenums {
            true => {
                self.graphadj[v1.nodeid][v2.nodeid] = Edge::have_edge();
                //下面这句注释去掉相当于把图当成无向图
                //self.graphadj[v2.nodeid][v1.nodeid] = Edge::have_edge();
            }
            false => {
                panic!("your nodeid is bigger than nodenums!");
            }
        }
    }
}


pub fn common_data_structure() {
    #[derive(PartialEq, Eq, Debug)]
    struct TestStruct {
        a: i32,
    }
    let a = TestStruct { a: 5 };
    let b = TestStruct { a: 9 };

    let mut s = Stack::<&TestStruct>::new();
    assert_eq!(s.pop(), None);

    s.push(&a);
    s.push(&b);
    println!("{:?}", s);

    assert_eq!(s.pop(), Some(&b));
    assert_eq!(s.pop(), Some(&a));
    assert_eq!(s.pop(), None);

    let mut q = Queue::new();
    q.push(1);
    q.push(2);
    println!("{:?}", q);
    q.pop();
    println!("{:?}", q);
    q.pop();
    println!("{:?}", q);

    let mut root = BST::<i32, i32>::new(3, 4);
    root.insert(2, 3);
    root.insert(4, 6);
    root.insert(5, 5);
    root.insert(6, 6);
    root.insert(1, 8);
    println!("{:?}", root);
    if let Some(ref left) = root.left {
        assert_eq!(left.value, 3);
    }

    if let Some(ref right) = root.right {
        assert_eq!(right.value, 6);
        if let Some(ref right) = right.right {
            assert_eq!(right.value, 5);
        }
    }
    println!("Pre Order traversal");
    root.pre_order();
    println!("In Order traversal");
    root.in_order();
    println!("Pos Order traversal");
    root.pos_order();

    let mut pq = PriorityQueue::new();
    pq.insert(3);
    pq.insert(2);
    pq.insert(1);
    pq.insert(4);
    assert!(pq.min().unwrap() == 1);

    let mut list = List::new();
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());

    let mut g = Graphadj::new(2);
    let v1 = GNode::new(0, "v1".to_string());
    let v2 = GNode::new(1, "v2".to_string());
    g.insert_edge(v1, v2);
    println!("{:?}", g);
}