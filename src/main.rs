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

// "3 * X^1 + 2 * X^2 = 1 * X^0"
fn parse_equation(input: &str) -> Result<(Vec<f64>, usize), String> {
    let sides: Vec<&str> = input.split('=').collect();                  // Vec = [ "3 * X^1 + 2 * X^2", "1 * X^0" ]

    if sides.len() != 2 {
        return Err(format!("Equation must contains one '=' sign"));
    }

    let mut coef = vec![0.0; sides[0].len() + sides[1].len()]; // Vec = [0.0, 0.0, 0.0]
    let mut check_zero;


    check_zero = sides[0].trim();

    if check_zero == "0" {}                                              // If only 0 in one side don't parse side

    else if let Err(e) = parse_side(&sides[0], &mut coef, 1.0) {
        return Err(e);
    }


    check_zero = sides[1].trim();

    if check_zero == "0" {}

    else if let Err(e) = parse_side(&sides[1], &mut coef, -1.0) {
        return Err(e);
    }

    // Coef is now something like this: Vec<f64> = [1, 3, 1] for x^0 + 2x^2 + 3x^1 = 0
    let mut max_degree = 0;
    for i in (0..coef.len()).rev() {
        if coef[i].abs() > 1e-10 {
            max_degree = i;
            break;
        }
    }

    coef.truncate(max_degree + 1);  // Erase useless 0 coef bigger that max_degree

    Ok((coef, max_degree))  

}

fn parse_side(side: &str, coef: &mut Vec<f64>, sign: f64) -> Result<(), String> {

    let processed_side = side.replace("+", " + ").replace("-", " - ");     // Side of equation
    let tokens: Vec<&str> = processed_side.split_whitespace().collect();                             // Slice the str and collect them into Vec<f64>
    let mut i = 0;

    
    while i < tokens.len() {
        let mut term_sign = sign;        // 1.0 or -1.0 dependings of the side

        if tokens[i] == "+" {                       
            i += 1;
        }
        else if tokens[i] == "-" {
            term_sign = -term_sign;         // + becomes - left / - becomes + right
            i += 1;
        }

        if i >= tokens.len() {
            return Err(format!("Incorrect expression after '+' or '-'"));
        }

        let coef_str = tokens[i];       // After a sign it's a coef

        let coefficient = match coef_str.parse::<f64>() {
            Ok(val) => val * term_sign,
            Err(_) => return Err(format!("Invalid coefficient: {}", coef_str)),
        };

        i += 1;

        if i >= tokens.len() || tokens[i] != "*" {              // Need * after coef
            return Err(format!("Expect '*' after coefficient {}", coef_str));
        }

        i += 1;

        if i >= tokens.len() {
            return Err(format!("Incorrect expression"));
        }

        let var_power = tokens[i];                       // var_power should be X^n with n any positive number

        if !var_power.starts_with("X^") {
            return Err(format!("Expecter format: 'X^n', receive {}", var_power));
        }

        let power_str = &var_power[2..];                    // power_str = power of X
    
        let power = match power_str.parse::<usize>() {      // Check if valid power
            Ok(val) => val,
            Err(_) => return Err(format!("Power not valid: {:?}", power_str)),
        };

        if power >= coef.len() {                                   
            return Err(format!("Power too big: {}", power));
        }

        coef[power] += coefficient;             // Ajoute au tableau de coef a l'index power le coefficient(pos ou neg)

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

    if result.abs() > 1e-10 {
        println!("The solution is:\n{}", result);
    }
    else {
        println!("The solution is:\n0");
    }
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
        if b.abs() > 1e-10 {
            println!("{}",(-b)/(2.0 * a))
        }
        else {
            println!("0");
        }
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