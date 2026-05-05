use std::cmp::max;

use nalgebra::DMatrix;

use crate::{image_chunking, regression_apply};

use nalgebra::{self as na, U2, DVector, dmatrix};



#[derive(Clone)]
pub struct Quad {
    nodes: [usize; 4] 
}

#[derive(Clone)]
pub enum Node<T: Clone> {
    Quad(Quad),
    Terminal(T),
    Empty,
}

struct NodeRegestry<T: Clone> {
    nodes: Vec<Node<T>>
}

pub struct QuadTree<T: Clone> {
    node: usize,
    regestry: NodeRegestry<T>,
}



impl<T: Clone> NodeRegestry<T> {

    fn new() -> Self {
        NodeRegestry { nodes: Vec::new() }
    }

    fn create_node(&mut self) -> usize {
        let idx = self.nodes.len();
        self.nodes.push(Node::Empty);
        idx
    }

    fn set_node(&mut self, id: usize, node: Node<T>) {
        self.nodes[id] = node;
    }

    fn create_quad(&mut self, nodes: [usize; 4]) -> usize {
        let idx = self.nodes.len();
        self.nodes.push(Node::Quad(Quad::new(nodes)));
        idx
    }

    fn regesterNode(&mut self, node: Node<T>) -> usize {
        let idx = self.nodes.len();
        self.nodes.push(node);
        idx
    }

    fn get(&self, id: usize) -> &Node<T> {
        &self.nodes[id]
    }

    fn get_mut(&mut self, id: usize) -> &mut Node<T> {
        &mut self.nodes[id]
    }
    
}



/// Quad is a node split into 4 children, the orienation for the indexes relative to coordinates are as follows
///  +----------> +x
///  | +---+---+
///  | | 0 | 1 |
///  | +---+---+
///  | | 2 | 3 |
///  | +---+---+
///  V
/// +y
impl Quad {
    fn new(nodeIDs: [usize; 4]) -> Self {
        Quad{nodes: nodeIDs}
    }
    fn is_terminal<T: Clone>(&self, regestry: &NodeRegestry<T>) -> bool {
        for id in self.nodes {
            match &regestry.nodes[id] {
                Node::Terminal(_) | Node::Empty => continue,
                Node::Quad(quad) => {
                    println!("Looking for terminal, found quad {}", quad.depth(regestry));
                    return false;
                },
                Node::Empty => {
                    println!("Looking for terminal, found Empty");

                }
            }
        }
        return true;
    }

    pub fn depth<T: Clone>(&self, regestry: &NodeRegestry<T>) -> usize {
        let mut max_child_depth = 0;
        for id in self.nodes.iter() {
            match &regestry.nodes[*id] {
                Node::Quad(quad) => {
                    let child_depth = quad.depth(regestry);
                    if child_depth > max_child_depth {
                        max_child_depth = child_depth;
                    }
                },
                _ => continue,
            }
        }
        return max_child_depth + 1;
    }

    pub fn min_depth<T: Clone> (&self, regestry: &NodeRegestry<T>) -> usize {
        let mut min_child_depth = 1000000000000;
        for id in self.nodes {
            match &regestry.nodes[id] {
                Node::Quad(quad) => {
                    let child_depth = quad.min_depth(regestry);
                    if child_depth < min_child_depth {
                        min_child_depth = child_depth;
                    }
                },
                _ => continue,
            }
        }
        return min_child_depth + 1;
    }
}



impl<T: Clone> Node<T> {
    fn new_from_depth(depth: usize, init: &dyn Fn((usize, usize)) -> T, offset: (usize, usize), regestry: &mut NodeRegestry<T>) -> usize {
        if depth == 0 {
            return regestry.regesterNode(Node::Terminal(init((offset.0, offset.1))));
        }
        let child_width = 1<<(depth-1);
        // let node_id = regestry.create_node();
        let nodeA = Self::new_from_depth(depth-1, init, offset, regestry);
        let nodeB = Self::new_from_depth(depth-1, init, (offset.0 + child_width, offset.1), regestry);
        let nodeC = Self::new_from_depth(depth-1, init, (offset.0 , offset.1 + child_width), regestry);
        let nodeD = Self::new_from_depth(depth-1, init, (offset.0 + child_width, offset.1 + child_width ), regestry);
        let node_id = regestry.create_quad([
            nodeA, nodeB, nodeC, nodeD
        ]);
        // regestry.set_node(node_id, Node::Quad(Quad::new(children)));
        node_id
    }

