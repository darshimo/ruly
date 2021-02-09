#[macro_use]
pub mod def;
pub mod rules;

use def::{Parse, Ruly};
use regex::Regex;
use rules::{
    Judgement::Plus,
    Nat::{Succ, Zero},
};

#[test]
fn test() {
    let z = Zero(String::from("Z"));

    let n1 = Succ(
        String::from("S"),
        String::from("("),
        Box::new(z.clone()),
        String::from(")"),
    );

    let n2 = Succ(
        String::from("S"),
        String::from("("),
        Box::new(n1.clone()),
        String::from(")"),
    );

    let n3 = Succ(
        String::from("S"),
        String::from("("),
        Box::new(n2.clone()),
        String::from(")"),
    );

    let n4 = Succ(
        String::from("S"),
        String::from("("),
        Box::new(n3.clone()),
        String::from(")"),
    );

    let j1 = Plus(
        Box::new(n1),
        String::from("plus"),
        Box::new(n2),
        String::from("is"),
        Box::new(n4),
    );

    let s = "S(Z) plus S(S(Z)) is S(S(S(S(Z))))";
    let mut ruly = Ruly::new();
    ruly.set_skip_reg(Regex::new(r"[ \n\r\t]*").unwrap());
    ruly.set_input(&s);

    if let Ok(j2) = ruly.run::<rules::Judgement>() {
        assert_eq!(j1, j2);
    } else {
        assert!(false);
    }
}
