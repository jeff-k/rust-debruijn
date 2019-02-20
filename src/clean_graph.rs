// Copyright 2017 10x Genomics

//! DeBruijn graph simplification routines. Currently tip-removal is implemented.
use graph::{DebruijnGraph, Node};
use std::fmt::Debug;
use std::marker::PhantomData;
use Kmer;

pub struct CleanGraph<K: Kmer, D, T1>
where
    T1: Fn(&Node<K, D>) -> bool,
{
    tip_predicate: T1,
    _k: PhantomData<K>,
    _d: PhantomData<D>,
}

impl<K: Kmer, D: Debug, T1> CleanGraph<K, D, T1>
where
    T1: Fn(&Node<K, D>) -> bool,
{
    pub fn new(tip_predicate: T1) -> CleanGraph<K, D, T1> {
        CleanGraph {
            tip_predicate,
            _k: PhantomData,
            _d: PhantomData,
        }
    }

    fn test_tip(&self, graph: &DebruijnGraph<K, D>, id: usize) -> Option<usize> {
        let node = graph.get_node(id);
        let exts = node.exts();
        if exts.num_exts_r() > 0 && exts.num_exts_l() > 0 {
            return None;
        }

        if (exts.num_exts_l() == 0 && exts.num_exts_r() <= 1)
            || (exts.num_exts_r() == 0 && exts.num_exts_l() <= 1)
        {
            if (self.tip_predicate)(&node) {
                return Some(id);
            }
        }

        None
    }

    pub fn find_bad_nodes(&self, graph: &DebruijnGraph<K, D>) -> Vec<usize> {
        (0..graph.len())
            .map(|i| self.test_tip(graph, i))
            .filter_map(|x| x)
            .collect()
    }
}
