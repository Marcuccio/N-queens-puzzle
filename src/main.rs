use std::io;
use rand::Rng;

fn main() {

    println!("Please input the number of queens.");

    let mut n_queens = String::new();

    io::stdin().read_line(&mut n_queens).expect("Failed to read line");

    let n: usize = match n_queens.trim().parse() {
        Ok(n) => n,
        Err(_) => panic!(),
    };

    let mut rows: Vec<usize> = (0..n).map(|_| rand::thread_rng().gen_range(0, n)).collect();

    let mut candidates: Vec<usize> = vec![0, n/2];

    // println!("Queens pos: {:?}", rows);

    let mut moves: usize = 0;

    loop {
        // Find nastiest queen
        let mut max_conflicts: u16 = u16::min_value();
        candidates.clear();

        for (col, row)  in rows.iter().enumerate() {
            let conflicts: u16 = conflicts(*row, col, &rows);
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

        // println!("Move: #{}, max_conflicts: {} in C{} out-of: {}", moves, max_conflicts, worst_queen_column, candidates.len());

        // Move her to the place with the least conflicts.
        let mut min_conflicts:u16 = u16::max_value();
        candidates.clear();

        for row in 0..rows.len() {
            let conflicts: u16 = conflicts(row, worst_queen_column, &rows);

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
            rows[worst_queen_column] = candidates[rand_index];
            // println!("Move: #{}, min_conflicts: {} in R{}, out-of: {}", moves, min_conflicts, candidates[rand_index], candidates.len());
        }

        if moves == rows.len() * 2 {
            // Trying too long... start oveprintln!("{:?}", rows);r.
            shuffle_up(&mut rows);
            moves = 0;
            // println!("--------------------------")
        }

        // println!("Move: #{}: {:?}", moves, rows);
        moves += 1;
    }

    println!("COMPLETED");
    println!("{:?}", rows);
}

/**
 * Returns the number of queens that conflict with (row,col), not
 * counting the queen in column col.
 */
pub fn conflicts(row: usize, col: usize, rows: &Vec<usize>) -> u16 {
    let mut count: u16 = 0;

    let mut c = 0;

    while c < rows.len() {
        if c == col { c += 1; continue; }

        let r = rows[c];

        if r == row {
            // same row
            count += 1;
        } else {
            let row_diff = if r > row {
                r - row
            } else {
                row - r
            };

            let col_diff = if c > col {
                c - col
            } else {
                col - c
            };

            if row_diff == col_diff {
                // same diag
                count += 1;
            }
        }
        c += 1;
    }

    // println!("# conflicts for C{}R{} = {}", col, row, count);

    count
}

pub fn shuffle_up(rows: &mut Vec<usize>) {
    let mut i = 0;

    while i < rows.len() {
        rows[i] = i;
        i += 1;
    }

    i = 0;

    while i < rows.len() {
        let j = rand::thread_rng().gen_range(0, rows.len());
        let row_to_swap = rows[i];
        rows[i] = rows[j];
        rows[j] = row_to_swap;
        i += 1;
    }
}
