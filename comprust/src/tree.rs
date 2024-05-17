#[derive(Debug)]
pub struct BTree {
    root: Node,
}

impl BTree {
    pub fn new(root: Node) -> Self {
        BTree { root }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Node {
    weight: usize,
    value: Option<String>,
    left: Option<Box<Self>>,
    right: Option<Box<Self>>,
    is_leaf: bool,
}

impl Node {
    pub fn new(
        weight: usize,
        value: Option<String>,
        left: Option<Box<Self>>,
        right: Option<Box<Self>>,
        is_leaf: bool,
    ) -> Self {
        Node {
            weight,
            value,
            left,
            right,
            is_leaf,
        }
    }

    pub fn weight(&self) -> usize {
        self.weight
    }
}
