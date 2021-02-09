#[macro_use]
use crate::def::*;
use regex::Regex;

add_rule!(
    Nat => Zero ({reg("Z")})
        |   Succ ({reg("S")},{reg(r"\(")},Nat,{reg(r"\)")})
);

add_rule!(
    Judgement => Plus (Nat, {reg("plus")}, Nat, {reg("is")}, Nat)
        |   Times (Nat, {reg("times")}, Nat, {reg("is")}, Nat)
);

fn reg(s: &str) -> Regex {
    Regex::new(s).unwrap()
}
