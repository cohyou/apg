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

impl APGTerm {
    pub fn is_apg(&self) -> bool {
        match self {
            APGTerm::Apg(_) => true,
            _ => false,
        }
    }
}

// #[allow(dead_code)]
// fn apg() -> APG {
//     let mut apg = APG::default();

//     // set up
//     add!(apg[v1: Person]);
//     add!(apg[v2: Person]);
//     let ev1 = ev!(apg, v1);
//     let ev2 = ev!(apg, v2);
//     add!(apg[e1: knows<ev1 * ev2>]);
//     apg
// }

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

type F = fn(Rc<Label>) -> Rc<Type>;
type G = fn(Rc<Element>) -> Rc<Value>;
type TV = (Rc<Type>, Rc<Value>);

fn diamond_value(f: F, g: G, tp_val: TV) -> TV {
    let (tp, val) = tp_val;
    match (tp.as_ref(), val.as_ref()) {
        (Type::One, Value::Unit) => (Rc::new(Type::One), Rc::new(Value::Unit)),
        (Type::Prim, _) => (Rc::new(Type::Prim), val.clone()),
        (Type::Lbl(lbl), Value::Id(elem)) => (f((*lbl).clone()), g((*elem).clone())),
        (Type::Sum(tp1, _tp2), Value::Inl(v, vtp2)) => {
            (
                diamond(f, tp.clone()), 
                Rc::new(Value::Inl(
                    diamond_value(f, g, (diamond(f, (*tp1).clone()), (*v).clone())).1,
                    diamond(f, (*vtp2).clone())
                ))
            )
        },
        (Type::Sum(_tp1, tp2), Value::Inr(vtp1, v)) => {
            (
                diamond(f, tp.clone()), 
                Rc::new(Value::Inr(
                    diamond(f, (*vtp1).clone()),
                    diamond_value(f, g, (diamond(f, (*tp2).clone()), (*v).clone())).1
                ))
            )
        },
        (Type::Product(tp1, tp2), Value::Pair(v1, v2)) => {
            (
                diamond(f, tp.clone()),
                Rc::new(Value::Pair(
                    diamond_value(f, g, ((*tp1).clone(), (*v1).clone())).1,
                    diamond_value(f, g, ((*tp2).clone(), (*v2).clone())).1
                )),
            )
        },
        _ => unimplemented!(),
    }
    
}

use std::collections::HashSet;
fn add_prefix_to_labels(labels: &HashSet<Rc<Label>>, prefix: &str) -> HashSet<Rc<Label>> {
    labels.iter()
        .map(|e| Rc::new(Label(prefix.to_string() + &e.as_ref().0)))
        .collect()
}

fn add_prefix_to_elements(elements: &HashSet<Rc<Element>>, prefix: &str) -> HashSet<Rc<Element>> {
    elements.iter()
        .map(|e| {
            if let Element(e_name) = e.as_ref() {
                Rc::new(Element(prefix.to_string() + e_name))
            } else {
                unimplemented!()
            }
        })
        .collect()
}

fn replace_value(lambda_upsilon: &HashMap<String, (String, Rc<Value>)>, prefix: &str)
-> HashMap<String, (String, Rc<Value>)> {
    lambda_upsilon.iter()
        // .inspect(|(k, v)| println!("replace_value: {:?} {:?}", k, v))
        .map(|(k, v)| {
            let new_key_sym = prefix.to_string() + k;
            let new_label_sym = prefix.to_string() + v.0.as_ref();
            match v.1.as_ref() {
                Value::Id(rc_element) => {
                    if let Element(e_name) = rc_element.as_ref() {
                        let new_sym = prefix.to_string() + e_name;
                        (new_key_sym, (new_label_sym, Rc::new(Value::Id(Rc::new(Element(new_sym))))))
                    } else {
                        unimplemented!()
                    }
                },
                _ => (new_key_sym, (new_label_sym, v.1.clone())),
            }
        })
        .collect::<HashMap<String, (String, Rc<Value>)>>()
}

fn co_product_of_apgs(apg1: &APG, apg2: &APG) -> APG {
    let prefix1 = apg1.name.clone() + ".";
    let prefix2 = apg2.name.clone() + ".";

    let a = add_prefix_to_labels(&apg1.labels, &prefix1);
    let b = add_prefix_to_labels(&apg2.labels, &prefix2);
    let labels = &a | &b;

    let a_elements = add_prefix_to_elements(&apg1.elements, &prefix1);
    let b_elements = add_prefix_to_elements(&apg2.elements, &prefix2);
    let elements = &a_elements | &b_elements;

    let mut a_f = replace_value(&apg1.lambda_upsilon, &prefix1);
    let b_f = replace_value(&apg2.lambda_upsilon, &prefix2);

    a_f.extend(b_f);

    let new_name = prefix1.clone() + &prefix2;
    APG::new(&new_name, elements, labels, a_f)
}

fn product_of_apgs(apg1: &APG, apg2: &APG) -> APG {
    let mut new_labels: HashSet<Rc<Label>> = HashSet::new();
    for l1 in &apg1.labels {
        for l2 in &apg2.labels {
            let new_label = format!("{}*{}", l1.0, l2.0);
            new_labels.insert(Rc::new(Label(new_label)));
        }
    }
    let mut new_elements: HashSet<Rc<Element>> = HashSet::new();
    for l1 in &apg1.elements {
        if let Element(e_name1) = l1.as_ref() {
            for l2 in &apg2.elements {
                if let Element(e_name2) = l2.as_ref() {
                    let new_element = format!("{}*{}", e_name1, e_name2);
                    new_elements.insert(Rc::new(Element(new_element)));
                }
            }
        }
    }
    APG::new(&(apg1.name.clone() + &apg2.name), new_elements, new_labels, HashMap::new())
}

use std::fs;
use std::collections::HashMap;

fn main() {
    let unparsed_file = fs::read_to_string("src/_.apg")
        .expect("cannot read file");
    let file = APGParser::parse(Rule::apg_file, &unparsed_file)
        .expect("unsuccessful parse") 
        .next().unwrap();

    let mut symbols: HashMap<String, APGTerm> = HashMap::new();

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
                // println!("others: {:?}", def);
            },
        }
    }
    // println!("{:?}", symbols);

    let mut iter = symbols.iter().filter(|e| e.1.is_apg());
    if let APGTerm::Apg(apg1) = iter.next().unwrap().1 {
        // println!("age1:\n{:?}", apg1);
        if let APGTerm::Apg(apg2) = iter.next().unwrap().1 {
            // println!("age2:\n{:?}", apg2);
            // println!("co-product:\n{:?}", co_product_of_apgs(apg1, apg2));
            // println!("product:\n{:?}", product_of_apgs(apg1, apg2));
            
        }
    }
}
