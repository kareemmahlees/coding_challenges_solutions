use crate::tree::Node;

#[derive(Default, Debug)]
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
