pub fn choisir_son_manteau(mut temperature_interval: Vec<(usize, usize)>) -> usize {
    temperature_interval.sort_by(|(inf_a, _), (inf_b, _)| inf_a.cmp(inf_b));

    let mut max_count = 0;
    for (i, (inf_a, sup_a)) in temperature_interval.iter().enumerate() {
        let mut sub_count = 0;
        for (j, (inf_b, sup_b)) in temperature_interval.iter().enumerate().rev() {
            if i == j {
                continue;
            }

            if inf_b < inf_a {
                break;
            }

            if inf_b >= inf_a && sup_b <= sup_a {
                sub_count += 1;
            }
        }
        max_count = max_count.max(sub_count)
    }
    max_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manteau() {
        let res = choisir_son_manteau(vec![
            (1, 3),
            (2, 5),
            (5, 8),
            (3, 6),
            (2, 5),
            (3, 8),
            (3, 6),
            (3, 8),
        ]);
        assert_eq!(res, 4)
    }
}
