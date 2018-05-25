extern crate num;
extern crate rand;

use std::collections::HashMap;
use std::collections::HashSet;
use std::clone::Clone;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

//use rand::Rng;

use num::Num;
use num::bigint::RandBigInt;
//use num::bigint::ToBigUint;
use num::One;
use num::Zero;

#[derive(Clone)]
pub struct TimeLockPuzzle {
    pub n: num::BigUint, // n = p * q, where p and q are primes
    pub a: num::BigUint, // Number to square
    pub t: num::BigUint, // Number of squarings
}

// puzzle with answers
pub struct UnlockedPuzzle {
    pub puzzle: TimeLockPuzzle,
    pub p: num::BigUint, // prime derivative of n
    pub q: num::BigUint, // prime derivative of n
    pub key: num::BigUint, // answer to the puzzle: a**t mod n
}

pub struct PuzzleGenerator {
    bit_size: usize, // Bitsize of p and q
}

impl PuzzleGenerator {
    pub fn new() -> PuzzleGenerator {
        PuzzleGenerator {
            bit_size: 256,
        }
    }

    pub fn with_bitsize(bit_size: usize) -> PuzzleGenerator {
        PuzzleGenerator {
            bit_size,
        }
    }

    // time - number of squarings we want client to perform
    pub fn gen_puzzle<U>(&self, time: U) -> UnlockedPuzzle
        where U:  num::bigint::ToBigUint
    {
        let t = time.to_biguint().unwrap();
        let p = gen_pseudo_prime(self.bit_size);
        let q = gen_pseudo_prime(self.bit_size);
        let n = &p * &q;
        let one = num::BigUint::one();
        let phi = (&p - &one) * (&q - &one);

        let mut rng = rand::thread_rng();

        let a = rng.gen_biguint_below(&n);
        let two = &one + &one;
        let e = two.modpow(&t, &phi);
        let key = a.modpow(&e, &n);
        UnlockedPuzzle {
            puzzle : TimeLockPuzzle {n, a, t},
            p,
            q,
            key,
        }
    }
}

