use proc_macro::*;

fn expect_ident(tt: Option<&TokenTree>) -> Ident {
    match tt {
        Some(TokenTree::Ident(id)) => id.clone(),
        Some(TokenTree::Group(g)) => match g.stream().into_iter().next() {
            Some(TokenTree::Ident(id)) => id,
            _ => panic!("expected identifier"),
        },
        _ => panic!("expected identifier"),
    }
}

#[proc_macro]
#[doc(hidden)]
pub fn create_enum(item: TokenStream) -> TokenStream {
    let input: Vec<TokenTree> = item.into_iter().collect();

    let mut s;

    let id1 = expect_ident(input.get(0));
    if let Some(TokenTree::Group(g2)) = input.get(1) {
        s = format!(
            "#[derive(Debug, Eq, PartialEq, Clone)]\npub enum {}{}\n",
            id1.to_string(),
            r"{"
        );

        for tmp in g2.stream() {
            // tmp corresponds to a variant
            if let TokenTree::Group(g21) = tmp {
                let gv: Vec<TokenTree> = g21.stream().into_iter().collect();

                let id2 = expect_ident(gv.get(0));
                if let Some(TokenTree::Group(g212)) = gv.get(1) {
                    // g211 corresponds to variant name, and g212 to a tuple of types
                    s += &format!("    {}(", id2.to_string());

                    for tree in g212.stream() {
                        match tree {
                            TokenTree::Ident(id) => {
                                // Box<T>
                                s += &format!("Box<{}>", id.to_string());
                            }

                            TokenTree::Group(g) => {
                                if g.delimiter() == Delimiter::Brace {
                                    // regex
                                    let gvv: Vec<TokenTree> = g.stream().into_iter().collect();

                                    if let Some(hoge) = gvv.get(0) {
                                        match hoge {
                                            TokenTree::Group(_) => {
                                                // regex
                                                if let None = gvv.get(1) {
                                                    // String
                                                    s += "String";
                                                } else if let Some(TokenTree::Ident(idd)) =
                                                    gvv.get(2)
                                                {
                                                    // (String, T)
                                                    s += &format!("(String,{})", idd.to_string());
                                                } else {
                                                    panic!("syntax error7.");
                                                }
                                            }

                                            TokenTree::Ident(_) => {
                                                // reserved word
                                                if let None = gvv.get(2) {
                                                    // String
                                                    s += "String";
                                                } else if let Some(TokenTree::Ident(idd)) =
                                                    gvv.get(3)
                                                {
                                                    // (String, T)
                                                    s += &format!("(String,{})", idd.to_string());
                                                } else {
                                                    panic!("syntax error7.");
                                                }
                                            }

                                            _ => {
                                                panic!("syntax error10.");
                                            }
                                        }
                                    } else {
                                        panic!("syntax error9.");
                                    }
                                // } else if g.delimiter() == Delimiter::Parenthesis {
                                //     // Vec<Rc<RefCell<T>>>
                                //     if let Some(TokenTree::Ident(id)) =
                                //         g.stream().into_iter().next()
                                //     {
                                //         s += &format!(
                                //             "Rc<RefCell<stack::Stack<{}>>>",
                                //             id.to_string()
                                //         );
                                //     } else {
                                //         panic!("syntax error8.");
                                //     }
                                } else if g.delimiter() == Delimiter::Bracket {
                                    // stack::Stack<S,T>

                                    let mut iter = g.stream().into_iter();

                                    if let Some(TokenTree::Ident(id1)) = iter.next() {
                                        iter.next();

                                        if let Some(TokenTree::Ident(id2)) = iter.next() {
                                            s += &format!(
                                                "stack::Stack<{},{}>",
                                                id1.to_string(),
                                                id2.to_string()
                                            );
                                        } else {
                                            s += &format!(
                                                "stack::Stack<{},{}>",
                                                id1.to_string(),
                                                id1.to_string()
                                            );
                                        }
                                    } else {
                                        panic!("syntax error8.");
                                    }
                                } else {
                                    panic!("syntax error9.");
                                }
                            }

                            TokenTree::Punct(_) => {
                                s += ",";
                            }

                            _ => {
                                panic!("syntax error6.");
                            }
                        }
                    }

                    s += "),\n";
                } else {
                    panic!("syntax error4.");
                }
            }
        }

        s += r"}";
    } else {
        panic!("syntax error1.");
    }

    s.parse().unwrap()
}