    fn new_from_depth_ranged(depth: usize, range: (usize, usize), init: &dyn Fn((usize, usize)) -> T, offset: (usize, usize), regestry: &mut NodeRegestry<T>) -> usize {
        if depth == 0 {
            return regestry.regesterNode(Node::Terminal(init((offset.0, offset.1))));
        }
        let child_width = 1<<(depth-1);
        // Build 4 child nodes
        // If the child is fully in the range, call new_from_depth and stop using the range for that path
        // if the child is partially in the range, recusively call this function, adapting the range and offset as needed
        // If the child is fully out of the range, set it to empty
        let node_id = regestry.create_node();
        let quad: [usize; 4] = [
            // Top left quadrant
            
            if range.0 >= child_width && range.1 >= child_width { // If quad is fully in the specified range
                Self::new_from_depth(depth-1, init, offset, regestry) // if able to build quad without range, fillout out to depth-1
            } else {
                Self::new_from_depth_ranged(depth-1, range, init, offset, regestry) // If not limit the expansion with a range 
            },
            // Top right quadrant
            if range.0 <= child_width {
                regestry.regesterNode(Node::Empty) // If quad is fully out of range 
            } else {
                Self::new_from_depth_ranged(depth-1,
                    (range.0 - child_width, range.1),
                    init, 
                    (offset.0 + child_width, offset.1),
                    regestry
                )
            },
            //  Bottom Left quadrant
            if range.1 <= child_width {
                regestry.regesterNode(Node::Empty) // If quad is fully out of range 
            } else {
                Self::new_from_depth_ranged(depth-1,
                    (range.0, range.1 - child_width),
                    init, 
                    (offset.0, offset.1 + child_width),
                    regestry
                )
            },
            // Bottom Right Quadrant
            if range.0 <= child_width || range.1 <= child_width {
                regestry.regesterNode(Node::Empty) // If quad is fully out of range 
            } else {
                Self::new_from_depth_ranged(depth-1,
                    (range.0 - child_width, range.1 - child_width),
                    init, 
                    (offset.0 + child_width, offset.1 + child_width),
                    regestry
                )
            }
        ];
        regestry.set_node(node_id, Node::Quad(Quad::new(quad)));
        node_id
    }

}




// // Iterator struct for all terminal nodes in the quad tree
// pub struct QuadTreeTerminalIter<'a, T> {
//     qt_stack: Vec::<(&'a Quad, usize)>,
//     cur: Option<&'a Node<T>>,
// }

// pub struct QuadTreeDepthIter<'a, T> {
//     qt_stack: Vec::<(&'a Quad<T>, usize)>,
//     cur: Option<&'a Quad<T>>,
//     depth: usize,
// }


impl<T: Clone> QuadTree<T> {

    pub fn new() -> Self {
        let mut reg = NodeRegestry::<T>::new();
        QuadTree{node: reg.regesterNode(Node::Empty), regestry: reg}
    }

    pub fn new_grid(xWidth: usize, yWidth: usize, init: &dyn Fn((usize, usize)) -> T) -> Self {
        if xWidth == 0 || yWidth == 0 {
            return Self::new();
        }
        // Required number of depth to fit max width and height equal to floor( log2( max(width,height) ) )
        let depth = usize::BITS - (max(xWidth, yWidth)-1).leading_zeros();

        let mut regestry = NodeRegestry::new();
        let node = Node::new_from_depth_ranged(depth as usize, (xWidth, yWidth), init, (0, 0), &mut regestry);

        return QuadTree{ node: node, regestry: regestry};
    }

    // pub fn iter<'a>(&'a self) -> QuadTreeTerminalIter<'a, T> {
    //     return QuadTreeTerminalIter::new(self);
    // }

