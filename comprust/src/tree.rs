use crate::heap::Heap;

#[derive(Debug)]
pub struct BTree {
    pub root: Node,
}

/// An implementations of a binary tree, constructed from a min heap.
impl BTree {
    pub fn new(heap: &mut Heap) -> Self {
        while heap.size() > 1 {
            let tmp1 = heap.delete();
            let tmp2 = heap.delete();
            let tmp3 = Node::new(
                tmp1.weight() + tmp2.weight(),
                None,
                Some(Box::new(tmp1)),
                Some(Box::new(tmp2)),
                false,
            );
            heap.insert(tmp3);
        }
        let btree_root = heap.delete();
        BTree { root: btree_root }
    }
}

#[derive(Debug, Clone, Default)]
/// An implementation of a Leaf/Internal tree node.
pub struct Node {
    /// if `is_leaf=true` this represents the frequency of a letter.
    weight: usize,
    pub(crate) value: Option<String>,
    pub(crate) left: Option<Box<Self>>,
    pub(crate) right: Option<Box<Self>>,
    pub(crate) is_leaf: bool,
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
