use std::io::{self, prelude::*};

struct Grid {
    data: Vec<char>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(data: &[char], width: usize, height: usize) -> Grid {
        Grid {
            data: data.to_vec(),
            width,
            height,
        }
    }

    fn in_bounds(&self, x: isize, y: isize) -> bool {
        x >= 0 && y >= 0 && x < self.width as isize && y < self.height as isize
    }

    fn get_char_at(&self, x: isize, y: isize) -> Option<char> {
        if !self.in_bounds(x, y) {
            return None;
        }

        self.data
            .get((x as usize) + (y as usize) * self.width)
            .copied()
    }

    fn find_word_in_direction(
        &self,
        start_x: isize,
        start_y: isize,
        length: usize,
        direction: (i8, i8),
    ) -> Option<String> {
        let mut word: Vec<char> = vec![];
        let mut x = start_x;
        let mut y = start_y;

        for _ in 0..length {
            let c = self.get_char_at(x, y);

            if c.is_none() {
                return None;
            }

            word.push(c?);

            x += direction.0 as isize;
            y += direction.1 as isize;
        }

        Some(word.into_iter().collect())
    }

    fn find_all_words_from_point(
        &self,
        start_x: isize,
        start_y: isize,
        length: usize,
    ) -> Vec<Option<String>> {
        ALL_DIRECTIONS
            .map(|dir| self.find_word_in_direction(start_x, start_y, length, dir))
            .to_vec()
    }

    fn find_all_words(&self, word_length: usize) -> Vec<String> {
        let mut results = vec![];

        for x in 0..self.width {
            for y in 0..self.height {
                let words = self.find_all_words_from_point(x as isize, y as isize, word_length);

                let words: Vec<String> = words.into_iter().flatten().collect();
                results.extend(words);
            }
        }

        results
    }

    fn count_found_word(&self, word: &str) -> usize {
        let all_words = self.find_all_words(word.len());

        let found_words = all_words.iter().filter(|&w| w == word);

        found_words.count()
    }

    fn find_all_words_from_centre_point(
        &self,
        start_x: isize,
        start_y: isize,
        length: usize,
    ) -> Vec<Option<String>> {
        X_DIRECTIONS
            .map(|dir| {
                self.find_word_in_direction(
                    start_x - dir.0 as isize,
                    start_y - dir.1 as isize,
                    length,
                    dir,
                )
            })
            .to_vec()
    }

    fn find_all_x_words(&self, word_length: usize) -> Vec<String> {
        let mut results = vec![];

        for x in 0..self.width {
            for y in 0..self.height {
                let words =
                    self.find_all_words_from_centre_point(x as isize, y as isize, word_length);

                let words: Vec<String> = words.into_iter().flatten().collect();

                // An X string is where both words are the same
                if words.len() == 2
                    && (words[0] == words[1] || words[0] == reverse_string(&words[1]))
                {
                    // println!("Found at {} {}", x, y);
                    // println!("{:?}", words);
                    results.extend(words);
                }
            }
        }

        results
    }

    fn count_found_x_word(&self, word: &str) -> usize {
        let all_words = self.find_all_x_words(word.len());

        let found_words = all_words
            .iter()
            .filter(|&w| w == word || *w == reverse_string(word));

        found_words.count() / 2
    }
}

fn reverse_string(original: &str) -> String {
    original.chars().rev().collect()
}

const ALL_DIRECTIONS: [(i8, i8); 8] = [
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
];

const X_DIRECTIONS: [(i8, i8); 2] = [(1, 1), (1, -1)];

fn parse(lines: &[String]) -> Grid {
    let grid_data = lines.iter().flat_map(|l| l.chars()).collect::<Vec<_>>();

    let height = lines.len();
    let width = lines[0].len();

    Grid::new(&grid_data, width, height)
}

fn main() -> std::io::Result<()> {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();

    let grid = parse(&lines);

    let xmas_count = grid.count_found_word("XMAS");

    println!("XMAS appears {} times", xmas_count);

    let x_mas_count = grid.count_found_x_word("MAS");

    println!("X-MAS appears {} times", x_mas_count);

    Ok(())
}
