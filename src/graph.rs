pub unsafe trait IndexType:
    Copy + Default + std::hash::Hash + Ord + std::fmt::Debug + 'static
{
    fn new(x: usize) -> Self;
    fn index(&self) -> usize;
    fn max() -> Self;
}

unsafe impl IndexType for usize {
    #[inline(always)]
    fn new(x: usize) -> Self {
        x
    }
    #[inline(always)]
    fn index(&self) -> Self {
        *self
    }
    #[inline(always)]
    fn max() -> Self {
        ::std::usize::MAX
    }
}

unsafe impl IndexType for u32 {
    #[inline(always)]
    fn new(x: usize) -> Self {
        x as u32
    }
    #[inline(always)]
    fn index(&self) -> usize {
        *self as usize
    }
    #[inline(always)]
    fn max() -> Self {
        ::std::u32::MAX
    }
}

unsafe impl IndexType for i32 {
    #[inline(always)]
    fn new(x: usize) -> Self {
        x as i32
    }
    #[inline(always)]
    fn index(&self) -> usize {
        *self as usize
    }
    #[inline(always)]
    fn max() -> Self {
        ::std::i32::MAX
    }
}

#[derive(Copy, Clone, Debug)]
pub struct EdgeIndex<Ix>(Ix);
#[derive(Copy, Clone, Debug)]
pub struct NodeIndex<Ix>(Ix);

impl<Ix> EdgeIndex<Ix>
where
    Ix: IndexType,
{
    #[inline]
    pub fn new(e: Ix) -> Self {
        Self { 0: e }
    }

    #[inline]
    pub fn index(&self) -> usize {
        self.0.index()
    }
}

impl<Ix> NodeIndex<Ix>
where
    Ix: IndexType,
{
    pub fn new(n: Ix) -> Self {
        Self { 0: n }
    }

    #[inline]
    pub fn index(&self) -> usize {
        self.0.index()
    }
}

pub trait IntoWeightedEdge<E> {
    type NodeId;
    fn into_weighted_edge(self) -> (Self::NodeId, Self::NodeId, E);
}

impl<Ix, E> IntoWeightedEdge<E> for (Ix, Ix)
where
    E: Default,
{
    type NodeId = Ix;

    fn into_weighted_edge(self) -> (Self::NodeId, Self::NodeId, E) {
        let (s, t) = self;
        (s, t, E::default())
    }
}

pub struct Edge<Weight, Ix> {
    from: NodeIndex<Ix>,
    to: NodeIndex<Ix>,
    pub weight: Weight,
}

impl<Weight, Ix> Edge<Weight, Ix>
where
    Ix: IndexType,
{
    pub fn source(&self) -> NodeIndex<Ix> {
        self.from
    }

    pub fn target(&self) -> NodeIndex<Ix> {
        self.to
    }
}

pub struct Node<N> {
    pub data: N,
}

pub struct Directed {}

type DefaultIndex = u32;

pub struct Graph<N, Weight, Ty = Directed, Ix = DefaultIndex> {
    nodes: Vec<Node<N>>,
    edges: Vec<Edge<Weight, Ix>>,
    incidences: Vec<Vec<EdgeIndex<Ix>>>,
    _marker: std::marker::PhantomData<Ty>,
}

impl<N, Weight, Ty, Ix> Graph<N, Weight, Ty, Ix>
where
    Weight: Ord,
    Ix: IndexType,
{
    pub fn new(vertex_count: usize) -> Self {
        Graph {
            nodes: Vec::with_capacity(vertex_count),
            edges: Vec::new(),
            incidences: Vec::with_capacity(vertex_count),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn get_edge(&self, e: EdgeIndex<Ix>) -> Option<&Edge<Weight, Ix>> {
        self.edges.get(e.index() as usize)
    }

    pub fn extend_with_edges<I>(&mut self, iterable: I)
    where
        I: IntoIterator,
        Weight: Default,
        I::Item: IntoWeightedEdge<Weight>,
        <I::Item as IntoWeightedEdge<Weight>>::NodeId: Into<NodeIndex<Ix>>,
        N: Default,
    {
        let iter = iterable.into_iter();
        let (low, _) = iter.size_hint();
        self.edges.reserve(low);

        for e in iter {
            let (from, to, weight) = e.into_weighted_edge();
            let (from, to) = (from.into(), to.into());
            let m = std::cmp::max(from.index(), to.index());
            while m >= self.node_count() {
                self.nodes.push(Node { data: N::default() });
                self.incidences.push(Vec::new());
            }
            self.add_edge(from, to, weight);
        }
    }

    pub fn add_edge(&mut self, from: NodeIndex<Ix>, to: NodeIndex<Ix>, w: Weight) -> EdgeIndex<Ix> {
        let f = from.index();
        let t = to.index();
        let m = std::cmp::max(t, f);
        if m >= self.nodes.len() || m >= self.incidences.len() {
            panic!("Graph::add_edge: nodes id out of bound");
        }
        self.edges.push(Edge {
            from,
            to,
            weight: w,
        });
        self.incidences[t].push(EdgeIndex::new(IndexType::new(f)));
        self.incidences[f].push(EdgeIndex::new(IndexType::new(t)));

        EdgeIndex::new(IndexType::new(self.edges.len() - 1))
    }
}