    // pub fn iter_depth<'a>(&'a self, depth: usize) -> QuadTreeDepthIter<'a, T> {
    //     return QuadTreeDepthIter::new(self, depth);
    // }

    
    // pub fn collapse(&mut self, depth: usize, collapser: fn([&Node<T>;4]) -> Option<Node<T>>) {
    //     let mut stack: Vec<(usize, usize)> = Vec::new();
    //     let mut cur = self.node;
    //     loop  {
    //         let res = self.descend(cur, &mut stack, depth);
            
    //         // Check if descent found a node, is a quad, and is terminal
    //         // If so call collapser, if Some(Node) recieved, overright node
    //         if let Some(next_node) = res {
    //             let node = self.regestry.get(next_node);
    //             if let Node::Quad(quad) = node {
    //                 let is_term = quad.is_terminal(&self.regestry);
    //                 if !is_term {
    //                     let collapse_res = collapser([
    //                         self.regestry.get(quad.nodes[0]),
    //                         self.regestry.get(quad.nodes[1]),
    //                         self.regestry.get(quad.nodes[2]),
    //                         self.regestry.get(quad.nodes[3]),
    //                     ]);
    //                     if let Some(collapse_node) = collapse_res {
    //                         self.regestry.set_node(next_node, collapse_node);
    //                     }
    //                 }
    //             }
    //         }

    //         if let Some(ascend_res) = self.ascend(&mut stack) {
    //             cur = ascend_res;
    //         } else {
    //             break;
    //         }


    //     }


    // }

    pub fn depth(&self) -> usize {
        if let Node::Quad(quad) = self.regestry.get(self.node) {
            quad.depth(&self.regestry)
        } else {
            0
        }
    }

    pub fn min_depth(&self) -> usize {
        if let Node::Quad(quad) = self.regestry.get(self.node) {
            quad.min_depth(&self.regestry)
        } else {
            0
        }
    }



}


impl<'c> QuadTree<image_chunking::Chunk<'c>> {
    fn descend(&mut self, cur: usize, stack: &mut Vec<(usize, usize)>, depth: usize) -> Option<usize> {
        if stack.len() == depth {
            return Some(cur);
        }
        if let Node::Quad(quad) = self.regestry.get(cur) {
            let next = quad.nodes[0];
            stack.push((cur, 1));
            self.descend(next, stack, depth)
        } else {
            None
        }
    }

    fn ascend(&mut self, stack: &mut Vec<(usize, usize)>) -> Option<usize> {
        if let Some((pop_node, pop_idx)) = stack.pop() {
            if pop_idx == 4 {
                self.ascend(stack)
            } else {
                if let Node::Quad(quad) = self.regestry.get(pop_node) {
                    Some(quad.nodes[pop_idx])
                } else {
                    panic!("Unexpected Non-quad in internal stack during ascent while collapsing")
                }
            }
        } else {
            // Ascended to the root without finding next node to descend
            None
        }

    }

