use std::time::Instant;

use rand::{rng, seq::SliceRandom}; // To track performance

/// **[IMPLEMENTED]**
/// Finds the minimal set of clues required for the grid to have a unique solution.
///
/// # Arguments
/// * `grid` - The complete, solved grid.
/// * `size` - The grid size N.
///
/// # Returns
/// A Vec<u8> containing the 4*N minimal clues (0 indicates a removed clue).
pub fn create_puzzle(grid: &Vec<Vec<u8>>, size: usize) -> Vec<u8> {
    let now = Instant::now();

    // 1. Start with the full set of clues calculated from the solved grid.
    let all_clues = calculate_all_clues(grid, size);
    let mut puzzle_clues = all_clues.clone();
    let total_clues = 4 * size;

    // 2. Create a list of all 4*N clue indices (0 to TOTAL_CLUES - 1).
    let mut indices: Vec<usize> = (0..total_clues).collect();
    // Shuffle the indices to randomly select which clue to test for removal.
    indices.shuffle(&mut rng());

    // 3. Iteratively try to remove a clue if the solution remains unique.
    for i in indices {
        let original_clue = puzzle_clues[i];

        // Tentatively remove the clue (set it to 0)
        puzzle_clues[i] = 0;

        // Check uniqueness with the reduced set of clues:

        // The solver needs a copy of an empty grid to start with.
        let mut temp_grid = vec![vec![0; size]; size];
        let mut solution_count = 0;

        // Run the solver to count solutions. We only need to know if count > 1.
        solve_skyscraper(&mut temp_grid, size, &puzzle_clues, &mut solution_count);

        // If the solution is still unique (count == 1), the clue was redundant.
        if solution_count == 1 {
            // Keep the clue removed (puzzle_clues[i] remains 0)
        } else {
            // Uniqueness was lost (count != 1). Restore the clue.
            puzzle_clues[i] = original_clue;
        }
    }

    println!("Puzzle reduction time: {} ms", now.elapsed().as_millis());

    puzzle_clues
}

/// Calculates all 4*N clues (Top, Right, Bottom, Left) for a solved grid.
fn calculate_all_clues(grid: &Vec<Vec<u8>>, size: usize) -> Vec<u8> {
    let total_clues = 4 * size;
    let mut all_clues = Vec::with_capacity(total_clues);

    // Helper to extract a column for clarity
    let get_col = |c: usize| grid.iter().map(|row| row[c]).collect::<Vec<u8>>();

    // 1. Top Clues (Index 0 to N-1)
    // Observer is looking from the top, viewing down the column (Normal order).
    for col in 0..size {
        let column = get_col(col);
        all_clues.push(calculate_clue_from_direction(&column));
    }

    // 2. Right Clues (Index N to 2N-1)
    // Observer is looking from the right, viewing left across the row (Reverse order).
    for row in 0..size {
        let reversed_row: Vec<u8> = grid[row].iter().copied().rev().collect();
        all_clues.push(calculate_clue_from_direction(&reversed_row));
    }

    // 3. Bottom Clues (Index 2N to 3N-1)
    // Observer is looking from the bottom, viewing up the column (Reverse order).
    for col in 0..size {
        let column = get_col(col);
        let reversed_column: Vec<u8> = column.iter().copied().rev().collect();
        all_clues.push(calculate_clue_from_direction(&reversed_column));
    }

    // 4. Left Clues (Index 3N to 4N-1)
    // Observer is looking from the left, viewing right across the row (Normal order).
    for row in 0..size {
        // The grid[row] slice is already in the correct Left-to-Right order.
        all_clues.push(calculate_clue_from_direction(&grid[row]));
    }

    all_clues
}

// Helper function to calculate the clue from one direction (reading forward)
// This version is safe for partial lines/columns (it ignores 0s).
fn calculate_clue_from_direction(row_or_col: &[u8]) -> u8 {
    let mut max_height = 0;
    let mut visible_count = 0;
    for &height in row_or_col {
        // Only count placed skyscrapers (non-zero)
        if height != 0 && height > max_height {
            max_height = height;
            visible_count += 1;
        }
    }
    visible_count
}

/// Checks if the current configuration of the grid satisfies the given clues for the given side.
fn check_clues(grid: &Vec<Vec<u8>>, size: usize, clues: &[u8]) -> bool {
    let all_filled = grid.iter().all(|row| row.iter().all(|&cell| cell != 0));
    if !all_filled {
        // Only check for partially filled constraints (partial checking is complex and slow).
        // For simplicity and to run a fast uniqueness check, we only check the full grid.
        return true;
    }

    let calculated_clues = calculate_all_clues(grid, size);

    // Check if the calculated clues match the provided clues, ignoring 0 clues
    let total_clues = size * 4;
    for i in 0..total_clues {
        if clues[i] != 0 && calculated_clues[i] != clues[i] {
            return false;
        }
    }

    true
}

