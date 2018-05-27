extern crate time_lock_puzzles;
extern crate num;

use std::collections::HashMap;

use time_lock_puzzles::*;

fn main() {
    let time_lock: u32 = 1_000_000;

    // Let's cteare 3 nodes of our net
    // We create a HashMap of id's and addresses
    // It is empty because there is no nodes yet
    let mut nodes = HashMap::<i32, String>::new();


    // Now we create first TlpServer entity
    // It takes our 'nodes' HashMap to know already existing nodes (there aren't any yet')
    let mut first = TlpServer::new(1, "127.0.0.1:7878".to_string(), time_lock, nodes.clone());

    // We create new thread and run first server on it
    let one = std::thread::spawn(move || {first.start().expect("Server 1 is down")});

    // We insert its id and address into HashMap so next servers know that it laready exists
    nodes.insert(1, "127.0.0.1:7878".to_string());

    // We create and run two more servers the same way
    let mut second = TlpServer::new(2, "127.0.0.1:7879".to_string(), time_lock, nodes.clone());
    let two = std::thread::spawn(move || {second.start().expect("Server 2 is down")});
    nodes.insert(2, "127.0.0.1:7879".to_string());

    let mut third = TlpServer::new(3, "127.0.0.1:7880".to_string(), time_lock, nodes.clone());
    let three = std::thread::spawn(move || {third.start().expect("Server 3 is down")});
    nodes.insert(3, "127.0.0.1:7880".to_string());

    // Now let there be a client with index 0
    let mut client = TlpClient::new(0, nodes);

    // Client requests puzzle
    // It returns 'Result' so we let him do his request till he receives 'Ok'
    loop { 
       match client.request() {
           Ok(()) => { break },
           _ => {}
       }
    }


    // Client solves puzzles (this takes lime-lock squarings)
    println!("[Client is solving puzzles...]");
    client.solve();
    
    // Client sends solutions to nodes
     client.send().unwrap();

    // After verification nodes send messages to each other and, after all of them
    // receive positive verdict from all other nodes, they write Client action to
    // blockchain (there is no blockchain here, it is only marked with console message)

    // This programm will not finish by itself (servers keep running)
    // So it needs to be manually shut down

    // These lines will not be reached if there are no errors in threads
    one.join().unwrap();
    two.join().unwrap();
    three.join().unwrap();
}
