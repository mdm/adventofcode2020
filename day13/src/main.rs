use std::io::BufRead;
type BusId = i64;


#[derive(Debug,PartialEq)]
enum Bus {
    WithId(BusId),
    OutOfService,
}

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let file = std::fs::File::open(filename).unwrap();
    let reader = std::io::BufReader::new(file);

    let mut lines = reader.lines();
    let earliest_time = lines.next().unwrap().unwrap().parse::<BusId>().unwrap();
    let buses = lines.next().unwrap().unwrap().split(',').map(|bus| {
        if bus == "x" {
            Bus::OutOfService
        } else {
            let id = bus.parse::<BusId>().unwrap();
            Bus::WithId(id)
        }
    }).collect::<Vec<_>>();

    let mut departures1 = buses.iter().filter(|bus| {
        bus != &&Bus::OutOfService
    }).map(|bus| {
        match bus {
            Bus::WithId(id) => (id - (earliest_time % id), id),
            Bus::OutOfService => unreachable!(),
        }
    }).collect::<Vec<_>>();

    // dbg!(&departures1);
    departures1.sort();
    println!("{}", departures1[0].1 * departures1[0].0);

    let first_bus_id = match buses[0] {
        Bus::WithId(id) => id,
        Bus::OutOfService => unreachable!(),
    };
    let answer_part2 = buses.iter().enumerate().filter(|bus| {
        bus.1 != &Bus::OutOfService
    }).map(|bus| {
        match bus.1 {
            Bus::WithId(id) => (bus.0 as BusId, *id),
            Bus::OutOfService => unreachable!(),
        }
    }).fold((first_bus_id, 1), |answer, bus| {
        let mut answer = answer;
        let expected = (-bus.0).rem_euclid(bus.1);
        while answer.0 % bus.1 != expected {
            answer.0 += answer.1;
        }
        answer.1 *= bus.1;
        answer
    });

    println!("{}", answer_part2.0);
}

