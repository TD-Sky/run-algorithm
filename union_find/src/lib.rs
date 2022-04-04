use std::collections::HashMap;
use std::convert::From;
use std::iter::FromIterator;
use std::ops::AddAssign;
use std::ptr::NonNull;

type NodeID = u32;

#[derive(Debug)]
pub struct UF {
    parent_of: HashMap<NodeID, NodeID>,
    weight_of: HashMap<NodeID, usize>,
}

impl FromIterator<NodeID> for UF {
    fn from_iter<T: IntoIterator<Item = NodeID>>(iter: T) -> Self {
        let mut weight_of = HashMap::new();

        for id in iter {
            weight_of.insert(id, 1);
        }

        Self {
            parent_of: HashMap::new(),
            weight_of,
        }
    }
}

impl<const N: usize> From<[NodeID; N]> for UF {
    fn from(src: [NodeID; N]) -> Self {
        UF::from_iter(src.into_iter())
    }
}

impl UF {
    pub fn find(&self, mut id: NodeID) -> NodeID {
        while let Some(&parent) = self.parent_of.get(&id) {
            id = parent;
        }

        id
    }

    pub fn connected(&self, id1: NodeID, id2: NodeID) -> bool {
        self.find(id1) == self.find(id2)
    }

    pub fn union(&mut self, id1: NodeID, id2: NodeID) {
        let root1 = self.find(id1);
        let root2 = self.find(id2);

        if root1 == root2 {
            return;
        }

        let mut weight1: NonNull<usize> = self.weight_of.get_mut(&root1).unwrap().into();
        let mut weight2: NonNull<usize> = self.weight_of.get_mut(&root2).unwrap().into();

        unsafe {
            if *weight1.as_ptr() >= *weight2.as_ptr() {
                self.parent_of.insert(root2, root1);
                weight1.as_mut().add_assign(*weight2.as_ptr());
            } else {
                self.parent_of.insert(root1, root2);
                weight2.as_mut().add_assign(*weight1.as_ptr());
            }
        }
    }
}
