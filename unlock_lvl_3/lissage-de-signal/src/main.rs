use std::fs;

fn main() {
    let input = fs::read_to_string("./src/baseinput.txt").unwrap();
    let mut datas = input.lines();

    let diff_max = datas.nth(1).unwrap().parse::<f64>().unwrap();
    let mesures = datas
        .clone()
        .map(|v| v.parse::<f64>().unwrap())
        .collect::<Vec<_>>();
    drop(datas); // to not use it again, because it's now empty

    let mut nb_of_smooth = 0;
    let mut smoothed_mesures = mesures;
    while !is_lissage_ok(&smoothed_mesures, diff_max) {
        nb_of_smooth += 1;
        smoothed_mesures = smoothed_mesures
            .iter()
            .enumerate()
            .map(|(i, mesure)| {
                if i == 0 || i == smoothed_mesures.len() - 1 {
                    return *mesure;
                }

                let (prev, next) = (smoothed_mesures.get(i - 1), smoothed_mesures.get(i + 1));
                if prev.is_none() || next.is_none() {
                    return *mesure;
                }

                let (prev, next) = (prev.unwrap(), next.unwrap());
                (prev + next) / 2.0
            })
            .collect()
    }
    println!("{nb_of_smooth}")
}

fn is_lissage_ok(mesures: &Vec<f64>, diff_max: f64) -> bool {
    for i in 0..(mesures.len() - 1) {
        if (mesures[i] - mesures[i + 1]).abs() > diff_max {
            return false;
        }
    }
    true
}
