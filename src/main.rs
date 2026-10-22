use std::fs;

struct SizeStats {
    //Structure for holding all values needed for end proof.
    size: u64,
    programs: u64,
    bookkeeping_bits: u64,
    unused_bits: u64,
    total_penalty: u64,
}

fn main() {
    let (data, max_request) = read_data();
    let num_days = 32.0;
    let optimal = solve(&data, max_request);

    println!("Optimal sizes: {}", optimal.len()); // Answer: 1
    for s in &optimal {
        println!("Page size (bits):  {}", s.size); // Answer: 32768
        println!("Programs:          {}", s.programs); // Answer: 4096
        println!("Bookkeeping bits:  {}", s.bookkeeping_bits); // Answer: 67239936
        println!("Unused bits:       {}", s.unused_bits); // Answer: 34305503
        println!("Total penalty:     {}", s.total_penalty); // Answer: 101545439
        println!("Average score:     {:.4}", s.total_penalty as f64 / num_days); // Answer: 3173294.9688
        println!();
    }
}

fn read_data() -> (Vec<Vec<u64>>, u64) {
    //main function to read data from files
    let mut data = Vec::new();

    //necessary for uppper bounding our summation
    let mut max_request: u64 = 0;
    for day in 1..=32 {
        let string_data =
            fs::read_to_string(format!("workloads/day_{day:02}/loads")).unwrap();
        let (day_data, curr_max) = parse(&string_data);
        data.push(day_data);
        if curr_max > max_request {max_request = curr_max;}
    }
    (data, max_request)
}

fn parse(string_data: &str) -> (Vec<u64>, u64) {
    //helper function to parse through a string and seperate the integers into a vector
    let mut requests: Vec<u64> = Vec::new();
    let mut max_request: u64 = 0;
    for line in string_data.lines() {
        if line.is_empty() {continue;}
        let curr_request: u64 = line.parse().unwrap();
        requests.push(curr_request);
        if curr_request > max_request {max_request = curr_request;}
    }
    (requests, max_request)
}

fn evaluate(size: u64, data: &[Vec<u64>]) -> SizeStats {
    //helper function for solve that acts as our summation
    let mut programs = 0;
    let mut bookkeeping_bits = 0;
    let mut unused_bits = 0;
    for day in data {
        for &request in day {
            let pages = request.div_ceil(size);
            programs += 1;
            bookkeeping_bits += pages * 64;
            unused_bits += pages * size - request;
        }
    }
    SizeStats {
        size,
        programs,
        bookkeeping_bits,
        unused_bits,
        total_penalty: bookkeeping_bits + unused_bits,
    }
}

fn solve(data: &[Vec<u64>], max_request: u64) -> Vec<SizeStats> {
    //main solve function which returns the optimal sizes and values associated
    let mut optimal_sizes: Vec<SizeStats> = Vec::new();
    let mut least_penalty = u64::MAX;

    for size in 1..=max_request {
        let stats = evaluate(size, data);

        if stats.total_penalty < least_penalty {
            least_penalty = stats.total_penalty;
            optimal_sizes.clear();
            optimal_sizes.push(stats);
        } 
        else if stats.total_penalty == least_penalty {optimal_sizes.push(stats);}
    }
    optimal_sizes
}

