extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "apg.pest"]
pub struct APGParser;

use std::fs;
use std::rc::Rc;
use std::collections::HashMap;

fn _parse() {
    let unparsed_file = fs::read_to_string("src/_.apg")
        .expect("cannot read file");
    let file = APGParser::parse(Rule::apg_file, &unparsed_file)
        .expect("unsuccessful parse") 
        .next().unwrap();

    let mut symbols: HashMap<String, _APGTerm> = HashMap::new();

    for def in file.into_inner() {
        match def.as_rule() {
            Rule::define => {
                let mut inner = def.into_inner();
                let mut apg = APG::default();

                // symbol
                let span = inner.next().unwrap().as_span();
                let sym = span.as_str();
                apg.name = sym.to_string();

                // apg
                let line = inner.next().unwrap();
                for l in line.into_inner() {
                    let mut inner = l.into_inner();
        
                    // element
                    let span = inner.next().unwrap().as_span();
                    let element = span.as_str();

                    // value
                    let span = inner.next().unwrap().as_span();
                    let value = match span.as_str() {
                        "()" => Value::Unit,
                        _ => unimplemented!(),
                    };

                    // label
                    let span = inner.next().unwrap().as_span();
                    let label = span.as_str();

                    // type
                    let span = inner.next().unwrap().as_span();
                    let tp = match span.as_str() {
                        "1" => Type::One,
                        "0" => Type::Zero,
                        _ => unimplemented!(),
                    };
                    let tp_from = match value {
                        Value::Unit => Type::One,
                        _ => unimplemented!(),
                    };
                    if tp != tp_from {
                        println!("NG");
                    }

                    // apg.add_lambda_upsilon(element, label, value);
                }

                // シンボルテーブルに入れる
                symbols.insert(sym.to_string(), _APGTerm::Apg(apg));
            },
            Rule::plus => {
                let apg = APG::default();
                let mut inner = def.into_inner();

                // symbol
                let span = inner.next().unwrap().as_span();
                let sym = span.as_str();     
                let span2 = inner.next().unwrap().as_span();
                let sym2 = span2.as_str();     
                let span3 = inner.next().unwrap().as_span();
                let sym3 = span3.as_str();

                // シンボルテーブルに入れる
                let ope1 = Box::new(_APGTerm::Sym(sym2.to_string()));
                let ope2 = Box::new(_APGTerm::Sym(sym3.to_string()));
                symbols.insert(sym.to_string(), _APGTerm::Plus(ope1, ope2));
            },
            _ => {
                // println!("others: {:?}", def);
            },
        }
    }
    // println!("{:?}", symbols);

    let mut iter = symbols.iter().filter(|e| e.1.is_apg());
    if let _APGTerm::Apg(apg1) = iter.next().unwrap().1 {
        // println!("age1:\n{:?}", apg1);
        if let _APGTerm::Apg(apg2) = iter.next().unwrap().1 {
            // println!("age2:\n{:?}", apg2);
            // println!("co-product:\n{:?}", co_product_of_apgs(apg1, apg2));
            // println!("product:\n{:?}", product_of_apgs(apg1, apg2));       
        }
    }    
}

#[derive(Debug)]
enum _APGTerm {
    Apg(APG),
    Plus(Box<_APGTerm>, Box<_APGTerm>),
    Sym(String),
}

impl _APGTerm {
    pub fn is_apg(&self) -> bool {
        match self {
            _APGTerm::Apg(_) => true,
            _ => false,
        }
    }
}

// fn read_csv() -> std::io::Result<APG> {
//     use std::io::{BufRead, BufReader};
//     use std::fs::File;

//     let f = File::open("data/seven_heroes.csv")?;
//     let reader = BufReader::new(f);

//     let mut apg = APG::default();
//     let mut titles = vec![];    
//     for line in reader.lines() {
//         if let Ok(l) = line {
//             let l_ref = &l;
//             if titles.is_empty() {
//                 for col in l_ref.split(",") {                    
//                     titles.push(col.to_owned());
//                 }
//             } else {
                
//                 let cols: Vec<_> = l_ref.split(",").collect();
//                 apg.add_lambda_upsilon(&cols[0].to_owned(), &titles[0], Value::Unit);
//                 apg.add_lambda_upsilon(&cols[1].to_owned(), &titles[1], Value::Unit);

//                 let ev1 = apg.get_element_value(&cols[0]).unwrap();
//                 let ev2 = apg.get_element_value(&cols[1]).unwrap();
//                 // add!(apg[e1: originated<ev1 * ev2>]);                            
//                 apg.add_lambda_upsilon(&(cols[0].to_owned() + "行"), "originated", Value::Pair(ev1, ev2));
//             }
//         }
//     }

//     println!("{:?}", apg);

//     Ok(apg)
// }