// Generates pseudo-prime number with given bit size and number of check loops
// May be improved later with Miller-Rabin test
fn gen_pseudo_prime(bit_size: usize) -> num::BigUint {
    let mut rng = rand::thread_rng();
    let zero = num::BigUint::zero();
    let one = num::BigUint::one(); 
    let two = &one + &one;

    let small_primes : [u32; 999] = [3,5,7,11,13,17,19,23,29,31,37,41,43,47,53,59,61,67,71,73,79,83,89,97,101,103,107,109,113,127,131,137,139,149,151,157,163,167,173,179,181,191,193,197,199,211,223,227,229,233,239,241,251,257,263,269,271,277,281,283,293,307,311,313,317,331,337,347,349,353,359,367,373,379,383,389,397,401,409,419,421,431,433,439,443,449,457,461,463,467,479,487,491,499,503,509,521,523,541,547,557,563,569,571,577,587,593,599,601,607,613,617,619,631,641,643,647,653,659,661,673,677,683,691,701,709,719,727,733,739,743,751,757,761,769,773,787,797,809,811,821,823,827,829,839,853,857,859,863,877,881,883,887,907,911,919,929,937,941,947,953,967,971,977,983,991,997,1009,1013,1019,1021,1031,1033,1039,1049,1051,1061,1063,1069,1087,1091,1093,1097,1103,1109,1117,1123,1129,1151,1153,1163,1171,1181,1187,1193,1201,1213,1217,1223,1229,1231,1237,1249,1259,1277,1279,1283,1289,1291,1297,1301,1303,1307,1319,1321,1327,1361,1367,1373,1381,1399,1409,1423,1427,1429,1433,1439,1447,1451,1453,1459,1471,1481,1483,1487,1489,1493,1499,1511,1523,1531,1543,1549,1553,1559,1567,1571,1579,1583,1597,1601,1607,1609,1613,1619,1621,1627,1637,1657,1663,1667,1669,1693,1697,1699,1709,1721,1723,1733,1741,1747,1753,1759,1777,1783,1787,1789,1801,1811,1823,1831,1847,1861,1867,1871,1873,1877,1879,1889,1901,1907,1913,1931,1933,1949,1951,1973,1979,1987,1993,1997,1999,2003,2011,2017,2027,2029,2039,2053,2063,2069,2081,2083,2087,2089,2099,2111,2113,2129,2131,2137,2141,2143,2153,2161,2179,2203,2207,2213,2221,2237,2239,2243,2251,2267,2269,2273,2281,2287,2293,2297,2309,2311,2333,2339,2341,2347,2351,2357,2371,2377,2381,2383,2389,2393,2399,2411,2417,2423,2437,2441,2447,2459,2467,2473,2477,2503,2521,2531,2539,2543,2549,2551,2557,2579,2591,2593,2609,2617,2621,2633,2647,2657,2659,2663,2671,2677,2683,2687,2689,2693,2699,2707,2711,2713,2719,2729,2731,2741,2749,2753,2767,2777,2789,2791,2797,2801,2803,2819,2833,2837,2843,2851,2857,2861,2879,2887,2897,2903,2909,2917,2927,2939,2953,2957,2963,2969,2971,2999,3001,3011,3019,3023,3037,3041,3049,3061,3067,3079,3083,3089,3109,3119,3121,3137,3163,3167,3169,3181,3187,3191,3203,3209,3217,3221,3229,3251,3253,3257,3259,3271,3299,3301,3307,3313,3319,3323,3329,3331,3343,3347,3359,3361,3371,3373,3389,3391,3407,3413,3433,3449,3457,3461,3463,3467,3469,3491,3499,3511,3517,3527,3529,3533,3539,3541,3547,3557,3559,3571,3581,3583,3593,3607,3613,3617,3623,3631,3637,3643,3659,3671,3673,3677,3691,3697,3701,3709,3719,3727,3733,3739,3761,3767,3769,3779,3793,3797,3803,3821,3823,3833,3847,3851,3853,3863,3877,3881,3889,3907,3911,3917,3919,3923,3929,3931,3943,3947,3967,3989,4001,4003,4007,4013,4019,4021,4027,4049,4051,4057,4073,4079,4091,4093,4099,4111,4127,4129,4133,4139,4153,4157,4159,4177,4201,4211,4217,4219,4229,4231,4241,4243,4253,4259,4261,4271,4273,4283,4289,4297,4327,4337,4339,4349,4357,4363,4373,4391,4397,4409,4421,4423,4441,4447,4451,4457,4463,4481,4483,4493,4507,4513,4517,4519,4523,4547,4549,4561,4567,4583,4591,4597,4603,4621,4637,4639,4643,4649,4651,4657,4663,4673,4679,4691,4703,4721,4723,4729,4733,4751,4759,4783,4787,4789,4793,4799,4801,4813,4817,4831,4861,4871,4877,4889,4903,4909,4919,4931,4933,4937,4943,4951,4957,4967,4969,4973,4987,4993,4999,5003,5009,5011,5021,5023,5039,5051,5059,5077,5081,5087,5099,5101,5107,5113,5119,5147,5153,5167,5171,5179,5189,5197,5209,5227,5231,5233,5237,5261,5273,5279,5281,5297,5303,5309,5323,5333,5347,5351,5381,5387,5393,5399,5407,5413,5417,5419,5431,5437,5441,5443,5449,5471,5477,5479,5483,5501,5503,5507,5519,5521,5527,5531,5557,5563,5569,5573,5581,5591,5623,5639,5641,5647,5651,5653,5657,5659,5669,5683,5689,5693,5701,5711,5717,5737,5741,5743,5749,5779,5783,5791,5801,5807,5813,5821,5827,5839,5843,5849,5851,5857,5861,5867,5869,5879,5881,5897,5903,5923,5927,5939,5953,5981,5987,6007,6011,6029,6037,6043,6047,6053,6067,6073,6079,6089,6091,6101,6113,6121,6131,6133,6143,6151,6163,6173,6197,6199,6203,6211,6217,6221,6229,6247,6257,6263,6269,6271,6277,6287,6299,6301,6311,6317,6323,6329,6337,6343,6353,6359,6361,6367,6373,6379,6389,6397,6421,6427,6449,6451,6469,6473,6481,6491,6521,6529,6547,6551,6553,6563,6569,6571,6577,6581,6599,6607,6619,6637,6653,6659,6661,6673,6679,6689,6691,6701,6703,6709,6719,6733,6737,6761,6763,6779,6781,6791,6793,6803,6823,6827,6829,6833,6841,6857,6863,6869,6871,6883,6899,6907,6911,6917,6947,6949,6959,6961,6967,6971,6977,6983,6991,6997,7001,7013,7019,7027,7039,7043,7057,7069,7079,7103,7109,7121,7127,7129,7151,7159,7177,7187,7193,7207,7211,7213,7219,7229,7237,7243,7247,7253,7283,7297,7307,7309,7321,7331,7333,7349,7351,7369,7393,7411,7417,7433,7451,7457,7459,7477,7481,7487,7489,7499,7507,7517,7523,7529,7537,7541,7547,7549,7559,7561,7573,7577,7583,7589,7591,7603,7607,7621,7639,7643,7649,7669,7673,7681,7687,7691,7699,7703,7717,7723,7727,7741,7753,7757,7759,7789,7793,7817,7823,7829,7841,7853,7867,7873,7877,7879,7883,7901,7907,7919];

    loop {
        let mut seems_prime = true;
        // We set first and last bit to 1
        let candidate = rng.gen_biguint(bit_size)
                        | &one
                        | &one << (bit_size - 1);
        // First we check some small primes
        for small_prime in small_primes.into_iter() {
            if (&candidate % small_prime) == zero {
                seems_prime = false;
                break;
            }
        }

        // Now we use Fermat's teorem
        if seems_prime {
            // let a = rng.gen_biguint(bit_size);
            let test = &two.modpow(&(&candidate - 1 as u32), &candidate);
            if test != &one {
                seems_prime = false;
            }
        }

        // Miller-Rabin test
        // If number of rounds is 48, the probabulity of mistake is less than 2^-96
        if seems_prime {
            let k: usize = 48;
            let mut t = &candidate - &one;
            let mut s: usize = 0;
            while &t & &one == zero {
                s += 1;
                t >>= 1;
            }

            let n_1 = &candidate - &one;
            'A: for _ in 1..k+1 {
                if !seems_prime {
                    break
                }
                let a = rng.gen_biguint(bit_size);
                let mut x = a.modpow(&t, &candidate);
                if x == one || x == n_1 {
                    continue
                }

                for __ in 1..s {
                    x = (&x * &x) % &candidate;
                    if x == one {
                        break
                    } else if x == &candidate - &one {
                        continue 'A
                    }
                }

                seems_prime = false;
                break
            }
        }
        

        if seems_prime {
            return candidate;
        }
    }
}

