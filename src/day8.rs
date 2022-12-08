use std::{collections::HashSet};

use crate::utils::*;

fn read_input() -> Vec<Vec<i32>> {
    read_lines("./src/day8_input").map(|line| {
        line.expect("cannot read line").chars().map(|c| {
            c.to_string().parse::<i32>().expect("not a number")
        }).collect()
    }).collect()
}

fn create_vec<T>(n: usize, default: T) -> Vec<T> where T : Copy {
    (0..n).map(|_| default).collect()
}

fn get_visible_tree_indices(rows: &Vec<Vec<i32>>) -> HashSet<usize> {
    let first_row = &rows[0];
    let mut max_heights_left: Vec<i32> = create_vec(rows.len(), -1);
    let mut max_heights_right: Vec<i32> = create_vec(rows.len(), -1);
    let mut max_heights_top: Vec<i32> = create_vec(first_row.len(), -1);
    let mut max_heights_bottom: Vec<i32> = create_vec(first_row.len(), -1);
    let mut vis_trees: HashSet<usize> = HashSet::new();

    for i in 0..rows.len() {
        let row = &rows[i];
        for j in 0..row.len() {
            let height = row[j];
            let key = i * rows.len() + j;

            if max_heights_left[i] < height {
                max_heights_left[i] = height;
                vis_trees.insert(key);
            }

            if max_heights_top[j] < height {
                max_heights_top[j] = height;
                vis_trees.insert(key);
            }
        }
    }

    for i in (0..rows.len()).rev() {
        let row = &rows[i];
        for j in (0..row.len()).rev() {
            let height = row[j];
            let key = i * rows.len() + j;

            if max_heights_right[i] < height {
                max_heights_right[i] = height;
                vis_trees.insert(key);
            }

            if max_heights_bottom[j] < height {
                max_heights_bottom[j] = height;
                vis_trees.insert(key);
            }
        }
    }

    vis_trees
}

pub fn problem1() {
    let rows = read_input();
    let vis_trees = get_visible_tree_indices(&rows);
    println!("{}", vis_trees.len());
}

fn get_viewing_score<T>(start_height: i32, mut it: T) -> i32 where T : Iterator<Item = i32> {
    let mut count = 0;
    while let Some(h) = it.next() {
        count += 1;
        if h >= start_height {
            return count
        }
    };
    count
}

fn get_viewing_score_at(row_idx: usize, col_idx: usize, rows: &Vec<Vec<i32>>) -> i32 {
    let num_rows = rows.len();
    let num_cols = rows[0].len();
    let start_height = rows[row_idx][col_idx];
    let score_right = get_viewing_score(start_height, ((col_idx + 1)..num_cols).map(|c| rows[row_idx][c]));
    let score_left = get_viewing_score(start_height, (0..col_idx).rev().map(|c| rows[row_idx][c]));
    let score_bottom = get_viewing_score(start_height, ((row_idx + 1)..num_rows).map(|r| rows[r][col_idx]));
    let score_top = get_viewing_score(start_height, (0..row_idx).rev().map(|r| rows[r][col_idx]));
    score_right * score_left * score_top * score_bottom
}

pub fn problem2() {
    let rows = read_input();
    let num_rows = rows.len();
    let num_cols = rows[0].len();

    let mut highest_score: i32 = 0;
    for row_idx in 0..num_rows {
        for col_idx in 0..num_cols {
            let score = get_viewing_score_at(row_idx, col_idx, &rows);
            if score > highest_score {
                highest_score = score;
            }
        }
    }

    println!("{}", highest_score);
}
