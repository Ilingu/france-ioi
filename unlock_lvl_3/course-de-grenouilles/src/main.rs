use std::{collections::HashMap, io};

fn main() {
    let _nb_frog = input().parse::<usize>().unwrap();
    let nb_tours = input().parse::<usize>().unwrap();

    let mut frog_total_distance = HashMap::new();
    let mut frog_lead_count = HashMap::new();

    for ti in 0..nb_tours {
        if ti != 0 {
            let max_distance_so_far = frog_total_distance.values().max().unwrap();
            let leads_frogs = frog_total_distance
                .iter()
                .filter(|(_, d)| d == &max_distance_so_far)
                .collect::<Vec<_>>();
            if leads_frogs.len() == 1 {
                frog_lead_count
                    .entry(*leads_frogs[0].0)
                    .and_modify(|lc| *lc += 1)
                    .or_insert(1_usize);
            }
        }

        let raw_tour = input();
        let mut tour_data = raw_tour.split_whitespace();

        let frog_id = tour_data.next().unwrap().parse::<usize>().unwrap();
        let distance = tour_data.next().unwrap().parse::<usize>().unwrap();

        frog_total_distance
            .entry(frog_id)
            .and_modify(|td| *td += distance)
            .or_insert(distance);
    }

    // let result = frog_lead_count
    //     .iter()
    //     .max_by(|fa: &(&usize, &usize), fb| match fa.1.cmp(fb.1) {
    //         Ordering::Less => Ordering::Less,
    //         Ordering::Equal => fb.0.cmp(fa.0),
    //         Ordering::Greater => Ordering::Greater,
    //     })
    //     .unwrap();

    let biggest_lead = frog_lead_count.values().max().unwrap_or(&0);
    let result = frog_lead_count
        .iter()
        .filter(|(_, lc)| lc == &biggest_lead)
        .min_by(|a, b| a.0.cmp(b.0))
        .unwrap_or((&1, &0));

    println!("{}", result.0)
}

fn input() -> String {
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .expect("failed to read from stdin");
    buf.trim().to_string()
}
