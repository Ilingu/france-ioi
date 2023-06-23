use std::io;

use choisir_son_manteau::choisir_son_manteau;

fn main() {
    let nb_manteaux = input().parse::<usize>().unwrap();

    let mut temperature_interval = vec![];
    for _ in 0..nb_manteaux {
        let datas_str = input();
        let datas = datas_str.split_whitespace().collect::<Vec<_>>();

        let temp_inf = datas[0].parse::<usize>().unwrap();
        let temp_sup = datas[1].parse::<usize>().unwrap();

        temperature_interval.push((temp_inf, temp_sup))
    }

    println!("{}", choisir_son_manteau(temperature_interval))
}

fn input() -> String {
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .expect("failed to read from stdin");
    buf.trim().to_string()
}
