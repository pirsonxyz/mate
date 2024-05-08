use sauss_cramer::{print_2_variables_result, solve_2_variables};

fn homotesia(points: &mut [f64; 2], axis: &str, quantity: f64) {
    println!("Puntos originales: ({}, {})", points[0], points[1]);
    if axis == "origin" {
        points[0] = points[0] * quantity;
        points[1] = points[1] * quantity;
        println!("Puntos cambiados: ({}, {})", points[0], points[1]);
    } else {
        std::process::exit(1);
    }
}

fn reflexion(points: &mut [f64; 2], axis: &str) {
    println!("Puntos originales: ({}, {})", points[0], points[1]);
    if axis == "x".to_lowercase() {
        points[1] = points[1] * -1.0;
    } else if axis == "y".to_lowercase() {
        points[0] = points[0] * -1.0;
    } else if axis == "origin".to_lowercase() {
        points[0] = points[0] * -1.0;
        points[1] = points[1] * -1.0;
    } else if axis == "yx".to_lowercase() {
        let temp = points[0];
        points[0] = points[1];
        points[1] = temp;
    } else {
        println!("Eje invalido");
        std::process::exit(1);
    }

    println!("Puntos reflejados: ({}, {})", points[0], points[1]);
}

fn traslation(points: &mut [f64; 2], h: f64, k: f64) {
    println!("Puntos originales: ({}, {})", points[0], points[1]);
    points[0] += h;
    points[1] += k;
    println!("Puntos trasladados: ({}, {})", points[0], points[1]);
}
fn rotation(points: &mut [f64; 2], angle: f64) {
    let angle_d = angle.to_radians();
    let x = points[0];
    let y = points[1];
    println!("Puntos originales: ({}, {})", x, y);
    points[0] = x * angle_d.cos() - y * angle_d.sin();
    points[1] = x * angle_d.sin() + y * angle_d.cos();
    println!("Puntos rotados: ({}, {})", points[0], points[1]);
}
fn main() {
    let mut a = [-6.0, 3.0];
    let mut b = [-4.0, 5.0];
    let mut c = [0.0, 5.0];
    let mut d = [-2.0, 3.0];

    homotesia(&mut a, "origin", 2.0);
    homotesia(&mut b, "origin", 2.0);
    homotesia(&mut c, "origin", 2.0);
    homotesia(&mut d, "origin", 2.0);

    reflexion(&mut a, "x");
    reflexion(&mut b, "x");
    reflexion(&mut c, "x");
    reflexion(&mut d, "x");

    let result = solve_2_variables(1.0, 4.0, 1.0, 2.0, 27.0, 21.0);
    print_2_variables_result(result);
}
