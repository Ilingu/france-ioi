def main():
    datas = []
    try:
      while True:
          last_input = input()
          datas.append(last_input)
    except EOFError:
      pass

    diff_max = float(datas[1])
    mesures = [float(v) for v in datas[2:]]

    nb_of_smooth = 0
    smoothed_mesures = mesures
    while not is_lissage_ok(smoothed_mesures, diff_max):
        nb_of_smooth += 1
        smoothed_mesures = [
            (smoothed_mesures[i - 1] + smoothed_mesures[i + 1]) / 2.0 if i != 0 and i != len(smoothed_mesures) - 1 else smoothed_mesures[i]
            for i in range(len(smoothed_mesures))
        ]

    print(nb_of_smooth)

def is_lissage_ok(mesures, diff_max):
    for i in range(len(mesures) - 1):
        if abs(mesures[i] - mesures[i + 1]) > diff_max:
            return False
    return True

if __name__ == "__main__":
    main()