impl TimeLockPuzzle {
    pub fn solve(&self) -> num::BigUint {
        let (n, mut a, mut t) = (self.n.clone(), self.a.clone(), self.t.clone());
        let zero = num::BigUint::zero();
        let one = num::BigUint::one();

        while t != zero {
            t -= &one;
            a = (&a * &a) % &n;
        }

        a
    }

    pub fn solve_with_a(&self, mut a: num::BigUint) -> num::BigUint {
        let (n, mut t) = (self.n.clone(), self.t.clone());
        let zero = num::BigUint::zero();
        let one = num::BigUint::one();

        while t != zero {
            t -= &one;
            a = (&a * &a) % &n;
        }

        a
    }

    // returns string ""n a t"
    pub fn stringify(&self) -> String {
        format!
        (
            "{} {} {}"
            , &self.n.to_str_radix(10)
            , &self.a.to_str_radix(10)
            , &self.t.to_str_radix(10)
        )
    }
}

// Entity who provides the puzzles and validates the solutions
pub struct Auditor {
    pub id: i32,
    time_lock: num::BigUint,
    generator: PuzzleGenerator,
    requests: HashMap<i32, UnlockedPuzzle>,  // множество выданных загадок
}

impl Auditor {
    pub fn new<U>(id: i32, tl: U) -> Auditor
        where U: num::bigint::ToBigUint
    {
        Auditor {
            id,
            time_lock: tl.to_biguint().unwrap(),
            generator: PuzzleGenerator::new(),
            requests: HashMap::new(),
        }
    }

    // return puzzle and remember the solver
    pub fn serve_puzzle(&mut self, solver_id: i32, number_of_auditors: usize) -> TimeLockPuzzle {
        let unlocked = self.generator.gen_puzzle(self.time_lock.clone() / number_of_auditors);
        let puzzle = unlocked.puzzle.clone();

        self.requests.insert(solver_id, unlocked);

        puzzle
    }

