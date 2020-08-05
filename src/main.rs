extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "apg.pest"]
pub struct APGParser;

use std::rc::Rc;
use apg::*;

#[derive(Debug)]
enum APGTerm {
    Apg(APG),
    Plus(Box<APGTerm>, Box<APGTerm>),
    Sym(String),
}

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

fn diamond(f: fn(Rc<Label>) -> Rc<Type>, tp: Rc<Type>) -> Rc<Type> {
    match tp.as_ref() {
        Type::Prim |
        Type::Zero | 
        Type::One => tp,
        Type::Lbl(rc_lbl) => f(rc_lbl.clone()),
        Type::Sum(tp1, tp2) => Rc::new(Type::Sum(diamond(f, tp1.clone()), diamond(f, tp2.clone()))),
        Type::Product(tp1, tp2) => Rc::new(Type::Product(diamond(f, tp1.clone()), diamond(f, tp2.clone()))),
    }
}

fn diamond_value(f: fn(Rc<Label>) -> Rc<Type>, g: fn(Rc<Element>) -> Rc<Value>, tp: Rc<Type>) -> (Rc<Type>, Rc<Value>) {
    (Rc::new(Type::One), Rc::new(Value::Unit))
}

use std::fs;
use std::collections::HashMap;

fn main() {
    let unparsed_file = fs::read_to_string("src/_.apg").expect("cannot read file");
    let file = APGParser::parse(Rule::apg_file, &unparsed_file)
        .expect("unsuccessful parse") 
        .next().unwrap();

    let mut symbols: HashMap<String, APGTerm> = HashMap::new();

    for def in file.into_inner() {
        match def.as_rule() {
            Rule::define => {
                let mut apg = APG::default();
                let mut inner = def.into_inner();

                // symbol
                let span = inner.next().unwrap().as_span();
                let sym = span.as_str();

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

                    apg.add_lambda_upsilon(element, label, value);
                }

                // シンボルテーブルに入れる
                symbols.insert(sym.to_string(), APGTerm::Apg(apg));
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
                let ope1 = Box::new(APGTerm::Sym(sym2.to_string()));
                let ope2 = Box::new(APGTerm::Sym(sym3.to_string()));
                symbols.insert(sym.to_string(), APGTerm::Plus(ope1, ope2));
            },
            _ => {
                println!("others: {:?}", def);
            },
        }
    }
    println!("{:?}", symbols);
    
    // let equalizer = eq();
    // println!("<EQ>\n{:?}", equalizer);
}
