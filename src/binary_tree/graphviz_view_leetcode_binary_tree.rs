use super::{null, serialize_binary_tree_to_vec, TreeLink};

#[derive(Debug, Clone)]
pub(super) struct Edge {
    src_idx: usize,
    dst_idx: usize,
}

pub(super) struct GraphvizLeetcodeTree {
    tree: Vec<i32>,
    edges: Vec<Edge>,
    show_null_node: bool,
}

impl GraphvizLeetcodeTree {
    fn build_edges(tree: &[i32]) -> Vec<Edge> {
        #[allow(clippy::cast_precision_loss)]
        let tree_high = ((tree.len() + 1) as f64).log2() as u32;
        // e.g tree_height is 3, edges len is 2.pow(1)+2.pow(2)
        let mut edges = vec![];
        let mut src_idx = 0;
        // construct edge
        for height in 0..tree_high - 1 {
            for _ in 0..2_usize.pow(height) {
                edges.push(Edge {
                    src_idx,
                    dst_idx: src_idx * 2 + 1,
                });
                edges.push(Edge {
                    src_idx,
                    dst_idx: src_idx * 2 + 2,
                });
                src_idx += 1;
            }
        }
        edges
    }

    pub(super) fn new(root: TreeLink, show_null_node: bool) -> Self {
        let tree = serialize_binary_tree_to_vec(root);
        let edges = Self::build_edges(&tree);
        Self {
            tree,
            edges,
            show_null_node,
        }
    }

    pub(super) fn from_leetcode_tree_vec_format(tree: Vec<i32>) -> Self {
        let edges = Self::build_edges(&tree);
        Self {
            tree,
            edges,
            show_null_node: false,
        }
    }
}

impl<'a> rustc_graphviz::GraphWalk<'a> for GraphvizLeetcodeTree {
    type Node = usize;
    type Edge = &'a Edge;

    fn nodes(&'a self) -> rustc_graphviz::Nodes<'a, Self::Node> {
        // self.flatten_binary_tree.iter().copied().collect()
        if self.show_null_node {
            (0..self.tree.len()).collect()
        } else {
            (0..self.tree.len())
                .filter(|x| self.tree[*x] != null)
                .collect()
        }
    }

    fn edges(&'a self) -> rustc_graphviz::Edges<'a, Self::Edge> {
        if self.show_null_node {
            self.edges.iter().collect()
        } else {
            self.edges
                .iter()
                .filter(|x| self.tree[x.dst_idx] != null)
                .collect()
        }
    }

    fn source(&'a self, edge: &Self::Edge) -> Self::Node {
        edge.src_idx
    }

    fn target(&'a self, edge: &Self::Edge) -> Self::Node {
        edge.dst_idx
    }
}

impl<'a> rustc_graphviz::Labeller<'a> for GraphvizLeetcodeTree {
    type Node = usize;
    type Edge = &'a Edge;

    fn graph_id(&'a self) -> rustc_graphviz::Id<'a> {
        rustc_graphviz::Id::new("rustc_graphviz").unwrap()
    }

    fn node_id(&'a self, n: &Self::Node) -> rustc_graphviz::Id<'a> {
        rustc_graphviz::Id::new(format!("_{}", n)).unwrap()
    }

    fn node_label(&'a self, e: &Self::Node) -> rustc_graphviz::LabelText<'a> {
        let val = self.tree[*e];
        if val == null {
            rustc_graphviz::LabelText::LabelStr("N".into())
        } else {
            rustc_graphviz::LabelText::LabelStr(val.to_string().into())
        }
    }

    // fn node_shape(&'a self, _node: &Self::Node) -> Option<rustc_graphviz::LabelText<'a>> {
    //     Some(rustc_graphviz::LabelText::LabelStr("circle".into()))
    // }
}

#[test]
fn test_leetcode_tree_grpahviz() {
    const DOT_OUTPUT_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/target/graphviz.gv");
    use super::{deserialize_vec_to_binary_tree, print_binary_tree};
    let level_order_1 = vec![3, 9, 20, null, null, 15, 7];
    let node = deserialize_vec_to_binary_tree(&level_order_1);
    print_binary_tree(node.clone()).unwrap();
    let graph = GraphvizLeetcodeTree::new(node, false);
    let mut f = std::fs::File::create(DOT_OUTPUT_PATH).unwrap();
    rustc_graphviz::render(&graph, &mut f).unwrap();
    let is_success = std::process::Command::new("xdot")
        .arg(DOT_OUTPUT_PATH)
        .spawn()
        .unwrap()
        .wait()
        .unwrap()
        .success();
    assert!(is_success);
}
