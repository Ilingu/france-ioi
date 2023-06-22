class BookState:
    Available = 0
    NotAvailable = 1


def main():
    first_line = input()
    first_line_datas = first_line.split()
    nb_jours = int(first_line_datas[1])

    results = []

    books_state = {}
    for day_id in range(nb_jours):
        nb_clients = int(input())
        for _ in range(nb_clients):
            client_req = input()
            client_req_datas = client_req.split()

            book_id = int(client_req_datas[0])
            duration = int(client_req_datas[1])

            state, next_available_day = books_state.get(book_id, (BookState.Available, 0))

            if state == BookState.Available:
                books_state[book_id] = (BookState.NotAvailable, day_id + duration)
                is_available = True
            else:
                if day_id >= next_available_day:
                    books_state[book_id] = (BookState.NotAvailable, day_id + duration)
                    is_available = True
                else:
                    is_available = False

            results.append("1" if is_available else "0")

    print("\n".join(results))

if __name__ == "__main__":
    main()