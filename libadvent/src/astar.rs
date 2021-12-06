use std::collections::HashMap;
use std::hash::Hash;

pub trait AStarEntry: Eq + Hash + Clone {
    fn heuristic(&self) -> f64;
    fn distance(&self, other: &Self) -> f64;
    fn get_neighbors(&self) -> Vec<Self>;
    fn decide<T: Iterator<Item=Self>>(mut between: T) -> Self {
        return between.next().unwrap();
    }
}

struct AStarNodeMeta<T> where T: AStarEntry {
    f_score: f64,
    g_score: f64,
    came_from: Option<T>
}

impl<T> AStarNodeMeta<T> where T: AStarEntry {
    fn new() -> Self {
        return Self {
            f_score: f64::INFINITY,
            g_score: f64::INFINITY,
            came_from: None
        }
    }

    fn with_f_score(mut self, f: f64) -> Self {
        self.f_score = f;
        return self;
    }
}

fn reconstruct_path<T>(goal: T, metas: &mut HashMap<T, AStarNodeMeta<T>>) -> Vec<T> where T: AStarEntry {
    let mut path = Vec::new();
    let mut current = Some(goal);
    while let Some(next) = current {
        current = metas.get_mut(&next).unwrap().came_from.take();
        path.push(next);
    }

    path.reverse();
    return path;
}

pub fn astar<T>(start: T, goal: T) -> Result<Vec<T>, ()> where T: AStarEntry {
    let mut openset = Vec::new();
    openset.push(start.clone());

    let mut metas = HashMap::new();
    let h = start.heuristic();
    metas.insert(start, AStarNodeMeta::new().with_f_score(h));

    while openset.len() > 0 {
        let min_f_score = openset.iter().map(|o| metas.get(o).unwrap().f_score).reduce(|a, b| f64::min(a, b)).unwrap();
        let next_pos = openset.iter().position(|c| metas.get(c).unwrap().f_score == min_f_score).unwrap();
        let current = openset.remove(next_pos);

        if current == goal {
            return Ok(reconstruct_path(current, &mut metas));
        }

        for neighbor in current.get_neighbors() {
            let mut neigbor_meta = metas.remove(&neighbor).unwrap_or(AStarNodeMeta::new());
            let current_meta = metas.get(&current).unwrap();
            let new_score = current_meta.g_score + current.distance(&neighbor);
            if new_score < neigbor_meta.g_score {
                neigbor_meta.came_from = Some(current.clone());
                neigbor_meta.g_score = new_score;
                neigbor_meta.f_score = new_score + neighbor.heuristic();
            }

            metas.insert(neighbor.clone(), neigbor_meta);
            if !openset.contains(&neighbor) {
                openset.push(neighbor);
            }
        }
    }

    return Err(());
}