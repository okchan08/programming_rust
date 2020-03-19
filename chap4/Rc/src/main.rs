use std::rc::Rc;


fn main() {
    let s = Rc::new("Test string".to_string());
    let t = s.clone();
    let u = s.clone();

    assert!(s.contains("Test"));
    assert_eq!(t.find("str"), Some(5));
    println!("Variable u: {}", u);

    // borrowed as immutable
    // compile error!
    // s.push_str("hogehoge");
}
