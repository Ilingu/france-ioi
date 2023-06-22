from collections import defaultdict

def main():
    nb_frog = int(input())
    nb_tours = int(input())

    frog_total_distance = defaultdict(int)
    frog_lead_count = defaultdict(int)

    for ti in range(nb_tours):
        if ti != 0:
            max_distance_so_far = max(frog_total_distance.values())
            leads_frogs = [k for k, v in frog_total_distance.items() if v == max_distance_so_far]
            if len(leads_frogs) == 1:
                frog_lead_count[leads_frogs[0]] += 1

        raw_tour = input()
        tour_data = raw_tour.split()

        frog_id = int(tour_data[0])
        distance = int(tour_data[1])

        frog_total_distance[frog_id] += distance

    biggest_lead = max(frog_lead_count.values(), default=0)
    result = min([k for k, v in frog_lead_count.items() if v == biggest_lead], default=1)

    print(result)

if __name__ == '__main__':
    main()