/// Recursive backtracking solver to find all solutions matching the clues.
/// Returns true if two solutions are found (optimization), false otherwise.
fn solve_skyscraper(
    grid: &mut Vec<Vec<u8>>,
    size: usize,
    clues: &[u8],
    solution_count: &mut u32,
) -> bool {
    // Optimization: If we've already found two solutions, stop immediately.
    if *solution_count >= 2 {
        return true;
    }

    // 1. Find the next empty cell (value 0)
    let mut current_row = 0;
    let mut current_col = 0;
    let mut found_empty = false;
    // ... (search loop for empty cell remains here) ...
    for r in 0..size {
        for c in 0..size {
            if grid[r][c] == 0 {
                current_row = r;
                current_col = c;
                found_empty = true;
                break;
            }
        }
        if found_empty {
            break;
        }
    }

    // 2. Base Case: If the grid is full
    if !found_empty {
        // IMPORTANT: Check if the complete grid satisfies ALL clues (0s ignored).
        // This relies on the separate 'check_clues' function being correct.
        if check_clues(grid, size, clues) {
            *solution_count += 1;
        }
        return *solution_count >= 2;
    }

    // 3. Recursive Step: Try numbers 1 to N
    for num in 1..=(size as u8) {
        if is_valid_placement(grid, current_row, current_col, num, size) {
            // Tentative assignment
            grid[current_row][current_col] = num;

            // **TEMPORARY CHANGE: NO PARTIAL PRUNING HERE**
            // We are deliberately letting the slow but potentially correct solver explore more.
            // This is the reversion to the previous, less efficient but more reliable state.

            // Recurse: continue finding solutions
            if solve_skyscraper(grid, size, clues, solution_count) {
                return true; // Stop if we hit 2 solutions (optimization)
            }

            // FULL BACKTRACK: Recursion failed, reset the cell.
            grid[current_row][current_col] = 0;
        }
    }

    false // No solution found starting from this path
}

/// Checks if placing 'num' at (row, col) is valid based on Skyscraper rules.
/// For generation, we only need to check the row and column uniqueness.
fn is_valid_placement(grid: &Vec<Vec<u8>>, row: usize, col: usize, num: u8, size: usize) -> bool {
    // Check row
    for c in 0..size {
        if grid[row][c] == num {
            return false;
        }
    }

    // Check column
    for r in 0..size {
        if grid[r][col] == num {
            return false;
        }
    }

    true // Placement is valid
}

/// Recursive backtracking function to fill the grid randomly.
fn fill_grid(grid: &mut Vec<Vec<u8>>, size: usize) -> bool {
    // 1. Find the next empty cell (value 0)
    let mut current_row = 0;
    let mut current_col = 0;
    let mut found_empty = false;

    // Search for the next empty cell
    for r in 0..size {
        for c in 0..size {
            if grid[r][c] == 0 {
                current_row = r;
                current_col = c;
                found_empty = true;
                break;
            }
        }
        if found_empty {
            break;
        }
    }

    // 2. Base Case: If no empty cells are found, the grid is complete and valid.
    if !found_empty {
        return true;
    }

    // 3. Recursive Step: Try numbers 1 to N in a random order.
    let mut numbers: Vec<u8> = (1..=(size as u8)).collect();
    // Shuffle the numbers to ensure a random grid each time!
    numbers.shuffle(&mut rng());

    for num in numbers {
        if is_valid_placement(grid, current_row, current_col, num, size) {
            // Tentative assignment
            grid[current_row][current_col] = num;

            // Recurse to fill the rest of the grid
            if fill_grid(grid, size) {
                return true; // Solution found!
            }

            // BACKTRACK: If the recursion failed, reset the cell and try the next number.
            grid[current_row][current_col] = 0;
        }
    }

    // 4. No number worked for this cell, backtrack to the previous call.
    false
}

/// Generates a complete, valid, and random Skyscraper grid of size N x N.
pub fn generate_complete_grid(size: usize) -> Option<Vec<Vec<u8>>> {
    // Initialize an empty grid (0 represents an empty cell)
    let mut grid = vec![vec![0; size]; size];

    if fill_grid(&mut grid, size) {
        Some(grid)
    } else {
        None // Should not happen for a valid size N > 0
    }
}
