#include <stdio.h>

int factorial(int n) {
    int fact = 1;
    for (int i = 1; i <= n; i++) {
        fact *= i;
    }
    return fact;
}

int choose_upper_limit(int max) {
    int n = 0;
    while (factorial(n) < max) {
        n++;
    }
    if (factorial(n) == max) {
        return n;
    }
    else {
        return n - 1;
    }
}

int smallest_box_amount(int n, int result[][3]) {
    int nb_petits_pois[100][3];
    int nb_pois = 0;

    int csum = 0;
    int ca = 1;
    int cf = choose_upper_limit(n);

    if (cf == n) {
        nb_petits_pois[nb_pois][0] = 1;
        nb_petits_pois[nb_pois][1] = cf;
        nb_petits_pois[nb_pois][2] = factorial(cf);
        nb_pois += 1;

        for (int f = 0; f < cf - 1; f++) {
            nb_petits_pois[nb_pois][0] = 0;
            nb_petits_pois[nb_pois][1] = f;
            nb_petits_pois[nb_pois][2] = 0;
            nb_pois += 1;
        }

        for (int i = nb_pois - 1; i >= 0; i--) {
            result[nb_pois - 1 - i][0] = nb_petits_pois[i][0];
            result[nb_pois - 1 - i][1] = nb_petits_pois[i][1];
            result[nb_pois - 1 - i][2] = nb_petits_pois[i][2];
        }

        return nb_pois;
    }

    while (1) {
        int factorial_cf = factorial(cf);
        int cmp_res = csum + ca * factorial_cf - n;
        if (cmp_res < 0) {
            ca += 1;
        }
        else if (cmp_res == 0) {
            nb_petits_pois[nb_pois][0] = ca;
            nb_petits_pois[nb_pois][1] = cf;
            nb_petits_pois[nb_pois][2] = factorial_cf;
            nb_pois += 1;

            for (int f = 0; f < cf - 1; f++) {
                nb_petits_pois[nb_pois][0] = 0;
                nb_petits_pois[nb_pois][1] = f;
                nb_petits_pois[nb_pois][2] = 0;
                nb_pois += 1;
            }

            break;
        }
        else {
            nb_petits_pois[nb_pois][0] = ca - 1;
            nb_petits_pois[nb_pois][1] = cf;
            nb_petits_pois[nb_pois][2] = factorial_cf;
            nb_pois += 1;
            csum += (ca - 1) * factorial_cf;
            ca = 1;
            cf -= 1;
        }
    }

    for (int i = nb_pois - 1; i >= 0; i--) {
        result[nb_pois - 1 - i][0] = nb_petits_pois[i][0];
        result[nb_pois - 1 - i][1] = nb_petits_pois[i][1];
        result[nb_pois - 1 - i][2] = nb_petits_pois[i][2];
    }

    return nb_pois;
}

int get_input() {
    int num;
    scanf("%d", &num);
    return num;
}

int main() {
    int result[100][3];
    int n = get_input();
    int nb_pois = smallest_box_amount(n, result);

    printf("%d\n", result[nb_pois - 1][1]);

    for (int i = 0; i < nb_pois; i++) {
        printf("%d ", result[i][0]);
    }
    printf("\n");

    return 0;
}