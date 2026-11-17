fn main() {
    let int_list = vec![2, 24, 2, 4, 4, 5, 6 ,10, 13, 1];
    let mut sum = 0.0;
    let mut dif = 0.0;

    for num in &int_list {
        sum = *num as f64 + sum;
    }
    let prom = sum / int_list.len() as f64;
    println!("Promedio: {prom}");

    let mut dif_prom = Vec::new();
    for num in &int_list {
        dif = (*num as f64 - prom).abs();
        dif_prom.push(dif as usize);
    }

    println!("{dif_prom:?}");

    //let dif_prom_min = dif_prom.iter().copied().min().unwrap();

    let indice_min = dif_prom
        .iter()
        .enumerate()
        .min_by_key(|&(_, valor)| valor)
        .map(|(indice, _)| indice);

    let indice_min = match indice_min {
        Some(idx) => {idx}, // Imprime: 3
        None => {0},
    };
    let med = int_list.get(indice_min).unwrap();
    println!("{med}");

}
