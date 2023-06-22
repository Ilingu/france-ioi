const NB_LETTRES: usize = 26;
const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

fn main() {
    let mut lines: Vec<String> = vec![];
    for y in 0..NB_LETTRES {
        let mut first_part = String::new();
        for x in 0..(NB_LETTRES - 1) {
            let x = if x > y { y } else { x };
            first_part.push(ALPHABET.chars().nth(x).unwrap());
        }
        let last_part = first_part.chars().rev().collect::<String>();

        let line = format!(
            "{first_part}{}{last_part}",
            ALPHABET.chars().nth(y).unwrap()
        );
        lines.push(line)
    }

    let mut bottom_part = lines[..NB_LETTRES - 1]
        .iter()
        .rev()
        .map(|x| x.to_owned())
        .collect::<Vec<_>>();
    lines.append(&mut bottom_part);

    let rendered_target = lines.join("\n");
    println!("{rendered_target}")
}
