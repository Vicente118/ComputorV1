use std::env;

#[allow(unused_variables)]
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2
    {
        println!("Usage: ./computer \"EQUATION\"");
        return;
    }

    let input = &args[1];

    match parse_equation(input) {
        Ok((coef, max_degree)) => {
            print_reduced_form(&coef);
            solve_equation(&coef, max_degree);
        }

        Err(err) => {
            println!("Error: {}", err);
        }
    }
}


fn parse_equation(input: &str) -> Result<(Vec<f64>, usize), String> {
    let sides: Vec<&str> = input.split('=').collect();

    if sides.len() != 2 {
        return Err(format!("Equation must contains one '=' sign"));
    }

    let mut coef = vec![0.0; sides[0].len() + sides[1].len()];
    let mut check_zero;


    check_zero = sides[0].trim();

    if check_zero == "0" {}

    else if let Err(e) = parse_side(&sides[0], &mut coef, 1.0) {
        return Err(e);
    }


    check_zero = sides[1].trim();

    if check_zero == "0" {}

    else if let Err(e) = parse_side(&sides[1], &mut coef, -1.0) {
        return Err(e);
    }


    let mut max_degree = 0;
    for i in (0..coef.len()).rev() {
        if coef[i].abs() > 1e-10 {
            max_degree = i;
            break;
        }
    }

    coef.truncate(max_degree + 1);

    Ok((coef, max_degree))

}

fn parse_side(side: &str, coef: &mut Vec<f64>, sign: f64) -> Result<(), String> {

    let processed_side = side.replace("+", " + ").replace("-", " - ");
    let tokens: Vec<&str> = processed_side.split_whitespace().collect();
    let mut i = 0;


    while i < tokens.len() {
        let mut term_sign = sign;

        if tokens[i] == "+" {
            i += 1;
        }
        else if tokens[i] == "-" {
            term_sign = -term_sign;
            i += 1;
        }

        if i >= tokens.len() {
            return Err(format!("Incorrect expression after '+' or '-'"));
        }

        let coef_str = tokens[i];

        let coefficient = match coef_str.parse::<f64>() {
            Ok(val) => val * term_sign,
            Err(_) => return Err(format!("Invalid coefficient: {}", coef_str)),
        };

        i += 1;

        if i >= tokens.len() || tokens[i] != "*" {
            return Err(format!("Expect '*' after coefficient {}", coef_str));
        }

        i += 1;

        if i >= tokens.len() {
            return Err(format!("Incorrect expression"));
        }

        let var_power = tokens[i];

        if !var_power.starts_with("X^") {
            return Err(format!("Expecter format: 'X^n', receive {}", var_power));
        }

        let power_str = &var_power[2..];

        let power = match power_str.parse::<usize>() {
            Ok(val) => val,
            Err(_) => return Err(format!("Power not valid: {}", power_str)),
        };

        if power >= coef.len() {
            return Err(format!("Power too big: {}", power));
        }

        coef[power] += coefficient;

        i += 1;
    }

    Ok(())
}

fn print_reduced_form(coef: &Vec<f64>) {

    print!("Reduced form: ");

    let mut i = 0;
    while i < coef.len() {
        if i == 0 {
            print!("{} * X^{}", coef[i], i);
        }

        else {
            if coef[i] < 0.0 {
                print!(" - {} * X^{}", coef[i].abs(), i);
            }

            else {
                print!(" + {} * X^{}", coef[i], i);
            }
        }

        i += 1;
    }
    println!(" = 0");
    println!("Polynomial degree: {}", i - 1);
}


fn solve_equation(coef: &[f64], degree: usize) {
    match degree {
        0 => solve_degree_zero(&coef),
        1 => solve_degree_one(&coef),
        2 => solve_degree_two(&coef),
        _ => println!("The polynomial degree is strictly greater than 2, I can't solve."),
    }
}

fn solve_degree_zero(coef: &[f64]) {
    let c = coef[0];

    if c.abs() < 1e-10 {
        println!("Any real number is a solution.");
    }
    else {
        println!("No solution.");
    }
}

fn solve_degree_one(coef: &[f64]) {
    
    let coef_zero = coef[0];
    let coef_one = coef[1];

    let result = coef_zero / (-1.0 * coef_one);

    println!("The solution is:\n{}", result);
}

fn solve_degree_two(coef: &[f64]) {
    
    let c = coef[0];
    let b = coef[1];
    let a = coef[2];

    let delta = (b * b) - (4.0 * a * c);
    
    if delta > 0.0 {
        println!("Discriminant is strictly positive, the two solutions are:");
        println!("{}\n{}", (-b - delta.sqrt())/(2.0 * a), (-b + delta.sqrt())/(2.0 * a))
    }

    else if delta.abs() < 1e-10 {
        println!("Discriminant is zero, the solution is:");
        println!("{}", (-b)/(2.0 * a));
    }

    else {
        println!("Discriminant is strictly negative, the two complex solutions are:");

        let real_part = -b / (2.0 * a);
        let imaginary_part = (-delta).sqrt() / (2.0 * a);
        println!("{} {}", a, b);
        if real_part.abs() < 1e-10 {
            println!("{}i", imaginary_part);
            println!("-{}i", imaginary_part);
        }
        else {
            println!("{} + {}i", real_part, imaginary_part);
            println!("{} - {}i", real_part, imaginary_part);
        }
    }
}