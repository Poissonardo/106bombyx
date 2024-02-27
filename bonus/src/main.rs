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
            if result > 1000 {
                display_error(format!("Invalid argument \"{}\", please enter a value bellow 1000", value).as_str());
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

    for i in 0..100 {
        if i != 0 {
            result = result * k * (1000.0 - result) / 1000.0;
        }
        println!("{} {:.2}", (i + 1), result);
    }
}

fn case2(_args: Vec<String>) -> i32 {
    println!("case1");
    0
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let nb_args = args.len();

    if nb_args == 3 {
        case1(args);
        exit(0);
    } else if nb_args == 4 {
        exit(case2(args));
    } else if nb_args == 2 && args[1].eq(HELP_TAG) {
        println!("USAGE\n\t./106bombyx n [k | i0 i1]\nDESCRIPTION\n\tn\tnumber of first generation individuals\n\tk\tgrowth rate from 1 to 4\n\ti0\tinitial generation (included)\n\ti1\tfinal generation (included)");
        exit(84);
    } else {
        display_error("Wrong argument number (try \"-h\" to display help)");
        exit(84);
    }

}
