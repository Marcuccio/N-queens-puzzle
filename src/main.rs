use std::io;
use rand::Rng;

pub struct Conflicts {
    collapsed_rows_conflicts: Vec<usize>,
    collapsed_diag1_conflicts: Vec<usize>,
    collapsed_diag2_conflicts: Vec<usize>,
    diag1_offset: usize,
}

fn main() {

    println!("Please input the number of queens.");

    let mut n_queens = String::new();

    io::stdin().read_line(&mut n_queens).expect("Failed to read line");

    let n: usize = match n_queens.trim().parse() {
        Ok(n) => n,
        Err(_) => panic!(),
    };

    let mut chessboard_1d: Vec<usize> = (0..n).map(|_| rand::thread_rng().gen_range(0, n)).collect();

    let mut conflicts = Conflicts {
        collapsed_rows_conflicts: vec![0; n],
        collapsed_diag1_conflicts: vec![0; 2*n-1],
        collapsed_diag2_conflicts: vec![0; 2*n-1],
        diag1_offset: chessboard_1d.len() - 1
    };

    compute_all_conflicts(&chessboard_1d, &mut conflicts);

    let mut candidates: Vec<usize> = Vec::with_capacity(n/2);

    let mut moves: usize = 0;

    loop {
        // Find the queens with more conflicts
        let mut max_conflicts: usize = usize::min_value();

        candidates.clear();

        for col in 0..n {
            let row = chessboard_1d[col];

            let conflicts: usize = (conflicts.collapsed_rows_conflicts[row] +
                    conflicts.collapsed_diag1_conflicts[conflicts.diag1_offset + col - row] +
                    conflicts.collapsed_diag2_conflicts[row + col]) - 3;

            if conflicts == max_conflicts {
                candidates.push(col);
            } else if conflicts > max_conflicts {
                max_conflicts = conflicts;
                candidates.clear();
                candidates.push(col);
            }
        }

        if max_conflicts == 0 {
            // Checked *every* queen and found no conflicts
            break;
        }

        // Pick a random queen from those that had the most conflicts
        let rand_index = rand::thread_rng().gen_range(0, candidates.len());
        let worst_queen_column: usize = candidates[rand_index];
        let worst_queen_row: usize = chessboard_1d[worst_queen_column];

        // Move her to the place with the least conflicts.
        let mut min_conflicts: usize = usize::max_value();
        candidates.clear();

        for row in 0..n {

            if row == worst_queen_row {
                continue;
            }

            let conflicts: usize = conflicts.collapsed_rows_conflicts[row] +
                    conflicts.collapsed_diag1_conflicts[conflicts.diag1_offset + worst_queen_column - row] +
                    conflicts.collapsed_diag2_conflicts[worst_queen_column + row];

            if conflicts == min_conflicts {
                candidates.push(row);
            } else if conflicts < min_conflicts {
                min_conflicts = conflicts;
                candidates.clear();
                candidates.push(row);
            }
        }

        if !candidates.is_empty() {
            let rand_index = rand::thread_rng().gen_range(0, candidates.len());
            chessboard_1d[worst_queen_column] = candidates[rand_index];

            let worst_queen_diag1 = conflicts.diag1_offset + worst_queen_column - worst_queen_row ;
            let worst_queen_diag2 = worst_queen_column + worst_queen_row;

            conflicts.collapsed_rows_conflicts[worst_queen_row] -= 1;
            conflicts.collapsed_diag1_conflicts[worst_queen_diag1] -= 1;
            conflicts.collapsed_diag2_conflicts[worst_queen_diag2] -= 1;

            let new_queen_diag1 = conflicts.diag1_offset + worst_queen_column - candidates[rand_index] ;
            let new_queen_diag2 = worst_queen_column + candidates[rand_index];

            conflicts.collapsed_rows_conflicts[candidates[rand_index]] += 1;
            conflicts.collapsed_diag1_conflicts[new_queen_diag1] += 1;
            conflicts.collapsed_diag2_conflicts[new_queen_diag2] += 1;
        }

        if moves == chessboard_1d.len() * 2 {
            // Trying too long... start oveprintln!("{:?}", chessboard_1d);r.
            shuffle_up(&mut chessboard_1d);

            for i in 0..(2*n-1) {
                if i < n {
                    conflicts.collapsed_rows_conflicts[i] = 0;
                }
                conflicts.collapsed_diag1_conflicts[i] = 0;;
                conflicts.collapsed_diag2_conflicts[i] = 0;;
            }
            compute_all_conflicts(&chessboard_1d, &mut conflicts);
            moves = 0;
            // println!("--------------------------")
        }

        // println!("Move: #{}: {:?}", moves, chessboard_1d);
        moves += 1;
    }

    println!("COMPLETED");
    println!("{:?}", chessboard_1d);
}

pub fn shuffle_up(chessboard_1d: &mut Vec<usize>) {
    let mut i = 0;

    while i < chessboard_1d.len() {
        chessboard_1d[i] = i;
        i += 1;
    }

    i = 0;

    while i < chessboard_1d.len() {
        let j = rand::thread_rng().gen_range(0, chessboard_1d.len());
        let row_to_swap = chessboard_1d[i];
        chessboard_1d[i] = chessboard_1d[j];
        chessboard_1d[j] = row_to_swap;
        i += 1;
    }
}

pub fn compute_all_conflicts(chessboard_1d: &Vec<usize>, conflicts: &mut Conflicts) {
    let mut col = 0;

    while col < chessboard_1d.len() {

        let row = chessboard_1d[col];

        let diag1 = conflicts.diag1_offset + col - row ;
        let diag2 = col + row;

        conflicts.collapsed_rows_conflicts[row] += 1;
        conflicts.collapsed_diag1_conflicts[diag1] += 1;
        conflicts.collapsed_diag2_conflicts[diag2] += 1;

        col += 1;
    }
}
