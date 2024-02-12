use core::fmt;
use std::collections::BTreeMap;

#[derive(Debug)]
pub enum HuffNode{
 Branch {
  weight: usize,
  left: Box<HuffNode>,
  right: Box<HuffNode>,
 },
 Leaf {
  char: char,
  weight: usize,
 }
}

impl HuffNode {
 pub fn get_weight(&self) -> usize {
  match *self {
   HuffNode::Branch { weight, .. }=> return weight,
   HuffNode::Leaf { weight, .. }=> return weight,
  }
 }

 pub fn create_branch(left: Box<HuffNode>, right: Box<HuffNode>) -> HuffNode {
  let weight = left.get_weight() + right.get_weight();
  HuffNode::Branch { weight, left: Box::new(*left), right: Box::new(*right) }
 }

 pub fn create_leaf(char: char, weight: usize) -> HuffNode {
  HuffNode::Leaf { char, weight }
 }
}

#[derive(Debug)]
pub struct HuffBTree {
 root: Option<Box<HuffNode>>
}

impl HuffBTree {
 pub fn new (root: Box<HuffNode>) -> Self {
  HuffBTree {
   root: Some(root)
  }
 }

 pub fn gen_tree(v:&mut HuffVector) -> HuffBTree {
  while v.frequencies.len() > 1 {
   let left = v.frequencies.remove(0);
   let right = v.frequencies.remove(0);

   let new_branch = HuffNode::create_branch(left, right);
   v.insert_node(Box::new(new_branch));
  }

  HuffBTree::new(v.frequencies.remove(0))
 }

 pub fn gen_table(&self) -> BTreeMap<char, HuffCode> {

  let mut huffman_table = BTreeMap::new();
  let mut code: Vec<bool> = Vec::new();

  fn traverse(node: &HuffNode, code: &mut Vec<bool>, huffman_table: &mut BTreeMap<char, HuffCode>) {
   match node {
    HuffNode::Branch {  left, right , ..} => {
     code.push(false);
     traverse(&left, code, huffman_table);
     code.pop();

     code.push(true);
     traverse(&right, code, huffman_table);
     code.pop();
    },
    HuffNode::Leaf { char, .. } => {
     huffman_table.insert(*char ,HuffCode::new(*char, code.clone()));
    }
   }
  }
  
  if let Some(root) = &self.root {
   traverse(root, &mut code, &mut huffman_table);
  }
  huffman_table
 }
}

#[derive(Debug)]
pub struct HuffVector {
 frequencies: Vec<Box<HuffNode>>
}

impl HuffVector {
 pub fn new() -> Self {
  HuffVector { 
   frequencies: Vec::new()
  }
 }

 pub fn gen_huff_vector(frequencies: Vec<(char, usize)>) -> Self {
  let mut huff_vector = HuffVector::new();

  for freq in frequencies {
   huff_vector.insert(freq)
  }

  huff_vector
 }
 
 pub fn insert(&mut self, freq: (char, usize)) {

  let new_leaf = HuffNode::create_leaf(freq.0, freq.1);
  let mut new_leaf_index = None;

  // Find insertion position
  for (index,freq) in self.frequencies.iter().enumerate() {
   if new_leaf.get_weight() < freq.get_weight() {
    new_leaf_index = Some(index);
    break;
   }
  }

  // Insert new element keeping the vector sorted by weight
  match new_leaf_index {
   Some(index) => self.frequencies.insert(index, Box::new(new_leaf)),
   None => self.frequencies.push(Box::new(new_leaf))
  }
 }

 pub fn insert_node(&mut self, node: Box<HuffNode>){
  let mut new_leaf_index = None;

  for (index,freq) in self.frequencies.iter().enumerate() {
   if node.get_weight() < freq.get_weight() {
    new_leaf_index = Some(index);
    break;
   }
  }

  match new_leaf_index {
   Some(index) => self.frequencies.insert(index, Box::new(*node)),
   None => self.frequencies.push(Box::new(*node))
  }
 }
}

#[derive(Clone, Debug)]
pub struct HuffCode {
 char: char,
 code: Vec<bool>,
 length: usize
}

impl HuffCode {
 pub fn new(char:char, code: Vec<bool>) -> Self {
  let length = code.len();
  HuffCode { 
   char,
   code,
   length
  }
 }

 pub fn get_header_data(&self) -> (char, Vec<bool>, usize) {
  (self.char, self.code.clone(), self.length)
 }

 pub fn get_code(&self) -> Vec<bool> {
  self.code.clone()
 }
}

impl fmt::Display for HuffCode {
 fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
  for bit in self.code.iter() {
   write!(f, "{}", if *bit {'1'} else {'0'} )?
  }
  Ok(())
 }
}

