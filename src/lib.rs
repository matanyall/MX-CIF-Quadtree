#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}


mod MX_CIF_Quadtree {

    const NDIR_1D: usize = 2;
    const NDIR_2D: usize = 4;

    enum axis {
        X,
        Y
    }

    enum quadrant {
        NW,
        NE,
        SW,
        SE
    }

    enum direction {
        LEFT,
        RIGHT,
        BOTH
    }

    struct Rectangle {
        name: String,
        center: [f64; 2],
        length: [f64; 2],
        bin_son: [Option<Box<Rectangle>>; NDIR_1D],
        label: usize
    }

    struct BNode {
        bin_son: [Option<Box<BNode>>; NDIR_2D],
        rect: Option<Box<Rectangle>>,
        is_black: bool
    }

    struct CNode {
        spc_son: [Option<Box<CNode>>; NDIR_2D],
        bin_son: [Option<Box<BNode>>; NDIR_1D],
    }

    struct MxCifQuadtree {
        root: Option<Box<CNode>>,
        world: Rectangle,
        id: usize
    }


}

