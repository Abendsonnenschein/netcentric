use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

fn main() {
    let a = Vertex::new("a");
    let b = Vertex::new("b");
    let c = Vertex::new("c");
    let d = Vertex::new("d");
    let e = Vertex::new("e");
    let f = Vertex::new("f");
    let g = Vertex::new("g");

    let mut adjacency_list = HashMap::new();

    adjacency_list.insert(a, vec![(b, 4), (c, 5), (d, 3)]);
    adjacency_list.insert(b, vec![(a, 4), (c, 2), (f, 1), (g, 3)]);
    adjacency_list.insert(c, vec![(a, 5), (b, 2), (d, 6), (e, 4), (f, 4)]);
    adjacency_list.insert(d, vec![(a, 3), (c, 6), (e, 3)]);
    adjacency_list.insert(e, vec![(c, 4), (d, 3), (f, 2)]);
    adjacency_list.insert(f, vec![(b, 1), (c, 4), (e, 2), (g, 5)]);
    adjacency_list.insert(g, vec![(b, 3), (f, 5)]);

    let distances = dijkstra(f, &adjacency_list);

    for (vertex, distance) in distances.iter() {
        println!("{}: {}", vertex.name, distance);
    }
}

fn dijkstra<'a>(
    start: Vertex<'a>,
    adjacency_list: &HashMap<Vertex<'a>, Vec<(Vertex<'a>, usize)>>,
) -> HashMap<Vertex<'a>, usize> {
    let mut distances = HashMap::new();
    let mut visited = HashSet::new();
    let mut to_visit = BinaryHeap::new();

    distances.insert(start, 0);

    to_visit.push(Visit {
        vertex: start,
        distance: 0,
    });

    while let Some(Visit { vertex, distance }) = to_visit.pop() {
        if !visited.insert(vertex) {
            continue;
        }

        if let Some(neighbors) = adjacency_list.get(&vertex) {
            for (neighbor, cost) in neighbors {
                let new_distance = distance + cost;
                let is_shorter = distances
                    .get(&neighbor)
                    .map_or(true, |&current| new_distance < current);

                if is_shorter {
                    distances.insert(*neighbor, new_distance);
                    to_visit.push(Visit {
                        vertex: *neighbor,
                        distance: new_distance,
                    });
                }
            }
        }
    }

    distances
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Vertex<'a> {
    name: &'a str,
}

impl<'a> Vertex<'a> {
    fn new(name: &'a str) -> Vertex<'a> {
        Vertex { name }
    }
}

#[derive(Debug)]
struct Visit<V> {
    vertex: V,
    distance: usize,
}

impl<V> Ord for Visit<V> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl<V> PartialOrd for Visit<V> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<V> PartialEq for Visit<V> {
    fn eq(&self, other: &Self) -> bool {
        self.distance.eq(&other.distance)
    }
}

impl<V> Eq for Visit<V> {}
