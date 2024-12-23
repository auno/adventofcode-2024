use std::collections::{HashMap, VecDeque};

use anyhow::{Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use hashbrown::HashSet;
use itertools::Itertools;

type Input = HashMap<String, HashSet<String>>;

#[aoc_generator(day23)]
fn parse(input: &str) -> Result<Input> {
    input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once("-").with_context(|| format!("Unable to parse line: {line}"))?;
            Ok([
                (a.to_string(), b.to_string()),
                (b.to_string(), a.to_string()),
            ])
        })
        .flatten_ok()
        .process_results(|connections| connections.into_grouping_map().collect::<HashSet<_>>())

}

#[aoc(day23, part1)]
fn part1(connections: &Input) -> usize {
    let computers_sorted = connections.iter().sorted_by_key(|(computer, _)| computer.as_str()).collect_vec();
    let mut groups_of_three = vec![];

    for (a, ac) in computers_sorted {
        for b in ac {
            if b <= a { continue; }

            for c in ac.intersection(&connections[b]) {
                if c <= b { continue; }
                groups_of_three.push([a, b, c]);
            }
        }
    }

    groups_of_three
        .into_iter()
        .filter(|group| group.iter().any(|computer| computer.starts_with("t")))
        .count()
}

#[aoc(day23, part2)]
fn part2(connections: &Input) -> Option<String> {
    let computers = connections.keys().map(String::as_str).sorted().collect_vec();
    let computer_indices = computers.iter().enumerate().map(|(i, n)| (*n, i)).collect::<HashMap<_,_>>();
    let connections = computers
        .iter()
        .map(|&computer| {
            connections[computer]
                .iter()
                .map(String::as_str)
                .map(|connection| computer_indices[connection])
                .collect_vec()
        })
        .collect_vec();

    let mut best_clique = None;
    let mut best_clique_len = 0;
    let mut queue = (0..computers.len()).map(|i| (vec![i], connections[i].clone())).collect::<VecDeque<_>>();

    while let Some((clique, candidates)) = queue.pop_front() {
        if clique.len() + candidates.len() <= best_clique_len { continue; }
        if candidates.is_empty() { continue; }

        let candidate = candidates[0];

        if clique.iter().all(|computer| connections[*computer].contains(&candidate)) {
            let mut candidate_clique = clique.clone();
            candidate_clique.push(candidate);
            candidate_clique.sort();

            if candidate_clique.len() > best_clique_len {
                best_clique = Some(candidate_clique.clone());
                best_clique_len = candidate_clique.len();
            }

            if candidates.len() > 1 {
                queue.push_back((candidate_clique, candidates[1..].to_vec()));
            }
        }

        if candidates.len() > 1 {
            queue.push_back((clique, candidates[1..].to_vec()));
        }
    }

    best_clique.map(|clique| clique.into_iter().map(|i| computers[i]).join(","))
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const EXAMPLE1: &str = indoc! {"
        kh-tc
        qp-kh
        de-cg
        ka-co
        yn-aq
        qp-ub
        cg-tb
        vc-aq
        tb-ka
        wh-tc
        yn-cg
        kh-ub
        ta-co
        de-co
        tc-td
        tb-wq
        wh-td
        ta-ka
        td-qp
        aq-cg
        wq-ub
        ub-vc
        de-ta
        wq-aq
        wq-vc
        wh-yn
        ka-de
        kh-ta
        co-tc
        wh-qp
        tb-vc
        td-yn
    "};

    #[test]
    fn part1_example1() {
        assert_eq!(7, part1(&parse(EXAMPLE1).unwrap()));
    }

    #[test]
    fn part1_input() {
        assert_eq!(1314, part1(&parse(include_str!("../input/2024/day23.txt")).unwrap()));
    }

    #[test]
    fn part2_example1() {
        let ans = part2(&parse(EXAMPLE1).unwrap());
        assert_ne!(None, ans);
        assert_eq!("co,de,ka,ta", ans.unwrap());
    }

    #[test]
    fn part2_input() {
        let ans = part2(&parse(include_str!("../input/2024/day23.txt")).unwrap());
        assert_ne!(None, ans);
        assert_eq!("bg,bu,ce,ga,hw,jw,nf,nt,ox,tj,uu,vk,wp", ans.unwrap());
    }
}
