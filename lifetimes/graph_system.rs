use std::marker::PhantomData;
use std::collections::{HashMap, HashSet};

pub struct Graph<'a, T> {
    nodes: Vec<T>,
    edges: Vec<(usize, usize)>,
    _marker: PhantomData<&'a T>,
}

pub struct NodeHandle<'a, T> {
    graph: &'a Graph<'a, T>,
    index: usize,
}

pub struct EdgeHandle<'a, T> {
    graph: &'a Graph<'a, T>,
    from: usize,
    to: usize,
}

impl<'a, T> Graph<'a, T> {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            _marker: PhantomData,
        }
    }

    pub fn add_node(&mut self, data: T) -> usize {
        self.nodes.push(data);
        self.nodes.len() - 1
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        if from < self.nodes.len() && to < self.nodes.len() {
            self.edges.push((from, to));
        }
    }

    pub fn get_node(&'a self, index: usize) -> Option<NodeHandle<'a, T>> {
        if index < self.nodes.len() {
            Some(NodeHandle { graph: self, index })
        } else {
            None
        }
    }

    pub fn iter_nodes(&'a self) -> NodeIter<'a, T> {
        NodeIter {
            graph: self,
            current: 0,
        }
    }
}

pub struct NodeIter<'a, T> {
    graph: &'a Graph<'a, T>,
    current: usize,
}

impl<'a, T> Iterator for NodeIter<'a, T> {
    type Item = NodeHandle<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.graph.nodes.len() {
            let handle = NodeHandle {
                graph: self.graph,
                index: self.current,
            };
            self.current += 1;
            Some(handle)
        } else {
            None
        }
    }
}

impl<'a, T> NodeHandle<'a, T> {
    pub fn data(&self) -> &'a T {
        &self.graph.nodes[self.index]
    }

    pub fn neighbors(&self) -> NeighborIter<'a, T> {
        NeighborIter {
            graph: self.graph,
            node_index: self.index,
            edge_index: 0,
        }
    }
}

pub struct NeighborIter<'a, T> {
    graph: &'a Graph<'a, T>,
    node_index: usize,
    edge_index: usize,
}

impl<'a, T> Iterator for NeighborIter<'a, T> {
    type Item = NodeHandle<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.edge_index < self.graph.edges.len() {
            let (from, to) = self.graph.edges[self.edge_index];
            self.edge_index += 1;
            if from == self.node_index {
                return Some(NodeHandle {
                    graph: self.graph,
                    index: to,
                });
            }
        }
        None
    }
}

pub struct GraphView<'a, T> {
    graph: &'a Graph<'a, T>,
    visible_nodes: HashSet<usize>,
}

impl<'a, T> GraphView<'a, T> {
    pub fn new(graph: &'a Graph<'a, T>, seed: usize) -> Self {
        let mut visible = HashSet::new();
        visible.insert(seed);
        Self { graph, visible_nodes: visible }
    }

    pub fn expand(&mut self) {
        let mut new_nodes = Vec::new();
        for &node_idx in &self.visible_nodes {
            for (from, to) in &self.graph.edges {
                if *from == node_idx {
                    new_nodes.push(*to);
                }
            }
        }
        for n in new_nodes {
            self.visible_nodes.insert(n);
        }
    }

    pub fn contains(&self, handle: &NodeHandle<'a, T>) -> bool {
        self.visible_nodes.contains(&handle.index)
    }
}

pub struct Path<'a, T> {
    nodes: Vec<NodeHandle<'a, T>>,
}

impl<'a, T> Path<'a, T> {
    pub fn len(&self) -> usize {
        self.nodes.len()
    }
}

pub struct PathFinder<'a, T> {
    graph: &'a Graph<'a, T>,
}

impl<'a, T> PathFinder<'a, T> {
    pub fn find_path(&self, start: usize, end: usize) -> Option<Path<'a, T>> {
        let mut queue = std::collections::VecDeque::new();
        let mut visited = HashSet::new();
        let mut parent = HashMap::new();

        queue.push_back(start);
        visited.insert(start);

        while let Some(current) = queue.pop_front() {
            if current == end {
                let mut path = Vec::new();
                let mut curr =end;
                while let Some(&p) = parent.get(&curr) {
                    path.push(NodeHandle { graph: self.graph, index: curr });
                    curr = p;
                }
                path.push(NodeHandle { graph: self.graph, index: start });
                path.reverse();
                return Some(Path { nodes: path });
            }

            for (from, to) in &self.graph.edges {
                if *from == current && !visited.contains(to) {
                    visited.insert(*to);
                    parent.insert(*to, current);
                    queue.push_back(*to);
                }
            }
        }
        None
    }
}

pub trait GraphVisitor<'a, T> {
    fn visit(&mut self, node: NodeHandle<'a, T>);
}

pub struct LoggingVisitor;

impl<'a, T: std::fmt::Debug> GraphVisitor<'a, T> for LoggingVisitor {
    fn visit(&mut self, node: NodeHandle<'a, T>) {
        println!("Visiting {:?}", node.data());
    }
}

pub struct DfsTraversal<'a, T, V> {
    graph: &'a Graph<'a, T>,
    visitor: V,
    visited: HashSet<usize>,
}

