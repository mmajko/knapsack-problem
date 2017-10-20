use parser::Knapsack;
use parser::KnapItem;
use time::PreciseTime;
use std::u32;

#[derive(Debug, Clone, Copy)]
pub enum SolutionType {
    Bruteforce,
    Heuristic,
}

#[derive(Debug, Clone, Copy)]
pub struct KnapSolution {
    pub knap_id: u16,
    pub bitmask: u32,
    pub price: u16,
    pub weight: u16,
    pub elapsed: f32,
    pub soltype: SolutionType,
}

/// Converts bitmask to vector of items. (bitmask is a presence mask)
fn items_from_bmask(knap: &Knapsack, bitmask: u32) -> Vec<&KnapItem> {
    knap.items.iter().enumerate().filter(|item| {
        bitmask & (1 << item.0) != 0
    }).map(|item| item.1).collect()
}

/// Converts a vector of items to a presence bitmask.
fn bitmask_from_items(items: Vec<&KnapItem>) -> u32 {
    items.iter().fold(0, |acc, ref item| {
        acc + (1 << item.id)
    })
}

/// Calculate a "fitness" of specified vector of items, returning a tuple
/// consisting of total items weight and total items price.
fn calc_fitness(items: Vec<&KnapItem>) -> (u16, u16) {
    items.iter().fold((0, 0), |acc, ref item| (acc.0 + item.weight, acc.1 + item.price))
}

fn solve_bruteforce(knap: &Knapsack) -> (u16, u16, u32) {
    let mut bitmask = 0;
    let items = items_from_bmask(knap, bitmask);
    let fitness = calc_fitness(items);
    let mut best = (fitness.0, fitness.1, 0);
    
    let max_bitmask = u32::pow(2, knap.items.len() as u32);
        
    while bitmask < max_bitmask {
        bitmask += 1;
        let items = items_from_bmask(knap, bitmask);
        let fitness = calc_fitness(items);
        
        if fitness.0 <= knap.capacity && fitness.1 > best.1 {
            best = (fitness.0, fitness.1, bitmask);
        }
    }
    
    best
}

fn solve_heuristic(knap: &Knapsack) -> (u16, u16, u32) {
    let mut items: Vec<(usize, &KnapItem)> = knap.items.iter().enumerate().collect();
    items.sort_unstable_by(|a, b| (a.1.price / a.1.weight).cmp(&(b.1.price / b.1.weight)));
    
    let mut result_items: Vec<&KnapItem> = vec![];
    let mut total_weight = 0;
    
    for item in items {
        if item.1.weight + total_weight <= knap.capacity {
            result_items.push(item.1);
            total_weight += item.1.weight;
        } else {
            let last_item = result_items.last().unwrap().clone();
            let total_weight_after = item.1.weight + total_weight - last_item.weight;
            if last_item.price < item.1.price && total_weight_after <= knap.capacity {
                result_items.pop();
                result_items.push(item.1);
                total_weight = total_weight_after;
            }
        }
    }
    
    let fitness = calc_fitness(result_items.clone());
    let bitmask = bitmask_from_items(result_items.clone());
    
    (fitness.0, fitness.1, bitmask)
}

/// Validates a solution. (items weight can't exceed knapsack capacity)
pub fn validate(solution: &KnapSolution, knap: &Knapsack) -> bool {
    solution.weight <= knap.capacity
}

/// Solves an instance of knapsack problem, returning a solution structure.
/// Types of solutions are Bruteforce and heuristic (sorted by price/weight ratio).
pub fn solve(knap: &Knapsack, soltype: SolutionType) -> KnapSolution {
    let start = PreciseTime::now();
    
    let (weight, price, bitmask) = match soltype {
        SolutionType::Bruteforce => solve_bruteforce(knap),
        SolutionType::Heuristic => solve_heuristic(knap),
    };
    
    let elapsed_t = start.to(PreciseTime::now());
    let elapsed: f32 = if elapsed_t.num_milliseconds() == 0 { elapsed_t.num_nanoseconds().unwrap() as f32 / 1000000.0 } else { elapsed_t.num_milliseconds() as f32 };
    
    KnapSolution {
        knap_id: knap.id,
        bitmask: bitmask,
        weight: weight,
        price: price,
        elapsed: elapsed,
        soltype: soltype,
    }
}