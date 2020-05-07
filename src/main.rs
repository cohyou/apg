// extern crate apg;

use std::rc::Rc;
use apg::*;

fn main() {
    let mut apg = APG::default();

    // set up
    add!(apg[v1: Person]); 
    add!(apg[v2: Person]);
    let ev1 = ev!(apg, v1);
    let ev2 = ev!(apg, v2);
    add!(apg[e1: knows<ev1 * ev2>]);

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

    let equalizer = get_equalizer(&mor1, &mor2);
    println!("<FROM>\n{:?}", apg_ref1);
    println!("<TO>\n{:?}", apg_ref2);
    println!("<EQ>\n{:?}", equalizer);
}
