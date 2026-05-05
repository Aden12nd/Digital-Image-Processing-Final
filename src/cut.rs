

enum Cut {
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
    LeftAngleLeft,
    LeftAngleRight,
    RightAngleLeft,
    RightAngleRight,
    
}