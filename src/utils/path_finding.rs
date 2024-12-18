use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, VecDeque};
use std::hash::Hash;

use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

type Distances<SearchNode> = HashMap<SearchNode, (usize, Vec<SearchNode>)>;
pub type PathMap<SearchNode> = HashMap<SearchNode, Vec<SearchNode>>;

fn resolve_path_map<SearchNode>(distances: &Distances<SearchNode>, targets: &[SearchNode]) -> PathMap<SearchNode> where
    SearchNode: Copy + Clone + PartialEq + PartialOrd + Ord + Hash,
{
    let mut queue = VecDeque::from_iter(targets.iter().copied());
    let mut seen = HashSet::new();
    let mut path_map = HashMap::from_iter(targets.iter().map(|target| (*target, vec![])));

    while let Some(current) = queue.pop_front() {
        if !seen.insert(current) {
            continue;
        }

        for &previous in distances
            .get(&current)
            .map(|(_, previous)| previous)
            .unwrap_or(&vec![])
        {
            path_map.entry(previous).or_default().push(current);
            queue.push_back(previous);
        }
    }

    path_map
}

pub fn distance<SearchNode> (
    source: SearchNode,
    neighbors: impl Fn(SearchNode) -> Vec<(SearchNode, usize)>,
    is_target: impl Fn(SearchNode) -> bool,
) -> Option<(usize, PathMap<SearchNode>)> where
    SearchNode: Copy + Clone + PartialEq + PartialOrd + Ord + Hash,
{
    let mut distances = HashMap::from([(source, (0, vec![]))]);
    let mut queue = BinaryHeap::from([(Reverse(0), source)]);

    while let Some((Reverse(distance), current)) = queue.pop() {
        if is_target(current) {
            break;
        }

        for (neighbor, cost) in neighbors(current) {
            let (neighbor_distance, neighbor_source) = distances
                .entry(neighbor)
                .or_insert((usize::MAX, vec![]));

            match (distance + cost).cmp(neighbor_distance) {
                Ordering::Less => {
                    *neighbor_distance = distance + cost;
                    *neighbor_source = vec![current];
                    queue.push((Reverse(*neighbor_distance), neighbor));
                }
                Ordering::Equal => {
                    neighbor_source.push(current);
                }
                Ordering::Greater => {},
            }
        }
    }

    let potential_targets = distances
        .iter()
        .filter(|(node, _)| is_target(**node))
        .collect_vec();

    let min_distance = potential_targets
        .iter()
        .map(|(_, (distance, _))| *distance)
        .min()?;

    let targets = potential_targets
        .iter()
        .filter(|(_, (distance, _))| *distance == min_distance)
        .map(|(node, _)| **node)
        .collect_vec();

    Some((min_distance, resolve_path_map(&distances, &targets)))
}
