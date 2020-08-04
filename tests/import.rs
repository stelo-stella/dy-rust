use dy::*;

#[test]
#[cfg(feature = "import")]
fn import_dll_test() {
    let m = Module::new("import_test", &[&format!("{}/{}", module_path!(), "..")]).unwrap();
    let f = m.get_fn("multiply_two_only_numbers").unwrap();
    let args = vec![
        Value::new_int(5),
        Value::new_float(6.3),
        Value::new_str("Hello"),
    ];
    let res = f.call(args);
    let res = res.as_arr().unwrap();
    for elem in res.iter() {
        match elem.as_type() {
            As::Int(i) => assert_eq!(i.get(), 10),
            As::Float(f) => assert_eq!(f.get(), 12.6),
            As::Str(s) => assert_eq!(s.get(), "Hello"),
            _ => panic!("Invalid type"),
        }
    }
}
