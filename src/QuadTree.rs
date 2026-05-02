use std::cmp::max;



pub struct Quad<T> {
    nodes: Box<[Node<T>; 4]> 
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
impl<T> Quad<T> {
    fn new() -> Self {
        Quad{nodes: Box::new([Node::Empty, Node::Empty, Node::Empty, Node::Empty])}
    }
    fn is_terminal(&self) -> bool {
        for node in self.nodes.iter() {
            match node {
                Node::Terminal(_) | Node::Empty => continue,
                _ => return false,
            }
        }
        return true;
    }
}

enum Node<T> {
    Quad(Quad<T>),
    Terminal(T),
    Empty,
}

impl<T> Node<T> {
    fn new_from_depth(depth: usize, init: fn(usize, usize) -> T, offset: (usize, usize)) -> Self {
        if depth == 0 {
            return Node::Terminal(init(offset.0, offset.1));
        }
        let child_width = 1<<(depth-1);
        Node::Quad(
            Quad{nodes: Box::new([
                Self::new_from_depth(depth-1, init, offset),
                Self::new_from_depth(depth-1, init, (offset.0 + child_width, offset.1 ) ),
                Self::new_from_depth(depth-1, init, (offset.0 , offset.1 + child_width) ),
                Self::new_from_depth(depth-1, init, (offset.0 + child_width, offset.1 + child_width ) ),
            ])}
        )
    }

    fn new_from_depth_ranged(depth: usize, range: (usize, usize), init: fn(usize, usize) -> T, offset: (usize, usize)) -> Self {
        if depth == 0 {
            return Node::Terminal(init(offset.0, offset.1));
        }
        let child_width = 1<<(depth-1);
        // Build 4 child nodes
        // If the child is fully in the range, call new_from_depth and stop using the range for that path
        // if the child is partially in the range, recusively call this function, adapting the range and offset as needed
        // If the child is fully out of the range, set it to empty
        let mut quad: [Node<T>; 4] = [
            // Top left quadrant
            if range.0 >= child_width && range.1 >= child_width { // If quad is fully in the specified range
                Self::new_from_depth(depth-1, init, offset) // if able to build quad without range, fillout out to depth-1
            } else {
                Self::new_from_depth_ranged(depth-1, range, init, offset) // If not limit the expansion with a range 
            },
            // Top right quadrant
            if range.0 <= child_width {
                Node::Empty // If quad is fully out of range 
            } else {
                Self::new_from_depth_ranged(depth-1,
                    (range.0 - child_width, range.1),
                    init, 
                    (offset.0 + child_width, offset.1)
                )
            },
            //  Bottom Left quadrant
            if range.1 <= child_width {
                Node::Empty // If quad is fully out of range 
            } else {
                Self::new_from_depth_ranged(depth-1,
                    (range.0, range.1 - child_width),
                    init, 
                    (offset.0, offset.1 + child_width)
                )
            },
            // Bottom Right Quadrant
            if range.0 <= child_width && range.1 <= child_width {
                Node::Empty // If quad is fully out of range 
            } else {
                Self::new_from_depth_ranged(depth-1,
                    (range.0 - child_width, range.1 - child_width),
                    init, 
                    (offset.0 + child_width, offset.1 + child_width)
                )
            }
        ];
        Node::Quad(
            Quad{nodes: Box::new(quad)}
        )
    }

}

pub struct QuadTree<T> {
    node: Node<T>
}


// Iterator struct for all terminal nodes in the quad tree
pub struct QuadTreeTerminalIter<'a, T> {
    qt_stack: Vec::<(&'a Quad<T>, usize)>,
    cur: Option<&'a Node<T>>,
}

pub struct QuadTreeDepthIter<'a, T> {
    qt_stack: Vec::<(&'a Quad<T>, usize)>,
    cur: Option<&'a Quad<T>>,
    depth: usize,
}


impl<T> QuadTree<T> {

    pub fn new() -> Self {
        QuadTree{node: Node::Empty}
    }

    pub fn newGrid(xWidth: usize, yWidth: usize, init: fn(usize, usize) -> T) -> Self {
        if xWidth == 0 || yWidth == 0 {
            return Self::new();
        }
        // Required number of depth to fit max width and height equal to floor( log2( max(width,height) ) )
        let depth = usize::BITS - (max(xWidth, yWidth)-1).leading_zeros();
        return QuadTree{ node: Node::new_from_depth_ranged(depth as usize, (xWidth, yWidth), init, (0, 0)) };
    }

    pub fn iter<'a>(&'a self) -> QuadTreeTerminalIter<'a, T> {
        return QuadTreeTerminalIter::new(self);
    }

    pub fn iter_depth<'a>(&'a self, depth: usize) -> QuadTreeDepthIter<'a, T> {
        return QuadTreeDepthIter::new(self, depth);
    }


}

// // Function called on quads guaranteed to be consisting of terminals
// fn collapser<T>(node: Quad<T>) -> Node<T> {
//     if (_) {
//         return Node::Terminal(todo!());
//     }
//     return Node::Quad(node);
// }

