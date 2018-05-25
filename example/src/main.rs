extern crate time_lock_puzzles;
extern crate num;

use std::collections::HashMap;

use time_lock_puzzles::*;

fn main() {
    let time_lock = 1_000_000;

    // Let's cteare 3 nodes of our net
    // We create a HashMap of id's and addresses
    let mut nodes = HashMap::<i32, String>::new();

    nodes.insert(1, "127.0.0.1:7878".to_string());
    nodes.insert(2, "127.0.0.1:7879".to_string());
    nodes.insert(3, "127.0.0.1:7880".to_string());

    // Now we create three TlpServer entities
    let mut first = TlpServer::new(1, "127.0.0.1:7878".to_string(), time_lock, nodes.clone());
    let mut second = TlpServer::new(2, "127.0.0.1:7879".to_string(), time_lock, nodes.clone());
    let mut third = TlpServer::new(3, "127.0.0.1:7880".to_string(), time_lock, nodes.clone());

    // We spawn three threads and run servers on them
    let one = std::thread::spawn(move || {first.start()});
    let two = std::thread::spawn(move || {second.start()});
    let three = std::thread::spawn(move || {third.start()});

    // Now let there be a client with index 0
    let mut client = TlpClient::new(0, nodes);

    // Client requests puzzle 
    client.request();

    // Client solves puzzles (this takes lime-lock squarings)
    client.solve();
    
    // Client sends solutions to nodes
    client.send();

    // After verification nodes send messages to each other and, after all of them
    // receive positive verdict from all other nodes, they write Client action to
    // blockchain (there is no blockchain here, it is only marked with console message)

    // This programm will not finish by itself (servers keep running)
    // So it needs to be manually shut down

    // These lines will not be reached if there is no error in threads
    one.join().unwrap();
    two.join().unwrap();
    three.join().unwrap();
}
