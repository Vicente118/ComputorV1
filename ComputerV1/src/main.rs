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
            println!("{:?}", coef);
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

    if let Err(e) = parse_side(&sides[0], &mut coef, 1.0) {
        return Err(e);
    }
    if let Err(e) = parse_side(&sides[1], &mut coef, -1.0) {
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