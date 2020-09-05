fn _diamond(f: fn(Rc<Label>) -> Rc<Type>, tp: Rc<Type>) -> Rc<Type> {
    match tp.as_ref() {
        Type::Prim |
        Type::Zero | 
        Type::One => tp,
        Type::Lbl(rc_lbl) => f(rc_lbl.clone()),
        Type::Sum(tp1, tp2) => Rc::new(Type::Sum(_diamond(f, tp1.clone()), _diamond(f, tp2.clone()))),
        Type::Product(tp1, tp2) => Rc::new(Type::Product(_diamond(f, tp1.clone()), _diamond(f, tp2.clone()))),
    }
}

type _F = fn(Rc<Label>) -> Rc<Type>;
type _G = fn(Rc<Element>) -> Rc<Value>;
type _TV = (Rc<Type>, Rc<Value>);

fn _diamond_value(f: _F, g: _G, tp_val: _TV) -> _TV {
    let (tp, val) = tp_val;
    match (tp.as_ref(), val.as_ref()) {
        (Type::One, Value::Unit) => (Rc::new(Type::One), Rc::new(Value::Unit)),
        (Type::Prim, _) => (Rc::new(Type::Prim), val.clone()),
        (Type::Lbl(lbl), Value::Id(elem)) => (f((*lbl).clone()), g((*elem).clone())),
        (Type::Sum(tp1, _tp2), Value::Inl(v, vtp2)) => {
            (
                _diamond(f, tp.clone()), 
                Rc::new(Value::Inl(
                    _diamond_value(f, g, (_diamond(f, (*tp1).clone()), (*v).clone())).1,
                    _diamond(f, (*vtp2).clone())
                ))
            )
        },
        (Type::Sum(_tp1, tp2), Value::Inr(vtp1, v)) => {
            (
                _diamond(f, tp.clone()), 
                Rc::new(Value::Inr(
                    _diamond(f, (*vtp1).clone()),
                    _diamond_value(f, g, (_diamond(f, (*tp2).clone()), (*v).clone())).1
                ))
            )
        },
        (Type::Product(tp1, tp2), Value::Pair(v1, v2)) => {
            (
                _diamond(f, tp.clone()),
                Rc::new(Value::Pair(
                    _diamond_value(f, g, ((*tp1).clone(), (*v1).clone())).1,
                    _diamond_value(f, g, ((*tp2).clone(), (*v2).clone())).1
                )),
            )
        },
        _ => unimplemented!(),
    }
    
}