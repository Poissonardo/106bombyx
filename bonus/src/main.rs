use std::env;
use std::process::exit;

const HELP_TAG: &str = "-h";
const PROJECT_NAME: &str = "106bombyx";

fn display_error(error: &str) {
    println!("{}: {}", PROJECT_NAME, error);
}

fn get_user_int(value: &String) -> i32 {
    let n_result = value.parse::<i32>();

    match n_result {
        Err(_) => {
            display_error(format!("Invalid argument \"{}\", please enter a valid integer value", value).as_str());
            exit(84);
        }
        Ok(result) => {
            if result < 0 {
                display_error(format!("Invalid argument \"{}\", please enter a positive value", value).as_str());
                exit(84);
            }
            return result;
        }
    }
}

fn get_user_float(value: &String) -> f32 {
    let n_result = value.parse::<f32>();

    match n_result {
        Err(_) => {
            display_error(format!("Invalid argument \"{}\", please enter a valid integer value", value).as_str());
            exit(84);
        }
        Ok(result) => {
            if result < 0.0 {
                display_error(format!("Invalid argument \"{}\", please enter a positive value", value).as_str());
                exit(84);
            }
            if result > 4.0 {
                display_error(format!("Invalid argument \"{}\", please enter a value bellow 4.0", value).as_str());
                exit(84);
            }
            return result;
        }
    }
}

fn case1(args: Vec<String>) {
    let n = get_user_int(&args[1]);
    let k = get_user_float(&args[2]);
    let mut result = n as f32;

    if n > 1000 {
        display_error(format!("Invalid argument \"{}\", please enter a value bellow 1000", n).as_str());
        exit(84);
    }
    for i in 0..100 {
        if i != 0 {
            result = result * k * (1000.0 - result) / 1000.0;
        }
        println!("{} {:.2}", (i + 1), result);
    }
}

fn get_i0_population(n: i32, k: f32, i0: i32) -> f32 {
    let mut result = n as f32;

    for i in 0..i0 {
        if i != 0 {
            result = result * k * (1000.0 - result) / 1000.0;
        }
    }
    return result;
}

fn case2(args: Vec<String>) {
    let mut k = 1.00;
    let n = get_user_int(&args[1]);
    let i0 = get_user_int(&args[2]);
    let i1 = get_user_int(&args[3]);
    let mut result;

    if i0 > i1 {
        display_error(format!("Invalid argument \"{}\", i1 must be superior to i0", i1).as_str());
        exit(84);
    }
    if n > 1000 {
        display_error(format!("Invalid argument \"{}\", please enter a value bellow 1000", n).as_str());
        exit(84);
    }
    while k <= 4.0 {
        let i0_population_number = get_i0_population(n, k, i0);
        result = i0_population_number;
        for i in 0..(i1 - (i0 - 1)) {
            if i != i1 {
                result = result * k * (1000.0 - result) / 1000.0;
            }
            println!("{:.2} {:.2}", k, result);
        }
        k += 0.01;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let nb_args = args.len();

    if nb_args == 3 {
        case1(args);
        exit(0);
    } else if nb_args == 4 {
        case2(args);
        exit(0)
    } else if nb_args == 2 && args[1].eq(HELP_TAG) {
        println!("USAGE\n\t./106bombyx n [k | i0 i1]\nDESCRIPTION\n\tn\tnumber of first generation individuals\n\tk\tgrowth rate from 1 to 4\n\ti0\tinitial generation (included)\n\ti1\tfinal generation (included)");
        exit(84);
    } else {
        display_error("Wrong argument number (try \"-h\" to display help)");
        exit(84);
    }

}
