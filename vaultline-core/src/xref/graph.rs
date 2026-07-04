//! Directed xref graph for deferred verification paths.


use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use crate::xref::table::{XrefEntry, XrefTable};

#[derive(Debug, Default)]
pub struct XrefGraph {
    edges: BTreeMap<u32, Vec<u32>>,
}

impl XrefGraph {
    pub fn from_table(table: &XrefTable) -> XrefGraph {
        let mut g = XrefGraph::default();
        for e in table.entries() {
            g.edges.entry(e.from_id).or_default().push(e.to_id);
        }
        g
    }
    pub fn neighbors(&self, id: u32) -> &[u32] {
        self.edges.get(&id).map(|v| v.as_slice()).unwrap_or(&[])
    }
    pub fn has_path(&self, start: u32, goal: u32, limit: usize) -> bool {
        let mut stack = vec![start];
        let mut seen = BTreeMap::new();
        while let Some(n) = stack.pop() {
            if n == goal {
                return true;
            }
            if *seen.get(&n).unwrap_or(&0) >= limit {
                continue;
            }
            *seen.entry(n).or_insert(0) += 1;
            for &next in self.neighbors(n) {
                stack.push(next);
            }
        }
        false
    }
}
