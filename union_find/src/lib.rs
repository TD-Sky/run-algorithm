use std::collections::HashMap;

pub struct UF {
    parents: HashMap<u32, Option<u32>>,
    verts: HashMap<u32, usize>,
}

#[allow(dead_code)]
impl UF {
    pub fn new(vids: &Vec<u32>) -> Self {
        let mut parents = HashMap::with_capacity(vids.len());
        let mut verts = HashMap::with_capacity(vids.len());
        for v in vids {
            parents.insert(*v, None);
            verts.insert(*v, 1);
        }
        Self { parents, verts }
    }

    pub fn connected(&self, v_1: u32, v_2: u32) -> bool {
        self.find(v_1) == self.find(v_2)
    }

    pub fn find(&self, mut vid: u32) -> u32 {
        while let Some(parent) = self.parents.get(&vid) {
            match parent {
                Some(parent_id) => vid = *parent_id,
                None => return vid,
            }
        }
        vid
    }

    pub fn union(&mut self, v_1: u32, v_2: u32) {
        let root_1 = self.find(v_1);
        let root_2 = self.find(v_2);

        let vs_1 = self.verts.get(&root_1).copied().unwrap();
        let vs_2 = self.verts.get(&root_2).copied().unwrap();

        if root_1 == root_2 {
            return;
        }

        if vs_1 < vs_2 {
            self.parents.entry(root_1).and_modify(|v| {
                v.replace(root_2);
            });
            self.verts.entry(root_2).and_modify(|vs| *vs += vs_1);
        } else {
            self.parents.entry(root_2).and_modify(|v| {
                v.replace(root_1);
            });
            self.verts.entry(root_1).and_modify(|vs| *vs += vs_2);
        }
    }
}
