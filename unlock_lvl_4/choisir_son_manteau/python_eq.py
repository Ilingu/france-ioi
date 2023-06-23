import sys

def main():
    nb_manteaux = int(input())

    temperature_interval = []
    for _ in range(nb_manteaux):
        datas_str = input()
        datas = datas_str.split()

        temp_inf = int(datas[0])
        temp_sup = int(datas[1])

        temperature_interval.append((temp_inf, temp_sup))

    print(choisir_son_manteau(temperature_interval))

def choisir_son_manteau(temperature_interval):
    temperature_interval.sort()

    max_count = 0
    for i, (inf_a, sup_a) in enumerate(temperature_interval):
        sub_count = 0
        for j, (inf_b, sup_b) in reversed(list(enumerate(temperature_interval))):
            if i == j:
                continue

            if inf_b < inf_a:
                break

            if inf_b >= inf_a and sup_b <= sup_a:
                sub_count += 1

        max_count = max(max_count, sub_count)

    return max_count

if __name__ == '__main__':
    main()