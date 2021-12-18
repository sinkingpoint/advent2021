use libadvent::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct RiskNode {
    x: i32,
    y: i32,
    risk: i32
}

impl AStarEntry for RiskNode {
    fn distance(&self, other: &Self) -> f64 {
        return other.risk as f64
    }
}

fn main() {
    let input = must_read_input_to_lines();
    let mut nodes = Vec::new();
    for (y, line) in input.iter().enumerate() {
        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            row.push(RiskNode{
                x: x as i32,
                y: y as i32,
                risk: c as i32 - '0' as i32
            });
        }

        nodes.push(row);
    }

    let moved_nodes = nodes.clone();

    let get_neigbors = move |node: &RiskNode| {
        let mut neigbors = Vec::new();

        if node.x > 0 {
            neigbors.push(moved_nodes[node.y as usize][node.x as usize -1].clone());
        }

        if node.y > 0 {
            neigbors.push(moved_nodes[node.y as usize - 1][node.x as usize].clone());
        }

        if node.y < moved_nodes.len() as i32 - 1 {
            neigbors.push(moved_nodes[node.y as usize + 1][node.x as usize].clone());
        }

        if node.x < moved_nodes[0].len() as i32 - 1 {
            neigbors.push(moved_nodes[node.y as usize][node.x as usize + 1].clone());
        }

        neigbors
    };

    let heuristic = move |a: &RiskNode, b: &RiskNode| {
        let (start_x, end_x) = (a.x.min(b.x), a.x.max(b.x));
        let (start_y, end_y) = (a.y.min(b.y), a.y.max(b.y));

        return ((end_x - start_x) + (end_y - start_y)) as f64;
    };

    let path = astar(nodes[0][0].clone(), nodes[nodes.len() - 1][nodes[0].len() - 1].clone(), &get_neigbors as &AStarStep<RiskNode>, &heuristic as &AStarHeuristic<RiskNode>).unwrap();
    let score = path[1..].iter().map(|r| r.risk).sum::<i32>();

    println!("Part1 Score: {}", score);

    let part2_map_first = nodes.iter().map(|line| (0..5).map(|i| {
        line.iter().map(|node| {
            let mut n = node.clone();
            n.x = n.x + (line.len() * i as usize) as i32;
            n.risk = n.risk + i;
            if n.risk > 9 {
                n.risk -= 9;
            }
            n
        }).collect::<Vec<RiskNode>>()
    }).flatten().collect::<Vec<RiskNode>>()).collect::<Vec<Vec<RiskNode>>>();

    let part2_map = (0..(5 * part2_map_first.len())).map(|i| {
        part2_map_first[i % part2_map_first.len()].iter().map(|node| {
            let mut n = node.clone();
            n.risk = n.risk + (i as i32 / part2_map_first.len() as i32);
            while n.risk > 9 {
                n.risk -= 9;
            }
            n.y = i as i32;
            n
        }).collect::<Vec<RiskNode>>()
    }).collect::<Vec<Vec<RiskNode>>>();

    let moved_map = part2_map.clone();

    let get_neigbors = move |node: &RiskNode| {
        let mut neigbors = Vec::new();

        if node.x > 0 {
            neigbors.push(moved_map[node.y as usize][node.x as usize -1].clone());
        }

        if node.y > 0 {
            neigbors.push(moved_map[node.y as usize - 1][node.x as usize].clone());
        }

        if node.y < moved_map.len() as i32 - 1 {
            neigbors.push(moved_map[node.y as usize + 1][node.x as usize].clone());
        }

        if node.x < moved_map[0].len() as i32 - 1 {
            neigbors.push(moved_map[node.y as usize][node.x as usize + 1].clone());
        }
        neigbors
    };

    let path = astar(part2_map[0][0].clone(), part2_map[part2_map.len() - 1][part2_map[0].len() - 1].clone(), &get_neigbors as &AStarStep<RiskNode>, &heuristic as &AStarHeuristic<RiskNode>).unwrap();
    let score = path[1..].iter().map(|r| r.risk).sum::<i32>();

    println!("Part2 Score: {} with {} steps", score, path.len());
}
