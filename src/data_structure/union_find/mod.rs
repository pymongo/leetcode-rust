mod friend_circles;
mod min_cost_to_connect_all_points;

struct UnionFind {
    parents: Vec<usize>,
}

impl UnionFind {
    #[inline]
    fn new(n: usize) -> Self {
        Self {
            parents: (0..n).collect(),
        }
    }

    fn find_root(&self, node: usize) -> usize {
        let mut curr_node = node;
        let mut curr_node_parent = self.parents[curr_node];
        while curr_node_parent != curr_node {
            curr_node = curr_node_parent;
            curr_node_parent = self.parents[curr_node];
        }
        curr_node_parent
    }

    fn union(&mut self, node_a: usize, node_b: usize) {
        // 路径压缩: 不要直接将b连到a上，而是将b的祖先连向a的祖先，以此压缩路径减少连边
        let root_a = self.find_root(node_a);
        let root_b = self.find_root(node_b);
        if root_a != root_b {
            // 将b的祖先挂载到a的祖先下
            self.parents[root_b] = root_a;
        }
    }
}

struct UnionFindConstGenerics<const N: usize> {
    parents: [usize; N],
}

impl<const N: usize> UnionFindConstGenerics<N> {
    const fn new() -> Self {
        let mut parents = [0; N];
        let mut i = 0;
        while i < N {
            parents[i] = i;
            i += 1;
        }
        Self { parents }
    }

    const fn find_root(&self, node: usize) -> usize {
        let mut cur = node;
        let mut cur_parent = self.parents[cur];
        while cur_parent != cur {
            cur = cur_parent;
            cur_parent = self.parents[cur];
        }
        cur_parent
    }

    fn union(&mut self, node_a: usize, node_b: usize) {
        let root_a = self.find_root(node_a);
        let root_b = self.find_root(node_b);
        if root_a != root_b {
            // 将b的祖先挂载到a的祖先下
            self.parents[root_b] = root_a;
        }
    }
}
