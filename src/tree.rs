extern crate rand;
extern crate test;

use node::Node;
use node::Range;
use node::{insert,delete,search,min,max,is_interval_tree, min_pair, max_pair};
use iterators::RangePairIter;
use std::collections::Bound;


pub struct IntervalTree<D> {
    pub root: Option<Box<Node<D>>>
}

impl <D> IntervalTree<D>{

/// This function will construct a new empty IntervalTree.
/// # Examples
/// ```
/// extern crate interval_tree;
/// let mut t=interval_tree::IntervalTree::<i32>::new();
/// ```
    pub fn new() -> IntervalTree<D>{
        IntervalTree{root: None}
    }

/// This function will insert the key,value pair into the tree, overwriting the old data if the key is allready
/// part of the tree.
/// # Examples
/// ```
/// use interval_tree::Range;
/// 
/// let mut t=interval_tree::IntervalTree::<i32>::new();
/// t.insert(Range::new(2,2),25);
/// assert_eq!(t.get(Range::new(2,2)), Some(&25));
/// t.insert(Range::new(2,2),30);
/// assert_eq!(t.get(Range::new(2,2)), Some(&30));
/// ```
    pub fn insert(&mut self, key: Range, data: D) {
        match self.root.take() {
            Some(box_to_node) => self.root = Some(insert::<D>(key, data, box_to_node)),
            None => self.root = Some(Box::new(Node::new(key,data))),
        }
    }

/// This function will remove the key,value pair from the tree, doing nothing if the key is not
/// part of the tree.
/// # Examples
/// ```
/// use interval_tree::Range;
///
/// let mut t=interval_tree::IntervalTree::<i32>::new();
/// t.insert(Range::new(2,2),25);
/// t.delete(Range::new(2,2));
/// assert!(t.empty());
/// // deleting nonexistant keys doesn't do anything
/// t.delete(Range::new(3,3)); 
/// assert!(t.empty());
/// ```
    pub fn delete(&mut self, key: Range){
        match self.root.take() {
            Some(box_to_node) => self.root = delete(key,box_to_node),
            None => return
        }
    }

/// This function will return the Some(data) stored under the given key or None if the key is not
/// known.
/// # Examples
/// ```
/// use interval_tree::Range;
/// let mut t=interval_tree::IntervalTree::<i32>::new();
/// t.insert(Range::new(2,2),25);
/// assert_eq!(t.get(Range::new(2,2)), Some(&25));
/// assert_eq!(t.get(Range::new(3,3)), None);
///
/// ```
    pub fn get(&self, key: Range) -> Option<&D>{
        match self.root {
            Some(ref box_to_node) =>search(&key, box_to_node),
            None => None
        }
    }

/// This function will return the data stored under the given key or the default if the key is not
/// known.
/// # Examples
/// ```
/// use interval_tree::Range;
/// let mut t=interval_tree::IntervalTree::<i32>::new();
/// t.insert(Range::new(2,2),25);
/// assert_eq!(t.get_or(Range::new(2,2),&2000), &25);
/// assert_eq!(t.get_or(Range::new(3,3),&2000), &2000);
///
/// ```
    pub fn get_or<'a>(&'a self, key: Range, default: &'a D) -> &D{
        self.get(key).map_or(default, |data| data)
    }

/// This function will return true if the tree contains the given key, false otherwise
/// # Examples
/// ```
/// use interval_tree::Range;
/// let mut t=interval_tree::IntervalTree::<i32>::new();
/// t.insert(Range::new(2,2),25);
/// assert!(!t.contains(Range::new(3,3)));
/// assert!(t.contains(Range::new(2,2)));
///
/// ```
    pub fn contains(&self, key: Range) -> bool {
        self.get(key).is_some()
    }

/// This function will return true if the tree is empty, false otherwise.
/// # Examples
/// ```
/// use interval_tree::Range;
/// let mut t=interval_tree::IntervalTree::<i32>::new();
/// assert!(t.empty());
/// t.insert(Range::new(2,2),25);
/// assert!(!t.empty());
///
/// ```
    pub fn empty(&self) -> bool { self.root.is_none() }

/// This function will return the key/value pair with the smallest key in the tree, or None if the
/// tree is empty.
/// # Examples
/// ```
/// use interval_tree::Range;
/// let mut t=interval_tree::IntervalTree::<u64>::new();
/// t.insert(Range::new(2,2),25);
/// t.insert(Range::new(3,3),50);
/// assert_eq!(t.min().unwrap().0, &Range::new(2,2));
/// assert_eq!(t.min().unwrap().1, &25);
///
/// ```
    pub fn min<'a>(&'a self) -> Option<(&'a Range,&'a D)> {
        match self.root {
            Some(ref root) => Some(min_pair(root)),
            None => None
        }
    }

/// This function will return the key/value pair with the biggest key in the tree, or None if the
/// tree is empty.
/// # Examples
/// ```
/// use interval_tree::Range;
/// let mut t=interval_tree::IntervalTree::<i32>::new();
/// t.insert(Range::new(2,2),25);
/// t.insert(Range::new(3,3),50);
/// assert_eq!(t.max().unwrap().0, &Range::new(3,3));
/// assert_eq!(t.max().unwrap().1, &50);
///
/// ```
    pub fn max<'a>(&'a self) -> Option<(&'a Range,&'a D)> {
        match self.root {
            Some(ref root) => Some(max_pair(root)),
            None => None
        }
    }

/// This function will return a read only iterator for all (key,value) pairs in the tree.
/// # Examples
/// ```
/// # let mut t=interval_tree::IntervalTree::<i32>::new();
/// for (key,val) in t.iter() {
///     println!("{:?} -> {}",key,val)
/// }
///
/// ```
    pub fn iter(&self) -> RangePairIter<D>{
        RangePairIter::new(self, 0, 0xffff_ffff_ffff_ffff)
    }

/// This function will return a read only iterator for all (key,value) pairs between the two bounds (which can
/// be inclusive, exclusive or unbounded).
/// # Examples
/// ```
/// //[...]
/// # let mut t=interval_tree::IntervalTree::<i32>::new();
/// for (key,val) in t.range(9, 100) {
///     println!("{:?} -> {}",key,val)
/// }
///
/// ```
    pub fn range(&self, min: u64, max: u64) -> RangePairIter<D>{
        RangePairIter::new(self, min, max)
    }

    fn test_interval_tree(&self) -> bool {
        is_interval_tree(&self.root)
    }
}

#[test]
fn test_fuzz(){
    let mut t = IntervalTree::<i32>::new();
    for _ in 1..5000 {
        let decision = rand::random::<bool>();
        if  decision {
            let rnd = rand::random::<u64>()%500;
            let to_insert = Range::new(rnd,rnd);
            t.insert(to_insert, 1337);
            assert!(t.contains(to_insert));
            assert!(t.test_interval_tree());
        } else {
            let rnd = rand::random::<u64>()%500;
            let to_delete = Range::new(rnd, rnd);
            t.delete(to_delete);
            assert!(!t.contains(to_delete));
            assert!(t.test_interval_tree());
        };
    };
    return;
}