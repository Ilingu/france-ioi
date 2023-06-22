use std::{collections::HashMap, io};

enum BookState {
    Available,
    NotAvailable(usize),
}

fn main() {
    let first_line = input();
    let first_line_datas = first_line.split_whitespace().collect::<Vec<_>>();
    let nb_jours = first_line_datas[1].parse::<usize>().unwrap();

    let mut results = vec![];

    let mut books_state = HashMap::new();
    for day_id in 0..nb_jours {
        let nb_clients = input().parse::<usize>().unwrap();
        for _ in 0..nb_clients {
            let client_req = input();
            let client_req_datas = client_req.split_whitespace().collect::<Vec<_>>();

            let book_id = client_req_datas[0].parse::<usize>().unwrap();
            let duration = client_req_datas[1].parse::<usize>().unwrap();

            let state = books_state.get(&book_id).unwrap_or(&BookState::Available);
            let is_available = match state {
                BookState::Available => {
                    books_state.insert(book_id, BookState::NotAvailable(day_id + duration));
                    true
                }
                BookState::NotAvailable(next_available_day) => {
                    if &day_id >= next_available_day {
                        books_state.insert(book_id, BookState::NotAvailable(day_id + duration));
                        true
                    } else {
                        false
                    }
                }
            };

            results.push(if is_available { "1" } else { "0" });
        }
    }

    println!("{}", results.join("\n"))
}

fn input() -> String {
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .expect("failed to read from stdin");
    buf.trim().to_string()
}
