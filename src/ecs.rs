use sauss_cramer::{
    print_2_variables_result, print_3_variables_result, solve_2_variables, solve_3_variables,
};
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Result},
};
pub fn solve_ecs() -> Result<()> {
    println!("Escribe la direccion del archivo");
    let mut path = String::new();
    io::stdin().read_line(&mut path)?;
    let file = File::open(path.trim()).unwrap_or_else(|_| {
            eprintln!("No se pudo leer el archivo, asegurate de que exista o este en el mismo folder del programa!");
            std::process::exit(1);
        });

    let reader = BufReader::new(file);
    let mut numbers: Vec<f64> = Vec::new();
    for line in reader.lines() {
        let line: f64 = line.unwrap().trim().parse().unwrap_or_else(|_| {
            eprintln!("No se pudo procesar el archivo, asegurate de que solo sean numeros!");
            std::process::exit(1);
        });
        numbers.push(line);
    }
    if numbers[0] == 3.0 {
        let result = solve_3_variables(
            numbers[1],
            numbers[2],
            numbers[3],
            numbers[4],
            numbers[5],
            numbers[6],
            numbers[7],
            numbers[8],
            numbers[9],
            numbers[10],
            numbers[11],
            numbers[12],
        );
        print_3_variables_result(result);
    } else if numbers[0] == 2.0 {
        let result = solve_2_variables(
            numbers[1], numbers[2], numbers[3], numbers[4], numbers[5], numbers[6],
        );
        print_2_variables_result(result);
    }
    Ok(())
}
