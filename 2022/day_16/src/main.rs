use std::{
    collections::HashMap,
    io::{self, BufRead},
};

struct Valve {
    name: String,
    flow_rate: usize,
    tunnels: Vec<String>,
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|l| l.unwrap()).collect::<Vec<_>>();

    let valve_re =
        Regex::new(r"Valve (\w+) has flow rate=(\d+); tunnels lead to valves ((\w+|, \w+)+)")
            .unwrap();
    // Valve SY has flow rate=0; tunnels lead to valves GW, LW

    let mut valves = HashMap::new();

    for line in lines {
        let captures = valve_re.captures(&line).unwrap();
        let name = captures[1].to_string();
        let flow_rate = captures[2].parse::<usize>().unwrap();
        let tunnels = captures[3]
            .split(", ")
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        let valve = Valve {
            name,
            flow_rate,
            tunnels,
        };

        valves.insert(valve.name.clone(), valve);
    }

    

}