    fn collapse<'b, 'a>(children: [&Node<image_chunking::Chunk<'a>>; 4], reg_mat: &DMatrix<f64>, order_mat:&DMatrix<f64>) -> Option<Node<image_chunking::Chunk<'a>>> {
        println!("Collapsing children");
        let mut chunks: [&image_chunking::Chunk; 4] = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
        let mut children_cost_r = 0.0;
        let mut children_cost_g = 0.0;
        let mut children_cost_b = 0.0;
        for (i, child) in children.iter().enumerate() {
            if let Node::<image_chunking::Chunk>::Terminal(child_chunk) = *child {
                children_cost_r += child_chunk.regression_red.cost;
                children_cost_g += child_chunk.regression_green.cost;
                children_cost_b += child_chunk.regression_blue.cost;
                chunks[i] = child_chunk;
            } else {
                println!("Non-Terminal found, not reducing");
                return None;
            }
        }

        // let mut new_chunk = image_chunking::Chunk::new_combine(&chunks);
        let mut new_chunk = image_chunking::Chunk::new(chunks[0].image, chunks[0].coordinate, chunks[0].size *2);

        regression_apply::applyRegressionToChunk(reg_mat, order_mat, 1, &mut new_chunk, None);

        println!("children cost: {} {} {}", children_cost_r, children_cost_g, children_cost_b);
        println!("parent cost: {} {} {}", new_chunk.regression_red.cost, new_chunk.regression_red.cost, new_chunk.regression_red.cost);

        if new_chunk.regression_red.cost + new_chunk.regression_red.cost + new_chunk.regression_red.cost
            < children_cost_r + children_cost_g + children_cost_b {
            
            println!("Collapsing children\n");
            return Some(Node::Terminal(new_chunk));
        } else {
            println!("Not collapsing children\n");
            None
        }
        
    }

    // init: &dyn Fn((usize, usize)) -> T
    pub fn collapse_depth<'a: 'c>(& mut self, depth: usize, reg_mat: &DMatrix<f64>, order_mat: &DMatrix<f64>) {
        let mut stack: Vec<(usize, usize)> = Vec::new();
        let mut cur = self.node;
        loop  {
            let res = self.descend(cur, &mut stack, depth);
            
            // Check if descent found a node, is a quad, and is terminal
            // If so call collapser, if Some(Node) recieved, overright node
            if let Some(next_node) = res {
                println!("Descent found a node {} {}", next_node, stack.len());
                let node = self.regestry.get(next_node);
                if let Node::Quad(quad) = node {
                    let is_term = quad.is_terminal(&self.regestry);

                    if is_term {
                        println!("It's terminal");

                        let node1 = self.regestry.get(quad.nodes[0]);
                        let node2 =self.regestry.get(quad.nodes[1]);
                        let node3 =self.regestry.get(quad.nodes[2]);
                        let node4 =self.regestry.get(quad.nodes[3]);
                        let collapse_res = QuadTree::collapse([
                            node1,
                            node2,
                            node3,
                            node4
                        ],
                            reg_mat,
                            order_mat,
                        );
                        if let Some(collapse_node) = collapse_res {
                            self.regestry.set_node(next_node, collapse_node.clone());
                        }
                    }
                } else {
                    match node {
                        Node::Quad(quad) => println!("Quad"),
                        Node::Terminal(_) => println!("terminal"),
                        Node::Empty => println!("Empty"),
                    }
                    println!("Found non quad");
                }
            }

            if let Some(ascend_res) = self.ascend(&mut stack) {
                cur = ascend_res;
            } else {
                break;
            }


        }


    }

}

// // Function called on quads guaranteed to be consisting of terminals
// fn collapser<T>(node: Quad<T>) -> Node<T> {
//     if (_) {
//         return Node::Terminal(todo!());
//     }
//     return Node::Quad(node);
// }

// impl<'a, T> QuadTreeTerminalIter<'a, T> {
//     fn new(qt: &'a QuadTree<T>) -> Self {
//         QuadTreeTerminalIter {
//             qt_stack: Vec::new(),
//             cur: Some(&qt.node),
//         }

//     }
// }

// impl<T: Copy> Iterator for QuadTreeTerminalIter<'_, T> {
//     // We can refer to this type using Self::Item
//     type Item = T;


//     fn next(&mut self) -> Option<Self::Item> {

//         match self.cur {
//             None => {
//                 return None;
//             }
//             Some(Node::Quad(quad)) => { 
//                 // If its a quad assume we haven't explored it yet, descent untill terminal
//                 let new_cur = &quad.nodes[0];
//                 self.qt_stack.push((quad, 1)); // push quad with next idx to explore
//                 self.cur = Some(new_cur);
//                 self.next()
//             }
//             Some(Node::Terminal(ret)) => {
//                 let mut poped_idx;
//                 let mut poped_node: &Quad<T>;
//                 // idx of 4 mean there is no child left to iterate through on this node, keep going up the stack
//                 loop { // Do while poped_idx == 4
//                     (poped_node, poped_idx) = match self.qt_stack.pop() {
//                         Some(poped) => poped,
//                         None => {
//                             self.cur = None; // if reached end of stack, then iteration finished after current
//                             return Some(*ret); 
//                         },
//                     };
//                     if poped_idx != 4 {break;}
//                 }
//                 self.cur = Some(&poped_node.nodes[poped_idx]);
//                 self.qt_stack.push((poped_node, poped_idx+1));
//                 Some(*ret)
//             }
//             Some(Node::Empty) => {
//                 let mut poped_idx = 3;
//                 let mut poped_node: &Quad<T>;
//                 // idx of 4 mean there is no child left to iterate through on this node, keep going up the stack
//                 loop { // Do while poped_idx == 4
//                     (poped_node, poped_idx) = match self.qt_stack.pop() {
//                         Some(poped) => poped,
//                         None => {
//                             self.cur = None; // if reached end of stack, then iteration finished after current
//                             return self.next(); // Skip empty by recalling next
//                         },
//                     };
//                     if poped_idx != 4 {break;}
//                 }
//                 self.cur = Some(&poped_node.nodes[poped_idx]);
//                 self.qt_stack.push((poped_node, poped_idx+1));
//                 self.next() // Skep empty by recalling next
//             }
//         }
//     }
// }

// impl<'a, T> QuadTreeDepthIter<'a, T> {
//     fn new(qt: &'a QuadTree<T>, depth: usize) -> Self {
//         if let Node::Quad(quad) = &qt.node {
//             QuadTreeDepthIter {
//                 qt_stack: Vec::new(),
//                 cur: Some(quad),
//                 depth: depth,
//             }
//         } else {
//             QuadTreeDepthIter {
//                 qt_stack: Vec::new(),
//                 cur: None,
//                 depth: depth,
//             }
//         }   
//     }

//     // Ascend to the nearest quad with unexplored quad child.
//     // self.cur is set to the found child, if none found, set to None
//     fn ascend_nearest(&mut self) {
//         let mut poped_idx;
//         let mut poped_node: &Quad<T>;
//         // Ascend finding next quad child to explore
//         // If poped_idx == 4, then there is no child left to explore on this node, keep going up the stack
//         // If poped_idx != 4, then check if the next child is a quad, 
//         // if so set it as cur and return, else keep going up the stack
//         loop {

//             // Pop last node and idx from stack,
//             // if stack is empty the we ascended to the top of the tree
//             // And no quads are left to explore, set cur to None and return
//             (poped_node, poped_idx) = match self.qt_stack.pop() {
//                 Some(poped) => poped,
//                 None => {
//                     self.cur = None; // if reached end of stack, then iteration finished after current
//                     return; 
//                 },
//             };
//             self.depth += 1;

//             // If not all children explored on this quad, check rest of children for quad
//             if poped_idx != 4 {
//                 // Find if the next child is a quad, if so set it as cur and return, else keep going up the stack
//                 let mut idx: usize = poped_idx;
//                 for child in poped_node.nodes[idx..].iter() {
//                     if let Node::Quad(new_quad) = child {
//                         // If quad child is found repush poped quad to stack with updated idx, update cur, and return
//                         self.qt_stack.push((poped_node, idx+1));
//                         self.depth -= 1;
//                         self.cur = Some(new_quad);
//                         return;
//                     }
//                     idx += 1;
//                 }
//             }
//         }
//     }
// }

// impl<'a, T: Copy> Iterator for QuadTreeDepthIter<'a, T> {
//     // We can refer to this type using Self::Item
//     type Item = &'a Quad<T>;

//     fn next(&mut self) -> Option<Self::Item> {

//         match self.cur {
//             None => {
//                 return None;
//             }
//             Some(quad) => { 
//                 // If at depth return the quad (depth == 0)
//                 // else if depth != 0, descend step, and recursively call next
//                 if self.depth != 0 {
                    
//                     // Find the first child in the current quad that is itself a quad.
//                     let mut idx: usize = 0;
//                     for child in quad.nodes[idx..].iter() {
//                         if let Node::Quad(new_quad) = child {
//                             // If quad child is found push last quad to stack with explored idx,
//                             // update cur to the new found quad, and recall next to keep descending until depth is reached
//                             self.qt_stack.push((quad, idx+1));
//                             self.depth -= 1;
//                             self.cur = Some(&new_quad);
//                             return self.next();
//                         }
//                         idx += 1;
//                     }

//                     // If no quad child is found, then this node is terminal
//                     // ascend back to unexplored quad and recall next()
//                     self.ascend_nearest();
//                     return self.next();
//                 } else {
//                     // If at depth return the current quad, and reascend for next call
//                     let ret_quad = quad;
//                     self.ascend_nearest();
//                     return Some(&ret_quad);
//                 }
//             }
//         }
//     }
// }