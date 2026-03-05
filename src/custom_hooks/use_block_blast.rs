use dioxus::prelude::*;

use crate::{MainBoardStruct, Result};

fn clean_piece(piece: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut min_row = usize::MAX;
    let mut min_col = usize::MAX;
    let mut max_row = usize::MIN;
    let mut max_col = usize::MIN;

    for i in 0..piece.len() {
        for j in 0..piece[i].len() {
            if piece[i][j] == 1 {
                min_row = min_row.min(i);
                min_col = min_col.min(j);
                max_row = max_row.max(i);
                max_col = max_col.max(j);
            }
        }
    }

    if min_row == usize::MAX {
        return vec![vec![0; 0]; 0];
    }

    let mut cleaned_piece = vec![vec![0; max_col - min_col + 1]; max_row - min_row + 1];
    for i in min_row..=max_row {
        for j in min_col..=max_col {
            cleaned_piece[i - min_row][j - min_col] = piece[i][j];
        }
    }
    return cleaned_piece;
}

fn clean_and_paint_line(
    grid: &mut Vec<Vec<usize>>,
    painted_grid: &mut Vec<Vec<usize>>,
    row: Option<usize>,
    col: Option<usize>,
) {
    if let Some(r) = row {
        for j in 0..grid[r].len() {
            grid[r][j] = 0;
            painted_grid[r][j] = if painted_grid[r][j] == 1 { 3 } else { 2 };
        }
    }
    if let Some(c) = col {
        for i in 0..grid.len() {
            grid[i][c] = 0;
            painted_grid[i][c] = if painted_grid[i][c] == 1 { 3 } else { 2 };
        }
    }
}

fn clean_grid(painted_grid: &mut Vec<Vec<usize>>, grid: &mut Vec<Vec<usize>>) {
    let mut col_count = vec![0; grid[0].len()];

    for i in 0..grid.len() {
        let mut row_count = 0;
        for j in 0..grid[i].len() {
            if grid[i][j] >= 1 {
                row_count += 1;
                col_count[j] += 1;
            }
        }

        if row_count == grid[i].len() {
            clean_and_paint_line(grid, painted_grid, Some(i), None);
        }
    }

    for i in 0..col_count.len() {
        if col_count[i] == grid.len() {
            clean_and_paint_line(grid, painted_grid, None, Some(i));
        }
    }
}

fn update_grid(
    piece: &Vec<Vec<usize>>,
    grid: &Vec<Vec<usize>>,
    row: usize,
    col: usize,
) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let mut grid = grid.clone();
    let mut painted_grid = grid.clone();
    for i in 0..piece.len() {
        for j in 0..piece[i].len() {
            grid[row + i][col + j] += piece[i][j];
            painted_grid[row + i][col + j] += piece[i][j] * 2;
        }
    }

    clean_grid(&mut painted_grid, &mut grid);
    return (painted_grid, grid);
}

fn can_fit_pieces(
    current_piece: &Vec<Vec<usize>>,
    current_grid: &Vec<Vec<usize>>,
    row: usize,
    col: usize,
) -> bool {
    for i in 0..current_piece.len() {
        for j in 0..current_piece[i].len() {
            if row + i >= current_grid.len()
                || col + j >= current_grid[row + i].len()
                || (current_grid[row + i][col + j] + current_piece[i][j] > 1)
            {
                return false;
            }
        }
    }

    return true;
}

fn fit_pieces(
    pieces: &Vec<Vec<Vec<usize>>>,
    grid: &Vec<Vec<usize>>,
    current_piece_index: usize,
    piece_status: &mut Vec<bool>,
) -> Option<Vec<Result>> {
    piece_status[current_piece_index] = true;

    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if can_fit_pieces(&pieces[current_piece_index], grid, row, col) {
                let (painted_grid, updated_grid) =
                    update_grid(&pieces[current_piece_index], &grid, row, col);

                if piece_status.iter().all(|&s| s) {
                    return Some(vec![Result {
                        updated_grid,
                        painted_grid,
                    }]);
                }

                for next_piece_index in 0..piece_status.len() {
                    if !piece_status[next_piece_index] {
                        let next_res =
                            fit_pieces(pieces, &updated_grid, next_piece_index, piece_status);

                        if let Some(mut results) = next_res {
                            results.insert(
                                0,
                                Result {
                                    updated_grid,
                                    painted_grid,
                                },
                            );
                            return Some(results);
                        }
                    }
                }
            }
        }
    }

    piece_status[current_piece_index] = false;
    None
}

pub fn use_block_blast() -> impl FnMut(Event<MouseData>) {
    let main_board_struct = use_context::<MainBoardStruct>();
    let grid = main_board_struct.board;
    let pieces = main_board_struct.pieces;
    let mut solutions = main_board_struct.solutions;

    let on_solve = move |_| {
        let mut cleaned_pieces = Vec::new();
        for piece in &*pieces.read() {
            let cleaned_piece = clean_piece(piece);
            if cleaned_piece.len() > 0 {
                cleaned_pieces.push(cleaned_piece);
            }
        }

        for i in 0..cleaned_pieces.len() {
            let result = fit_pieces(
                &cleaned_pieces,
                &grid.read(),
                i,
                &mut vec![false; cleaned_pieces.len()],
            );

            if let Some(res) = result {
                solutions.set(Some(res));
                break;
            }
        }
    };

    return on_solve;
}