impl<'a, T, V: GraphVisitor<'a, T>> DfsTraversal<'a, T, V> {
    pub fn run(&mut self, start: usize) {
        self.visit_recursive(start);
    }

    fn visit_recursive(&mut self, node_idx: usize) {
        if self.visited.contains(&node_idx) {
            return;
        }
        self.visited.insert(node_idx);
        if let Some(handle) = self.graph.get_node(node_idx) {
            self.visitor.visit(handle); 
        }
        
        let neighbors: Vec<usize> = self.graph.edges.iter()
            .filter(|(from, _)| *from == node_idx)
            .map(|(_, to)| *to)
            .collect();

        for n in neighbors {
            self.visit_recursive(n);
        }
    }
}

pub struct SubGraph<'a, 'b, T> 
where 'a: 'b 
{
    parent: &'b Graph<'a, T>,
    subset: &'b [usize],
}

impl<'a, 'b, T> SubGraph<'a, 'b, T> {
    pub fn nodes(&self) -> impl Iterator<Item = &'a T> + 'b {
        self.subset.iter().map(move |&idx| &self.parent.nodes[idx])
    }
}

pub struct GraphMerger<'a, T> {
    g1: &'a Graph<'a, T>,
    g2: &'a Graph<'a, T>,
}

impl<'a, T: Clone> GraphMerger<'a, T> {
    pub fn merge(&self) -> Graph<'static, T> {
        let mut new_graph = Graph::<<T>>::new();
        new_graph
    }
}

pub struct LifetimeWrapper2<'a, T>(pub &'a T);

pub fn find_common_node<'a, 'b, T: PartialEq>(
    g1: &'a Graph<'a, T>, 
    g2: &'b Graph<'b, T>
) -> Option<&'a T> {
    for n1 in &g1.nodes {
        for n2 in &g2.nodes {
            if n1 == n2 {
                return Some(n1);
            }
        }
    }
    None
}

pub struct EdgeIterator<'a, T> {
    graph: &'a Graph<'a, T>,
    pos: usize,
}

impl<'a, T> Iterator for EdgeIterator<'a, T> {
    type Item = (&'a T, &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < self.graph.edges.len() {
            let (u, v) = self.graph.edges[self.pos];
            self.pos += 1;
            Some((&self.graph.nodes[u], &self.graph.nodes[v]))
        } else {
            None
        }
    }
}

pub struct BipartiteCheck<'a, T> {
    graph: &'a Graph<'a, T>,
    colors: HashMap<usize, bool>,
}

impl<'a, T> BipartiteCheck<'a, T> {
    pub fn new(graph: &'a Graph<'a, T>) -> Self {
        Self { graph, colors: HashMap::new() }
    }
    
    pub fn check(&mut self) -> bool {
        for i in 0..self.graph.nodes.len() {
            if !self.colors.contains_key(&i) {
                if !self.dfs(i, true) {
                    return false;
                }
            }
        }
        true
    }
    
    fn dfs(&mut self, u: usize, c: bool) -> bool {
        self.colors.insert(u, c);
        for (from, to) in &self.graph.edges {
            if *from == u {
                let v = *to;
                if let Some(& existing_color) = self.colors.get(&v) {
                    if existing_color == c {
                        return false;
                    }
                } else if !self.dfs(v, !c) {
                    return false;
                }
            } else if *to == u {
                let v = *from; 
                 if let Some(& existing_color) = self.colors.get(&v) {
                    if existing_color == c {
                        return false;
                    }
                } else if !self.dfs(v, !c) {
                    return false;
                }
            }
        }
        true
    }
}

pub struct LazyGraphLoader<'a, L> 
where L: Fn(usize) -> Option<String> + 'a
{
    loader: L,
    cache: HashMap<usize, String>,
    _marker: PhantomData<&'a ()>,
}

impl<'a, L> LazyGraphLoader<'a, L> 
where L: Fn(usize) -> Option<String> + 'a
{
    pub fn get(&'a mut self, id: usize) -> Option<&'a String> {
        if !self.cache.contains_key(&id) {
            if let Some(val) = (self.loader)(id) {
                self.cache.insert(id, val);
            }
        }
        self.cache.get(&id)
    }
}

pub struct RecursiveNode<'a> {
    pub parent: Option<&'a RecursiveNode<'a>>,
    pub children: Vec<&'a RecursiveNode<'a>>,
    pub value: i32,
}

pub fn traverse_up<'a>(mut node: &'a RecursiveNode<'a>) -> i32 {
    let mut sum = 0;
    loop {
        sum += node.value;
        match node.parent {
            Some(p) => node = p,
            None => break,
        }
    }
    sum
}

pub trait NodeVisitor<'a> {
    fn visit(&mut self, n: &'a usize);
}

pub struct NoOpVisitor;
impl<'a> NodeVisitor<'a> for NoOpVisitor {
    fn visit(&mut self, _: &'a usize) {}
}

pub struct MultiRefContainer<'a, 'b, 'c, T> {
    r1: &'a T,
    r2: &'b T,
    r3: &'c T,
}

impl<'a, 'b, 'c, T> MultiRefContainer<'a, 'b, 'c, T> {
    pub fn rotate(self) -> MultiRefContainer<'c, 'a, 'b, T> 
    where 'a: 'c, 'b: 'a, 'c: 'b 
    {
        MultiRefContainer {
            r1: self.r3,
            r2: self.r1,
            r3: self.r2,
        }
    }
}

fn main() {
    let g = Graph::<i32>::new();
    let bc = BipartiteCheck::new(&g);
}
