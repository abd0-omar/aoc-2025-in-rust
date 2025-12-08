use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "08";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn parse<R: BufRead>(reader: R) -> Vec<(i64, i64, i64)> {
        reader
            .lines()
            .map(|l| {
                let l = l.unwrap();
                let mut l = l.split(',');
                let (x, y, z) = (l.next().unwrap(), l.next().unwrap(), l.next().unwrap());
                (
                    x.parse::<i64>().unwrap(),
                    y.parse::<i64>().unwrap(),
                    z.parse::<i64>().unwrap(),
                )
            })
            .collect::<Vec<_>>()
    }

    fn distance(x: (i64, i64, i64), y: (i64, i64, i64)) -> i64 {
        ((x.0 - y.0).pow(2) + (x.1 - y.1).pow(2) + (x.2 - y.2).pow(2)).isqrt()
    }

    #[derive(Debug)]
    struct Dsu {
        parent: HashMap<(i64, i64, i64), (i64, i64, i64)>,
        rank: HashMap<(i64, i64, i64), i64>,
        forests: usize,
    }

    type Point = (i64, i64, i64);

    impl Dsu {
        fn new(points: &[(i64, i64, i64)], n: usize) -> Self {
            let mut rank = HashMap::new();
            let mut parent = HashMap::new();

            for &pt in points {
                rank.insert(pt, 1);
                parent.insert(pt, pt);
            }

            Self {
                parent,
                rank,
                forests: n,
            }
        }

        fn find(&mut self, x: Point) -> Point {
            let parent = *self.parent.get(&x).unwrap();

            if parent == x {
                return x;
            }

            let root = self.find(parent);
            self.parent.insert(x, root);
            root
        }

        fn union(&mut self, x: Point, y: Point) -> bool {
            let (x, y) = (self.find(x), self.find(y));

            if x == y {
                return false;
            }

            let x_rank = *self.rank.get(&x).unwrap();
            let y_rank = *self.rank.get(&y).unwrap();

            if x_rank > y_rank {
                *self.parent.get_mut(&y).unwrap() = x;
            } else if x_rank < y_rank {
                *self.parent.get_mut(&x).unwrap() = y;
            } else {
                *self.parent.get_mut(&x).unwrap() = y;
                *self.rank.get_mut(&y).unwrap() += 1;
            }

            self.forests -= 1;

            true
        }
    }

    #[derive(Eq, Hash, PartialEq, Ord, PartialOrd, Clone, Debug, Copy)]
    struct Cord {
        x: i64,
        y: i64,
        z: i64,
    }

    impl Cord {
        fn new(x: Point) -> Self {
            Self {
                x: x.0,
                y: x.1,
                z: x.2,
            }
        }
    }

    impl From<Point> for Cord {
        fn from(value: Point) -> Self {
            Self {
                x: value.0,
                y: value.1,
                z: value.2,
            }
        }
    }

    impl Into<Point> for Cord {
        fn into(self) -> Point {
            (self.x, self.y, self.z)
        }
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // looks like DSU
        // first parse
        // then get the closes one to each junction box, maybe it'll be 2sum for each
        // then dsu
        let cords = parse(reader);
        let n = cords.len();

        let mut edges: Vec<(i64, Cord, Cord)> = Vec::with_capacity(n * n);

        for i in 0..n {
            for j in (i + 1)..n {
                let dist = distance(cords[i], cords[j]);
                edges.push((dist, Cord::new(cords[i]), Cord::new(cords[j])));
            }
        }

        // it could also be done with dfs, idk yet what my starting point will be but I would've figured it out
        // , but I'll happily use dsu whenever I can
        edges.sort_unstable_by_key(|k| k.0);

        let mut dsu = Dsu::new(&cords, edges.len());

        let limit = if n < 50 { 10 } else { 1000 };

        for (_, u, v) in edges.iter().take(limit) {
            dsu.union((*u).into(), (*v).into());
        }

        // freq
        let mut counts: HashMap<Point, usize> = HashMap::new();
        for cord in &cords {
            let root = dsu.find(*cord);
            *counts.entry(root).or_default() += 1;
        }

        let mut sizes: Vec<usize> = counts.values().cloned().collect();
        sizes.sort_unstable_by(|a, b| b.cmp(a));

        let result = sizes.iter().take(3).product();

        Ok(result)
    }

    assert_eq!(40, part1(BufReader::new(TEST.as_bytes()))?);
    println!("=== Part 1 sample end ===");

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        // looks like DSU
        // first parse
        // then get the closes one to each junction box, maybe it'll be 2sum for each
        // then dsu
        let cords = parse(reader);
        let n = cords.len();

        let mut edges: Vec<(i64, Cord, Cord)> = Vec::with_capacity(n * n);

        for i in 0..n {
            for j in (i + 1)..n {
                let dist = distance(cords[i], cords[j]);
                edges.push((dist, Cord::new(cords[i]), Cord::new(cords[j])));
            }
        }

        // it could also be done with dfs, idk yet what my starting point will be but I would've figured it out
        // , but I'll happily use dsu whenever I can
        edges.sort_unstable_by_key(|k| k.0);

        let mut dsu = Dsu::new(&cords, cords.len());

        for (_, u, v) in edges.iter() {
            if dsu.union((*u).into(), (*v).into()) {
                if dsu.forests == 1 {
                    return Ok((u.x * v.x) as usize);
                }
            }
        }

        unreachable!()
    }

    assert_eq!(25272, part2(BufReader::new(TEST.as_bytes()))?);
    println!("=== Part 2 sample end ===");

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
