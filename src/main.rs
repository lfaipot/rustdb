use std::process;
use std::collections::HashMap;

use clap::Parser;

mod builtin;
mod config;
use crate::builtin::base::*;
use crate::config::loadvariables::*;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    variable: String,
}

fn set_value(variable_list: &mut HashMap<String, Variable>, reference: &str, varvalue: VarValue) {

    let var = variable_list.get_mut(reference).unwrap();

    println!("====== BEFORE");
    var.print_info();
    // let v: &mut Variable = var.unwrap();
    var.set_value(varvalue);
    println!("====== AFTER");
    var.print_info();
}

fn main() {
    let args = Args::parse();

    let mut variable_list: HashMap<String, Variable> = HashMap::new();

    if let Err(err) = load_csv(args.variable, &mut variable_list) {
        println!("error while loading: {}", err);
        process::exit(1);
    }

    set_value(&mut variable_list, "GROUP0101_ACTIVE", VarValue {boolean: true});
    set_value(&mut variable_list, "GROUP0101_PROD", VarValue {float: 25.4});
    set_value(&mut variable_list, "GATE01_WATERLEVEL", VarValue {uint: 127});

    println!("RustDB started");
}