#[proc_macro]
#[doc(hidden)]
pub fn create_match(item: TokenStream) -> TokenStream {
    let input: Vec<TokenTree> = item.into_iter().collect();
    let mut n: u32 = 0;

    let mut s;

    let id1 = expect_ident(input.get(0));
    if let Some(TokenTree::Group(g2)) = input.get(1) {
        s = format!("match self{}", r"{");

        for tmp in g2.stream() {
            if let TokenTree::Group(g21) = tmp {
                let gv: Vec<TokenTree> = g21.stream().into_iter().collect();

                let id2 = expect_ident(gv.get(0));
                if let Some(TokenTree::Group(g212)) = gv.get(1) {
                    let mut val = format!("{}::{}(", id1, id2);
                    let mut exp = r"{".to_string();

                    for tree in g212.stream() {
                        match tree {
                            TokenTree::Ident(_) => {
                                // Box<T>
                                val += "field";
                                val += &format!("{}", n);

                                exp += &format!(r#"write!(f,"{}{}",field{});"#, r"{", r"}", n);

                                n += 1;
                            }

                            TokenTree::Group(g) => {
                                if g.delimiter() == Delimiter::Brace {
                                    // regex
                                    val += "field";
                                    val += &format!("{}", n);

                                    let gvv: Vec<TokenTree> = g.stream().into_iter().collect();

                                    if let Some(hoge) = gvv.get(0) {
                                        match hoge {
                                            TokenTree::Group(_) => {
                                                // regex
                                                if let None = gvv.get(1) {
                                                    // String
                                                    exp += &format!(
                                                        r#"write!(f,"{}{}",field{});"#,
                                                        r"{", r"}", n
                                                    );
                                                } else if let Some(TokenTree::Ident(_)) = gvv.get(2)
                                                {
                                                    // (String, T)
                                                    exp += &format!(
                                                        r#"write!(f,"{}{}",field{}.0);"#,
                                                        r"{", r"}", n
                                                    );
                                                } else {
                                                    panic!("syntax error7.");
                                                }
                                            }

                                            TokenTree::Ident(_) => {
                                                // reserved word
                                                if let None = gvv.get(2) {
                                                    // String
                                                    exp += &format!(
                                                        r#"write!(f,"{}{}",field{});"#,
                                                        r"{", r"}", n
                                                    );
                                                } else if let Some(TokenTree::Ident(_)) = gvv.get(3)
                                                {
                                                    // (String, T)
                                                    exp += &format!(
                                                        r#"write!(f,"{}{}",field{}.0);"#,
                                                        r"{", r"}", n
                                                    );
                                                } else {
                                                    panic!("syntax error7.");
                                                }
                                            }

                                            _ => {
                                                panic!("syntax error10.");
                                            }
                                        }
                                    } else {
                                        panic!("syntax error9.");
                                    }
                                // } else if g.delimiter() == Delimiter::Parenthesis {
                                //     // Vec<Rc<RefCell<T>>>
                                //     val += "field";
                                //     val += &format!("{}", n);

                                //     if let Some(TokenTree::Ident(_)) =
                                //         g.stream().into_iter().next()
                                //     {
                                //         exp += &format!(
                                //             r#"write!(f,"{}{}",field{}.borrow());"#,
                                //             r"{", r"}", n
                                //         );
                                //     } else {
                                //         panic!("syntax error8.");
                                //     }
                                } else if g.delimiter() == Delimiter::Bracket {
                                    // stack::Stack<S,T>

                                    val += "field";
                                    val += &format!("{}", n);

                                    if let Some(TokenTree::Ident(_)) = g.stream().into_iter().next()
                                    {
                                        exp +=
                                            &format!(r#"write!(f,"{}{}",field{});"#, r"{", r"}", n);
                                    } else {
                                        panic!("syntax error8.");
                                    }
                                } else {
                                    panic!("syntax error9.");
                                }

                                n += 1;
                            }

                            TokenTree::Punct(_) => {
                                val += ",";
                                exp += &format!(r#"write!(f," ");"#);
                            }

                            _ => {
                                panic!("syntax error6.");
                            }
                        }
                    }

                    val += ")";
                    exp += r"}";

                    s += &val;
                    s += "=>";
                    s += &exp;
                } else {
                    panic!("syntax error4.");
                }
            }
        }

        s += r"}";
    } else {
        panic!("syntax error1.");
    }

    s.parse().unwrap()
}
