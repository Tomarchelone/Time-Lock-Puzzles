extern crate time_lock_puzzles;
extern crate num;

use std::collections::HashMap;
use time_lock_puzzles::*;

fn main() {
    // let's create 100 auditors (main nodes of our net)
    let number_of_auditors: usize = 10;
    let mut auditors = Vec::<Auditor>::new();

    // we want total computation time to be, for example, 100000 squarings
    let time_lock = 100_000;
    for i in 0..number_of_auditors {
        auditors.push(Auditor::new(i as i32, time_lock));
    }

    print!("Auditors ready\n");

    // now let there be some solver with index, for example, 0
    let solver_id = 0;
    let mut puzzles = HashMap::<i32, TimeLockPuzzle>::new();

    for i in 0..number_of_auditors {
        puzzles.insert(i as i32, auditors[i].serve_puzzle(solver_id, number_of_auditors));
    }

    print!("Puzzles served\n");

    // we now have 100 puzzles, solver solves them, using result of one
    // puzzle as input for the next one
    let solutions = time_lock_puzzles::solve(&puzzles);

    print!("Puzzles solved\n");

    // check that all solutions are valid
    // this code will panic if something is wrong
    for i in 0..number_of_auditors {
        assert_eq!(auditors[i].verify(solver_id, &solutions), true);
    }
}
