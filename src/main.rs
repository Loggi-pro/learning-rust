fn generic_function<Type1>(val: Type1) -> Type1 {
    return val;
}

//get lifetime of VAL and stick it to lifetime of return value (make same lifetime)
fn generic_function_borrow<'t, Type1>(val: &'t Type1) -> &'t Type1 {
    return val;
}

pub fn main() {
    let _x = generic_function(4);
    let _y = generic_function_borrow(&_x);
}
