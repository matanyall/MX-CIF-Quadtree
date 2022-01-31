#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}


pub mod MX_CIF_Quadtree {

    /// Number of directions in 1d space
    const NDIR_1D: usize = 2;
    /// Number of directions in 2d space
    const NDIR_2D: usize = 4;

    /// Axis enum for 2d space
    enum axis {
        X,
        Y
    }

    /// Quadrants in our Quadtree, uses this ordering in traversal
    enum quadrant {
        NW,
        NE,
        SW,
        SE
    }

    /// Direction enum for traversing Binary tree of rectangles
    enum direction {
        LEFT,
        RIGHT,
        BOTH
    }

    /// Rectangle struct (represented with center, length & width)
    struct Rectangle {
        /// name of the rectangle
        name: String,
        /// center of the rectangle (Centroid)
        center: [f64; 2],
        /// Distance to borders or rectangle
        length: [f64; 2],
        /// Left and right sons
        bin_son: [Option<Box<Rectangle>>; NDIR_1D],
        /// Used in component labeling
        label: usize
    }

    /// Binary Node struct
    struct BNode {
        /// Left and right sons
        bin_son: [Option<Box<BNode>>; NDIR_2D],
        /// Pointer to the rectangle whose area contains the axis subdivision point
        rect: Option<Box<Rectangle>>,
        is_black: bool
    }

    /// Cif Node struct
    struct CNode {
        /// Four principal quad directions
        spc_son: [Option<Box<CNode>>; NDIR_2D],
        /// Pointers to rectangle sets for each of the axis
        bin_son: [Option<Box<BNode>>; NDIR_1D],
    }

    /// Quadtree struct
    struct MxCifQuadtree {
        root: Option<Box<CNode>>,
        /// World extent
        world: Rectangle,
        id: usize
    }

    impl MxCifQuadtree {
        pub fn new(world: Rectangle) -> MxCifQuadtree {
            MxCifQuadtree {
                root: None,
                world: world,
                id: 0
            }
        }

    }


}

