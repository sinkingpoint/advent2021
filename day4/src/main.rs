use libadvent::*;

#[derive(Debug, Clone)]
struct Board {
    board_numbers: Vec<Vec<i32>>,
    is_marked: Vec<Vec<bool>>,
    is_done: bool
}

impl Board {
    fn new(number_lines: &[String]) -> Self  {
        let mut lines = Vec::new();
        let mut dones = Vec::new();
        for i in number_lines {
            let nums: Vec<i32> = i.split_ascii_whitespace().map(|i| i.parse().unwrap()).collect();
            dones.push((0..nums.len()).map(|_| false).collect());
            lines.push(nums);

        }

        return Self {
            board_numbers: lines,
            is_marked: dones,
            is_done: false
        }
    }

    fn mark_num(&mut self, num: i32) {
        for y in 0..self.board_numbers.len() {
            for x in 0..self.board_numbers[y].len() {
                if self.board_numbers[y][x] == num {
                    self.is_marked[y][x] = true;
                }
            }
        }
    }

    fn is_done(&self) -> bool {
        for y in 0..self.is_marked.len() {
            if self.is_marked[y].iter().all(|t| *t) {
                return true;
            }
        }

        for x in 0..self.is_marked[0].len() {
            let mut ok = true;
            for y in 0..self.is_marked.len() {
                if !self.is_marked[y][x] {
                    ok = false;
                    break;
                }
            }

            if ok {
                return true;
            }
        }

        return false;
    }

    fn score(&self, last_num: i32) -> i32 {
        let mut base = 0;
        for y in 0..self.is_marked.len() {
            for x in 0..self.is_marked[y].len() {
                if !self.is_marked[y][x] {
                    base += self.board_numbers[y][x];
                }
            }
        }

        return base * last_num;
    }
}

fn main() {
    let input = must_read_input_to_lines();

    let numbers: Vec<i32> = input[0].split(",").map(|i| i.parse().unwrap()).collect();

    let mut boards = Vec::new();

    let mut i = 2;
    while i < input.len() {
        boards.push(Board::new(&input[i..i+5]));
        i += 6;
    }

    let mut first = false;

    'outer: for num in numbers.iter() {
        for board in boards.iter_mut() {
            board.mark_num(*num);
        }

        let mut count = 0;
        let mut last = Vec::new();

        for (i, board) in boards.iter_mut().enumerate() {
            let is_done = board.is_done();
            if is_done && !first {
                println!("Board {} is done. Score: {}", i+1, board.score(*num));
                first = true;
            }
            else if !is_done {
                count += 1;
            }
            else if is_done {
                if !board.is_done {
                    last.push((i, board.clone()));
                }
                board.is_done = true;
            }
        }

        if count == 0 {
            let (i, board) = last[0].clone();
            println!("Board {} is last. Score: {}", i+1, board.score(*num));
            break 'outer;
        }
    }

}