    pub fn verify(&mut self, solver_id: i32, solutions: &Vec<(i32, num::BigUint)>) -> bool {
        let unblocked = self.requests.remove(&solver_id).unwrap();
        let puzzle = unblocked.puzzle;

        let mut input = num::BigUint::zero();
        for (id, solution) in solutions.iter() {
            if *id == self.id {
                if input == num::BigUint::zero() {
                    return *solution == unblocked.key; 
                }

                let (t, n) = (&puzzle.t, &puzzle.n);
                let (p, q) = (&unblocked.p, &unblocked.q);

                let one = num::BigUint::one();
                let two = &one + &one;

                let phi = (p - &one) * (q - &one);
                let e = two.modpow(t, &phi);
                let true_key = input.modpow(&e, n);

                return *solution == true_key

            }

            input = solution.clone();
        }

        false
    }
}

// Entity who solves puzzles
pub struct Solver {
    pub id: i32,
}

impl Solver {
    pub fn new(id: i32) -> Solver {
        Solver {
           id
        }
    }

    pub fn solve(&self, puzzles: &HashMap<i32, TimeLockPuzzle>) -> Vec<(i32, num::BigUint)> {
        let mut solutions: Vec<(i32, num::BigUint)> = Vec::new();

        let mut a = num::BigUint::zero();
        for (&id, puzzle) in puzzles {
            if solutions.is_empty() {
                a = puzzle.a.clone();
            }

            let solution = if solutions.is_empty() {
                puzzle.solve()
            } else {
                puzzle.solve_with_a(a.clone())
            };

            solutions.push((id, solution.clone()));
            a = solution;
        }

        solutions
    }

    // returns string "id id:solution id:solution ..."
    pub fn solutions_stringify(&self, solutions: Vec<(i32, num::BigUint)>) -> String {
        let mut result = self.id.to_string();

        for (id, solution) in solutions.iter() {
            result += &format!(" {}:{}", id.to_string(), solution.to_str_radix(10));
        }

        result
    }
}

pub struct TlpClient {
    solver: Solver,
    nodes: HashMap<i32, String>,
    puzzles: HashMap<i32, TimeLockPuzzle>,
    solution_str: Option<String>,
}

impl TlpClient {
    pub fn new(id: i32, nodes: HashMap<i32, String>) -> TlpClient {
        TlpClient {
            nodes,
            solver: Solver::new(id), 
            puzzles: HashMap::new(),
            solution_str: None,
        }
    }

    pub fn request(&mut self) {
        for (node_id, addr) in &self.nodes {
            let mut puzzle_stream = TcpStream::connect(&addr);
            loop {
                match puzzle_stream {
                    Ok(_) => { break },
                    _ => {
                        println!("Client {} wasn't able to connect, retry", self.solver.id);
                        puzzle_stream = TcpStream::connect(&addr);
                    },
                }
            };

            let mut puzzle_stream = puzzle_stream.unwrap();

            let message = format!("[PUZ] {}", self.solver.id);
            write_message(&mut puzzle_stream, message);

            println!("Client {} send the request to server {}", self.solver.id, node_id);

            let mut puzzle_str = receive_message(&mut puzzle_stream);

            println!("Client {} received the answer from server {}", self.solver.id, node_id);

            let mut iter = puzzle_str
                            .split_whitespace()
                            .map(|x| num::BigUint::from_str_radix(x, 10).unwrap());
            let (n, a, t)  = (iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap());

            self.puzzles.insert(*node_id, TimeLockPuzzle {n, a, t});
        }

        println!("CLIENT RECEIVED ALL PUZZLES");
    }

    pub fn solve(&mut self) {
        let solutions = self.solver.solve(&self.puzzles);

        self.solution_str = Some(self.solver.solutions_stringify(solutions));

        println!("SOLVER SOLVED ALL PUZZLES");
    }

    pub fn send(&self) {
        for addr in self.nodes.values() {
            let mut solution_stream = TcpStream::connect(&addr).unwrap();
            
            match &self.solution_str {
                Some(ref sol_str) => {
                    let message = format!("[VER] {}", &sol_str);
                    write_message(&mut solution_stream, message);
                },
                _ => {println!("Solution string did non match")}
            }
        }
    }
}

pub struct TlpServer {
    addr: String, // Web address
    auditor: Auditor,
    nodes: HashMap<i32, String>, // addresses of nodes
    solvers: HashMap<i32, HashSet<i32>>, // Set of nodes, who yet haven't verified solver
}

impl TlpServer {
    pub fn new<U>(id: i32, addr: String, tl: U, nodes: HashMap<i32, String>) -> TlpServer
        where U: num::bigint::ToBigUint
    {
        TlpServer {
            addr,
            auditor: Auditor::new(id, tl),
            nodes,
            solvers: HashMap::<i32, HashSet<i32>>::new(),
        }
    }

