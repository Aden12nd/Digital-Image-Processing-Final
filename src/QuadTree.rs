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

pub struct QuadTreeIter<'a, T> {
    qt_stack: Vec::<(&'a Quad<T>, usize)>,
    cur: Option<&'a Node<T>>,
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

    pub fn iter<'a>(&'a self) -> QuadTreeIter<'a, T> {
        return QuadTreeIter::new(self);
    } 
}

impl<'a, T> QuadTreeIter<'a, T> {
    fn new(qt: &'a QuadTree<T>) -> Self {
        QuadTreeIter {
            qt_stack: Vec::new(),
            cur: Some(&qt.node),
        }

    }
}

impl<T: Copy> Iterator for QuadTreeIter<'_, T> {
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