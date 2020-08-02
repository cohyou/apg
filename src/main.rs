extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "apg.pest"]
pub struct APGParser;

use std::rc::Rc;
use apg::*;

#[allow(dead_code)]
fn apg() -> APG {
    let mut apg = APG::default();

    // set up
    add!(apg[v1: Person]);
    add!(apg[v2: Person]);
    let ev1 = ev!(apg, v1);
    let ev2 = ev!(apg, v2);
    add!(apg[e1: knows<ev1 * ev2>]);
    apg
}

#[allow(dead_code)]
fn eq() -> APG {
    let unit = Rc::new(Value::Unit);
    let type_m = label_type!(M);
    let type_f = label_type!(F);

    let mut apg1 = APG::default();
    add!(apg1[f1: Sex <unit   L+ type_m>]);
    add!(apg1[m1: Sex <type_f R+ unit  >]);

    let mut apg2 = APG::default();
    add!(apg2[f1: Sex <unit   L+ type_m>]);
    add!(apg2[m1: Sex <type_f R+ unit  >]);

    let apg_ref1 = Rc::new(apg1);
    let apg_ref2 = Rc::new(apg2);

    let mor1 = mor!(apg_ref1, apg_ref2, |_e| elem!(f1));
    let mor2 = mor!(apg_ref1, apg_ref2, |e| e);

    println!("<FROM>\n{:?}", apg_ref1);
    println!("<TO>\n{:?}", apg_ref2);

    get_equalizer(&mor1, &mor2)
}

fn read_csv() -> std::io::Result<APG> {
    use std::io::{BufRead, BufReader};
    use std::fs::File;

    let f = File::open("data/seven_heroes.csv")?;
    let reader = BufReader::new(f);

    let mut apg = APG::default();
    let mut titles = vec![];    
    for line in reader.lines() {
        if let Ok(l) = line {
            let l_ref = &l;
            if titles.is_empty() {
                for col in l_ref.split(",") {                    
                    titles.push(col.to_owned());
                }
            } else {
                
                let cols: Vec<_> = l_ref.split(",").collect();
                apg.add_lambda_upsilon(&cols[0].to_owned(), &titles[0], Value::Unit);
                apg.add_lambda_upsilon(&cols[1].to_owned(), &titles[1], Value::Unit);

                let ev1 = apg.get_element_value(&cols[0]).unwrap();
                let ev2 = apg.get_element_value(&cols[1]).unwrap();
                // add!(apg[e1: originated<ev1 * ev2>]);                            
                apg.add_lambda_upsilon(&(cols[0].to_owned() + "行"), "originated", Value::Pair(ev1, ev2));
            }
        }
    }

    println!("{:?}", apg);

    Ok(apg)
}

use std::fs;

// use pest::iterators::Pair;
// use apg::Value::Pair;

fn main() {
    // let successful_parse = APGParser::parse(Rule::apg, "{ e1 = () :True [1] }");
    // println!("{:?}", successful_parse);

    let unparsed_file = fs::read_to_string("src/_.apg").expect("cannot read file");
    let file = APGParser::parse(Rule::apg_file, &unparsed_file)
        .expect("unsuccessful parse") 
        .next().unwrap(); 

    for line in file.into_inner() {
        // println!("{:?}", line);
        for l in line.into_inner() {
            for l in l.into_inner() {
                println!("Pair: {{");
                println!("  {:?}", l.as_rule());
                println!("  {:?}", l.as_span());
                println!("  {:?}", l.into_inner());
                println!("}}");
            }
        }
    }

    // let _res = read_csv();


    // let apg = apg();

    // let equalizer = eq();
    // println!("<EQ>\n{:?}", equalizer);
}
