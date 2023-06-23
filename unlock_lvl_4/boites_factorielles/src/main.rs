use std::cmp::Ordering;

fn main() {
    let result = smallest_box_amount(52);
    let parsed_result = result
        .iter()
        .rev()
        .map(|(a_n, f, _)| (a_n, f))
        .collect::<Vec<_>>();
    println!("{}", parsed_result[parsed_result.len() - 1].1);
    println!(
        "{}",
        parsed_result
            .iter()
            .map(|(a_n, _)| a_n)
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}

fn smallest_box_amount(n: usize) -> Vec<(usize, usize, usize)> {
    let mut nb_petits_pois = vec![];
    let mut csum = 0;

    let (mut ca, mut cf) = (1, choose_upper_limit(n));
    if cf == n {
        nb_petits_pois.push((1, cf, factorial(cf)));
        (0..cf - 1).for_each(|f| nb_petits_pois.push((0, f, 0)));
        return nb_petits_pois;
    }

    loop {
        let factorial_cf = factorial(cf);
        match (csum + ca * factorial_cf).cmp(&n) {
            Ordering::Less => ca += 1,
            Ordering::Equal => {
                nb_petits_pois.push((ca, cf, factorial_cf));
                (0..cf - 1).for_each(|f| nb_petits_pois.push((0, f, 0)));
                break nb_petits_pois;
            }
            Ordering::Greater => {
                nb_petits_pois.push((ca - 1, cf, factorial_cf));
                csum += (ca - 1) * factorial_cf;
                ca = 1;
                cf -= 1;
            }
        }
    }
}

fn choose_upper_limit(max: usize) -> usize {
    let mut n = 0;
    loop {
        match factorial(n).cmp(&max) {
            Ordering::Equal => break n,
            Ordering::Greater => break n - 1,
            _ => (),
        }
        n += 1
    }
}

fn factorial(n: usize) -> usize {
    (1..=n).product()
}
