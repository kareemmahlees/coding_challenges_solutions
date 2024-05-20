use crate::tree::Node;

#[derive(Default, Debug)]
/// Implementation of a `min-heap`.
pub(crate) struct Heap {
    elems: Vec<Node>,
}

impl Heap {
    pub fn size(&self) -> usize {
        self.elems.len()
    }
    fn get_parent_idx(&self, idx: usize) -> Option<usize> {
        if idx == 0 {
            return None;
        }

        // to prevent overflow when doing (1 - 2)
        if idx == 1 {
            return Some(0);
        }
        Some((idx - 2).div_ceil(2))
    }

    fn left_child_idx(&self, idx: usize) -> Option<usize> {
        let lft_child_idx = (idx * 2) + 1;

        if lft_child_idx < self.elems.len() {
            Some(lft_child_idx)
        } else {
            None
        }
    }

    fn right_child_idx(&self, idx: usize) -> Option<usize> {
        let rt_child_idx = (idx * 2) + 2;

        if rt_child_idx < self.elems.len() {
            Some(rt_child_idx)
        } else {
            None
        }
    }

    /// Inserts and Heapyfies.
    pub(crate) fn insert(&mut self, elem: Node) -> Vec<Node> {
        self.elems.push(elem);
        let mut idx = self.elems.len() - 1;

        loop {
            if let Some(parent_idx) = self.get_parent_idx(idx) {
                if self.elems[parent_idx].weight() > self.elems[idx].weight() {
                    self.elems.swap(idx, parent_idx);
                    idx = parent_idx;

                    continue;
                } else {
                    break;
                }
            }
            break;
        }

        self.elems.clone()
    }

    /// Deletes and Heapyfies.
    pub(crate) fn delete(&mut self) -> Node {
        let elems_len = self.elems.len();
        self.elems.swap(0, elems_len - 1);
        let return_value = self.elems.pop().unwrap();

        let mut idx = 0;

        loop {
            if let Some(left_child_idx) = self.left_child_idx(idx) {
                if self.elems[left_child_idx].weight() < self.elems[idx].weight() {
                    self.elems.swap(idx, left_child_idx);
                    idx = left_child_idx;
                    continue;
                }
            }

            if let Some(right_child_idx) = self.right_child_idx(idx) {
                if self.elems[right_child_idx].weight() < self.elems[idx].weight() {
                    self.elems.swap(idx, right_child_idx);
                    idx = right_child_idx;
                    continue;
                }
            }
            break;
        }
        // root.clone()
        return_value
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn setup() -> Heap {
        let mut heap = Heap::default();

        let n1 = Node::new(0, None, None, None, false);
        let n2 = Node::new(1, None, None, None, false);
        let n3 = Node::new(2, None, None, None, false);

        heap.insert(n1);
        heap.insert(n2);
        heap.insert(n3);

        heap
    }

    #[test]
    fn test_get_parent_idx() {
        let heap = setup();

        assert_eq!(heap.get_parent_idx(0), None);
        assert_eq!(heap.get_parent_idx(1), Some(0));
        assert_eq!(heap.get_parent_idx(4), Some(1));
    }

    #[test]
    fn test_get_left_child_idx() {
        let heap = setup();

        assert_eq!(heap.left_child_idx(0), Some(1));
        assert_eq!(heap.left_child_idx(1), None);
    }

    #[test]
    fn test_get_right_child_idx() {
        let heap = setup();

        assert_eq!(heap.right_child_idx(0), Some(2));
        assert_eq!(heap.right_child_idx(2), None);
    }

    #[test]
    fn test_insert() {
        let mut heap = setup();

        heap.insert(Node::new(3, None, None, None, false));

        assert_eq!(heap.elems.len(), 4);
        assert_eq!(heap.elems.last().unwrap().weight(), 3)
    }

    #[test]
    fn test_delete() {
        let mut heap = setup();

        let root = heap.delete();

        assert_eq!(heap.elems.len(), 2);
        assert_eq!(root.weight(), 0);

        assert_eq!(heap.elems.first().unwrap().weight(), 1)
    }
}
