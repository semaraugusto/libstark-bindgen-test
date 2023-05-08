pub fn parse_assignment_str(assignments_str: &str, ncols: usize) -> Vec<Vec<u64>> {
    let assignments = assignments_str
        .split_terminator("\n")
        .collect::<Vec<&str>>();

    println!("assignments_str: -{}-", assignments_str);
    assignments
        .chunks(ncols)
        .map(|matrix_line| {
            let row = matrix_line.iter().fold(vec![], |mut acc, column_str| {
                // let line = matrix_line[i];
                let column_str = column_str
                    .replace(" ", "")
                    .replace("[", "")
                    .replace("]", "");
                // let column_str = column_str.replace("[", "");
                // let column_str = column_str.replace("]", "");
                let val = u64::from_str_radix(&column_str, 2).unwrap();
                acc.push(val);
                acc
            });
            row
        })
        .collect()
}

pub fn parse_permutation_str(permutation_str: &str) -> Vec<usize> {
    permutation_str
        .split_terminator(',')
        .fold(vec![], |mut acc, numb_str| {
            // println!("numb: -{}-", numb_str.trim());
            acc.push(usize::from_str_radix(numb_str.trim(), 10).unwrap());
            acc
        })
}
