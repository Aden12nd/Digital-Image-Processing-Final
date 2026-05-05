use strum::IntoEnumIterator;
use strum_macros::EnumIter; 


#[derive(Debug, EnumIter, Clone)]
pub enum Cut {
    Vertical,
    Horizontal,
    
    /** Operate on each cornor
     *  + - - - - +
     *  | X     X | Showing each corner is different cut
     *  |         |
     *  |         |
     *  | X     X |
     *  + - - - - +
     */
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,

    /** Diagnals
     *  + - - - - + BackDiagnal
     *  | X       |
     *  | X X     |
     *  | X X X   |
     *  | X X X X |
     *  + - - - - +
     */
    BackDiagnal,
    ForwardDiagnal,
    
    /** Operate on angled edges
     *  + - - - - + BottomAngleRight
     *  |         |
     *  |         |
     *  |     X X |
     *  | X X X X | 
     *  + - - - - +
     */
    BottomAngleLeft,
    BottomAngleRight,
    TopAngleLeft,
    TopAngleRight,
    LeftAngleTop,
    LeftAngleBottom,
    RightAngleTop,
    RightAngleBottom,
    
}

fn idx(row: usize, col: usize, size: usize) -> usize {
    col * size + row
}

fn belongs_to_first(row: usize, col: usize, size: usize, cut: &Cut) -> bool {
    let mid = size / 2;

    match cut {
        // Straight cuts
        Cut::Vertical => col < mid,
        Cut::Horizontal => row < mid,

        // Corners (quadrants split diagonally inside that quadrant)
        Cut::TopRight => row + mid <= col,
        Cut::TopLeft => row + col < mid,
        Cut::BottomRight => row + col + 1 >= size + mid,
        Cut::BottomLeft => row >= col + mid,

        // Full diagonals
        Cut::BackDiagnal => row >= col,                  // \
        Cut::ForwardDiagnal => row + col < size,         // /

        // Angled halves (one side full, other diagonal)
        Cut::BottomAngleLeft => row >= col/2 + mid,
        Cut::BottomAngleRight => row + col/2 + 1 >= size,
        Cut::TopAngleLeft => row + col/2 < mid,
        Cut::TopAngleRight => row <= col/2,

        Cut::LeftAngleTop => row/2 + col < mid,
        Cut::LeftAngleBottom => row/2 >= col,
        Cut::RightAngleTop => row/2 + mid <= col,
        Cut::RightAngleBottom => row/2 + col + 1 >= size,
    }
}

pub fn cut_segment<T: Copy>(pixel_data: Vec<T>, size: usize, cut: Cut) -> (Vec<T>, Vec<T>) {
    assert_eq!(pixel_data.len(), size * size);

    let mut a = Vec::with_capacity(pixel_data.len());
    let mut b = Vec::with_capacity(pixel_data.len());

    for col in 0..size {
        for row in 0..size {
            let i = idx(row, col, size);
            if belongs_to_first(row, col, size, &cut) {
                a.push(pixel_data[i]);
            } else {
                b.push(pixel_data[i]);
            }
        }
    }

    (a, b)
}