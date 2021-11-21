use clipboard::{ ClipboardContext, ClipboardProvider};
use rand::{Rng};
use std::{thread::sleep, time::Duration};
use std::env;

fn gen_rand_str(chars_to_gen: usize) -> String {
    rand::thread_rng()
    .sample_iter::<char, _>(rand::distributions::Standard)
    .take(chars_to_gen)
    .map(char::from)
    .collect()
}

fn help<'h>() -> &'h str{
return "usage:
rage-text [options] <chars to gen> <ms between updates>
    every n ms, generate a new random string and copy it to the clipboard."
}

fn err_inf() {
    eprintln!("use -h for help");
    eprintln!("error while parsing args:");
}

fn parse_args<'a>() -> Result<(u64, usize), &'a str> {
    //(sleep_ms, chars_to_gen)
    let args: Vec<String> = env::args().collect();
    
    match args.len() {
        1 => {//no args passed
            err_inf();
            Err("missing arguments!")
        }
        2 => {
            match args[1].as_str() {
                "-h" => Err(help()),
                _ => {
                    err_inf();
                    Err("invalid arguments")
                }
            }
        }
        3 => {
            let sleep_ms_str = &args[1];
            let chars_to_gen_str = &args[2];
            let chars_to_gen: u64 = match chars_to_gen_str.parse::<u64>() {
                Ok(n) => n,
                Err(_) => {
                    err_inf();
                    return Err("error: chars to gen not a positive intager");
                },
            };
            let sleep_ms: usize = match sleep_ms_str.parse::<usize>() {
                Ok(n) => n,
                Err(_) => {
                    err_inf();
                    return Err("error: ms between updates not a intager");
                },
            };
            return Ok((chars_to_gen, sleep_ms))
        }
        _ => {
            err_inf();
            return Err("To many args!!")
        }
    }
}

fn main() {
    let args = parse_args();
    let args_val = match args {
        Ok(vals) => vals,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    let sleep_time: Duration = Duration::from_millis(args_val.0);
    println!("generating {} chars every {} seconds", args_val.1, sleep_time.as_secs_f64());
    loop {
        sleep(sleep_time);
        println!("updating");
        ctx.set_contents(gen_rand_str(args_val.1).to_owned()).unwrap();
    }
}
