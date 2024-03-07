use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NodeLabel(usize);

#[derive(Debug, Clone, PartialEq)]
pub struct Neighbors<T: Clone,const MAX_NEIGHBORS: usize>([Option<T>; MAX_NEIGHBORS], usize);

#[derive(Debug, PartialEq)]
pub enum AddEdgeError {
    Overflow,
    Exists
}

pub trait KeyTraits: Clone + Ord + Copy + std::hash::Hash {}
impl<T: Clone + Ord + Copy + std::hash::Hash + ?Sized> KeyTraits for T {}

pub fn option_less_than<T:KeyTraits>(p: &Option<T>,q: &Option<T>) -> std::cmp::Ordering{
    match (p,q) {
        (Some(_), None) => std::cmp::Ordering::Less,
        (None, Some(_)) => std::cmp::Ordering::Greater,
        (None, None) => std::cmp::Ordering::Equal,
        (Some(x), Some(y)) => x.cmp(y)
    }
} 

impl<T:KeyTraits,const MAX_NEIGHBORS: usize> Neighbors<T,MAX_NEIGHBORS> {
    pub fn append(&mut self, node_id: T) -> Result<&mut Self, AddEdgeError> {
        if self.1 < MAX_NEIGHBORS {
            match self.0.binary_search_by(|x| option_less_than(x,&Some(node_id))) {
                Ok(_) => Err(AddEdgeError::Exists),
                Err(pos) => {
                    if pos != self.1 {
                        self.0.copy_within(pos..self.1, pos+1);
                    } 
                    self.0[pos] = Some(node_id);
                    self.1 += 1;

                    Ok(self)
                }
            }
        } else {
            Err(AddEdgeError::Overflow)
        }
    }
}

impl<T:KeyTraits,const N: usize, const MAX_NEIGHBORS: usize> From<&[T; N]> for Neighbors<T,MAX_NEIGHBORS> {
    fn from(neighbors: &[T; N]) -> Neighbors<T,MAX_NEIGHBORS> {
        let mut ns = Neighbors([None; MAX_NEIGHBORS], N);

        ns.0[0..N].copy_from_slice( &neighbors.map(|x|Some(x)) );

        ns
    }
}

impl<T:KeyTraits,const MAX_NEIGHBORS: usize> From<&[Option<T>; MAX_NEIGHBORS]> for Neighbors<T,MAX_NEIGHBORS> {
    fn from(neighbors: &[Option<T>; MAX_NEIGHBORS]) -> Neighbors<T,MAX_NEIGHBORS> {
        let first_null = neighbors.iter().position(|x| x.is_none()).unwrap_or(MAX_NEIGHBORS);

        Neighbors::<T,MAX_NEIGHBORS>(*neighbors, first_null)
    }
}

pub type GraphNodes<T,const MAX_NEIGHBORS: usize> = HashMap<T, Neighbors<T,MAX_NEIGHBORS>>;

#[derive(Clone, Debug)]
pub struct Graph<T:KeyTraits,const MAX_NEIGHBORS: usize>(GraphNodes<T,MAX_NEIGHBORS>);

impl<T:KeyTraits,const MAX_NEIGHBORS: usize> std::ops::Deref for Graph<T,MAX_NEIGHBORS> {
    type Target = GraphNodes<T,MAX_NEIGHBORS>;

    fn deref(&self) -> &GraphNodes<T,MAX_NEIGHBORS> {
        &self.0
    }
}

impl<T:KeyTraits,const MAX_NEIGHBORS: usize> std::ops::DerefMut for Graph<T,MAX_NEIGHBORS> {
    fn deref_mut(&mut self) -> &mut GraphNodes<T,MAX_NEIGHBORS> {
        &mut self.0
    }
}

#[derive(Debug, PartialEq)]
pub enum InsertEdgeError {
    FromMissing,
    AppendEdgeError(AddEdgeError)
}

impl<T:KeyTraits,const MAX_NEIGHBORS: usize> Graph<T,MAX_NEIGHBORS> {
    pub fn new() -> Self {
        Graph(GraphNodes::new())
    }

    pub fn insert<const N: usize>(&mut self, node_id: T, neighbors: &[T; N]) {
        self.0.insert(node_id, Neighbors::from(neighbors));
    }

    pub fn insert_edge(&mut self, from: T, to: T) -> Result<&mut Self, InsertEdgeError> {
        if let Some(from_node) = self.0.get_mut(&from) {
            match from_node.append(to) {
                Ok(_) => Ok(self),
                Err(e) => Err(InsertEdgeError::AppendEdgeError(e))
            }
        } else {
            Err(InsertEdgeError::FromMissing)
        }
    }
}