impl<'a, T> QuadTreeTerminalIter<'a, T> {
    fn new(qt: &'a QuadTree<T>) -> Self {
        QuadTreeTerminalIter {
            qt_stack: Vec::new(),
            cur: Some(&qt.node),
        }

    }
}

impl<T: Copy> Iterator for QuadTreeTerminalIter<'_, T> {
    // We can refer to this type using Self::Item
    type Item = T;


    fn next(&mut self) -> Option<Self::Item> {

        match self.cur {
            None => {
                return None;
            }
            Some(Node::Quad(quad)) => { 
                // If its a quad assume we haven't explored it yet, descent untill terminal
                let new_cur = &quad.nodes[0];
                self.qt_stack.push((quad, 0));
                self.cur = Some(new_cur);
                self.next()
            }
            Some(Node::Terminal(ret)) => {
                let mut poped_idx;
                let mut poped_node: &Quad<T>;
                // idx of 4 mean there is no child left to iterate through on this node, keep going up the stack
                loop { // Do while poped_idx == 4
                    (poped_node, poped_idx) = match self.qt_stack.pop() {
                        Some(poped) => poped,
                        None => {
                            self.cur = None; // if reached end of stack, then iteration finished after current
                            return Some(*ret); 
                        },
                    };
                    if poped_idx != 4 {break;}
                }
                self.qt_stack.push((poped_node, poped_idx+1));
                Some(*ret)
            }
            Some(Node::Empty) => {
                let mut poped_idx = 3;
                let mut poped_node: &Quad<T>;
                // idx of 4 mean there is no child left to iterate through on this node, keep going up the stack
                loop { // Do while poped_idx == 4
                    (poped_node, poped_idx) = match self.qt_stack.pop() {
                        Some(poped) => poped,
                        None => {
                            self.cur = None; // if reached end of stack, then iteration finished after current
                            return self.next(); // Skip empty by recalling next
                        },
                    };
                    if poped_idx != 4 {break;}
                }
                self.qt_stack.push((poped_node, poped_idx+1));
                self.next() // Skep empty by recalling next
            }
        }
    }
}

impl<'a, T> QuadTreeDepthIter<'a, T> {
    fn new(qt: &'a QuadTree<T>, depth: usize) -> Self {
        if let Node::Quad(quad) = &qt.node {
            QuadTreeDepthIter {
                qt_stack: Vec::new(),
                cur: Some(quad),
                depth: depth,
            }
        } else {
            QuadTreeDepthIter {
                qt_stack: Vec::new(),
                cur: None,
                depth: depth,
            }
        }   
    }

    // Ascend to the nearest quad with unexplored quad child.
    // self.cur is set to the found child, if none found, set to None
    fn ascend_nearest(&mut self) {
        let mut poped_idx;
        let mut poped_node: &Quad<T>;
        // Ascend finding next quad child to explore
        // If poped_idx == 4, then there is no child left to explore on this node, keep going up the stack
        // If poped_idx != 4, then check if the next child is a quad, 
        // if so set it as cur and return, else keep going up the stack
        loop {

            // Pop last node and idx from stack,
            // if stack is empty the we ascended to the top of the tree
            // And no quads are left to explore, set cur to None and return
            (poped_node, poped_idx) = match self.qt_stack.pop() {
                Some(poped) => poped,
                None => {
                    self.cur = None; // if reached end of stack, then iteration finished after current
                    return; 
                },
            };
            // If not all children explored on this quad, check rest of children for quad
            if poped_idx != 4 {
                // Find if the next child is a quad, if so set it as cur and return, else keep going up the stack
                let mut idx: usize = poped_idx;
                for child in poped_node.nodes[idx..].iter() {
                    if let Node::Quad(new_quad) = child {
                        // If quad child is found repush poped quad to stack with updated idx, update cur, and return
                        self.qt_stack.push((poped_node, idx+1));
                        self.cur = Some(new_quad);
                        return;
                    }
                    idx += 1;
                }
            }
        }
    }
}

impl<'a, T: Copy> Iterator for QuadTreeDepthIter<'a, T> {
    // We can refer to this type using Self::Item
    type Item = &'a Quad<T>;

    fn next(&mut self) -> Option<Self::Item> {

        match self.cur {
            None => {
                return None;
            }
            Some(quad) => { 
                // If at depth return the quad (depth == 0)
                // else if depth != 0, descend step, and recursively call next
                if self.depth != 0 {
                    
                    // Find the first child in the current quad that is itself a quad.
                    let mut idx: usize = 0;
                    for child in quad.nodes[idx..].iter() {
                        if let Node::Quad(new_quad) = child {
                            // If quad child is found push last quad to stack with explored idx,
                            // update cur to the new found quad, and recall next to keep descending until depth is reached
                            self.qt_stack.push((quad, idx+1));
                            self.cur = Some(&new_quad);
                            return self.next();
                        }
                        idx += 1;
                    }

                    // If no quad child is found, then this node is terminal
                    // ascend back to unexplored quad and recall next()
                    self.ascend_nearest();
                    return self.next();
                } else {
                    // If at depth return the current quad, and reascend for next call
                    let ret_quad = quad;
                    self.ascend_nearest();
                    return Some(&ret_quad);
                }
            }
        }
    }
}