    pub fn start(&mut self) {
        println!("Server {} is trying to bind to {}", self.auditor.id, self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        println!("SERVER {} IS RUNNING", self.auditor.id);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    self.handle(stream); 
                },
                _ => {println!("Failed to connect to {}", self.auditor.id)}
            }
        }
        
    }

    pub fn handle(&mut self, mut stream: TcpStream) {
        let message: String = receive_message(&mut stream);
        let message: Vec<&str> = message.split_whitespace().collect();

        //let message: Vec<&str> = message.split_whitespace().collect();

        // [NEW] node_id            - message from new node
        // [PUZ] id                 - request for puzzle
        // [VER] id id:solution ... - sequest for verification
        // [COR] id node_id         - message from other node, verifying correctness of id's solution

        match message[0] as &str {
            "[NEW]" => {
               // TODO? 
            },

            "[PUZ]" => {
                let id = i32::from_str_radix(message[1], 10).unwrap();
                let puzzle_str = self.auditor.serve_puzzle(id, self.nodes.len()).stringify();

                write_message(&mut stream, puzzle_str);

                let mut set = HashSet::<i32>::new();
                for node in self.nodes.keys() {
                    set.insert(*node);
                }
                self.solvers.insert(id, set);
            },

            "[VER]" => {
                let id = i32::from_str_radix(message[1], 10).unwrap();

                let mut solutions = Vec::<(i32, num::BigUint)>::new();
                
                for pair in message.iter().skip(2) {
                    let pair: Vec<&str> = pair.split(":").collect();
                    solutions.push(
                        (
                            i32::from_str_radix(pair[0], 10).unwrap(),
                            num::BigUint::from_str_radix(pair[1], 10).unwrap()
                        )
                    );
                }

                if self.auditor.verify(id, &solutions) {
                    println!("Server {} verified solution of Client {}", self.auditor.id, &id);

                    self.solvers.get_mut(&id).unwrap().remove(&self.auditor.id);

                    self.send_ok(&id);
                } else {
                    self.solvers.remove(&id);
                    // TODO Send not ok to everyone
                }

            },

            "[COR]" => {
                let id = i32::from_str_radix(message[1], 10).unwrap();
                let node_id = i32::from_str_radix(message[2], 10).unwrap();

                println!(
                    "Server {} received comfirmation from server {} about Client {}"
                    , self.auditor.id
                    , node_id
                    , id
                );
                
                match self.solvers.get_mut(&id) {
                    Some(set) => {set.remove(&node_id);},
                    _ => { println!("get_mut error") },
                }

                if self.solvers[&id].is_empty() {
                    println!(
                        "SERVER {} ADDED REQUEST OF SOLVER {} TO BLOCKCHAIN"
                        , self.auditor.id
                        , id
                    );
                }
                    
            },
            _ => {}
        }
    }

    pub fn send_ok(&self, id: &i32) {
        for (node_id, addr) in &self.nodes {
            if *node_id != self.auditor.id {
                let mut ok_stream = TcpStream::connect(addr).unwrap();

                write_message(&mut ok_stream, format!("[COR] {} {}", id, self.auditor.id));
            }
        }
    }
}

fn receive_message(stream: &mut TcpStream) -> String {
    let mut len = [0 as u8; 4];

    stream.read_exact(&mut len).unwrap();
    let len = idiotic_u8_to_u32(len);

    let mut buff = vec![0 as u8; len as usize];
    stream.read_exact(&mut buff).unwrap();

    let message = String::from_utf8(buff).unwrap();

    message
}

fn write_message(stream: &mut TcpStream, message: String) {
   let len: u32 = message.len() as u32;
   let len = idiotic_u32_to_u8(len);

   stream.write(&len).unwrap();
   stream.write(message.as_bytes()).unwrap();
}

fn idiotic_u32_to_u8(mut n: u32) -> [u8; 4] {
    let mut res = [0 as u8; 4];
    for i in 0..4 {
        res[i] = (n % 256) as u8;
        n /= 256;
    }

    res
}

fn idiotic_u8_to_u32(arr: [u8; 4]) -> u32 {
    let mut res: u32 = 0;
    let mut mull: u32 = 1;
    for i in 0..4 {
        res += (arr[i] as u32) * mull;
        if i != 3 {
            mull *= 256;
        }
    }

    res
}
