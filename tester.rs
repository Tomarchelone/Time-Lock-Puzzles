extern crate time_lock_puzzles;

use time_lock_puzzles::*;

fn main() {
    // Let's crate a puzzle generator.
    // If we want we can change bitsize and assurance
    let puzzle_generator = PuzzleGenerator::new();

    // We can now generte a puzzle with 100 squarings (not a big number).
    // 'gen_puzzle()' function takes anything that satisfies 'ToBigUint' trait
    // for example, any built-in integer type.
    let (puzzle, true_key) = puzzle_generator.gen_puzzle(100);

    // Now we have a puzzle and a key. Now we keep the key and send
    // the puzzle to solver.

    // As solver we receive the puzzle and call 'solve()' method on it:
    let computed_key = puzzle.solve();
    // As soon as solver computes the key, he can send it back.

    // Let's check if it is computed correctly.
    // This program will panic if the values are not equal.
    assert_eq!(computed_key, true_key);

    // We can see them by ourselves
    println!("{}\n{}", computed_key, true_key);
}
