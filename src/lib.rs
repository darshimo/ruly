extern crate proc_macro;
use proc_macro::{Delimiter, TokenStream, TokenTree};

#[proc_macro]
#[doc(hidden)]
pub fn create_enum(item: TokenStream) -> TokenStream {
    let input: Vec<TokenTree> = item.into_iter().collect();

    let mut s;

    if let (Some(TokenTree::Group(g1)), Some(TokenTree::Group(g2))) = (input.get(0), input.get(1)) {
        if let Some(TokenTree::Ident(id1)) = g1.stream().into_iter().next() {
            s = format!(
                "#[derive(Debug, Eq, PartialEq, Clone)]\npub enum {}{}\n",
                id1.to_string(),
                r"{"
            );

            for tmp in g2.stream() {
                // tmp corresponds to a variant
                if let TokenTree::Group(g21) = tmp {
                    let gv: Vec<TokenTree> = g21.stream().into_iter().collect();

                    if let (Some(TokenTree::Group(g211)), Some(TokenTree::Group(g212))) =
                        (gv.get(0), gv.get(1))
                    {
                        // g211 corresponds to variant name, and g212 to a tuple of types
                        if let Some(TokenTree::Ident(id2)) = g211.stream().into_iter().next() {
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
                                            let gvv: Vec<TokenTree> =
                                                g.stream().into_iter().collect();
                                            if let None = gvv.get(2) {
                                                // String
                                                s += "String";
                                            } else {
                                                // (String, T)
                                                if let Some(TokenTree::Ident(idd)) = gvv.get(3) {
                                                    s += &format!("(String,{})", idd.to_string());
                                                } else {
                                                    panic!("syntax error7.");
                                                }
                                            }
                                        } else if g.delimiter() == Delimiter::Parenthesis {
                                            // Vec<T>
                                            if let Some(TokenTree::Ident(id)) =
                                                g.stream().into_iter().next()
                                            {
                                                s += &format!("Vec<{}>", id.to_string());
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
                            panic!("syntax error5.");
                        }
                    } else {
                        panic!("syntax error4.");
                    }
                }
            }

            s += r"}";
        } else {
            panic!("syntax error2.");
        }
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

    if let (Some(TokenTree::Group(g1)), Some(TokenTree::Group(g2))) = (input.get(0), input.get(1)) {
        if let Some(TokenTree::Ident(id1)) = g1.stream().into_iter().next() {
            s = format!("match self{}", r"{");

            for tmp in g2.stream() {
                if let TokenTree::Group(g21) = tmp {
                    let gv: Vec<TokenTree> = g21.stream().into_iter().collect();

                    if let (Some(TokenTree::Group(g211)), Some(TokenTree::Group(g212))) =
                        (gv.get(0), gv.get(1))
                    {
                        if let Some(TokenTree::Ident(id2)) = g211.stream().into_iter().next() {
                            let mut val = format!("{}::{}(", id1, id2);
                            let mut exp = r"{".to_string();

                            for tree in g212.stream() {
                                match tree {
                                    TokenTree::Ident(_) => {
                                        // Box<T>
                                        val += "field";
                                        val += &format!("{}", n);

                                        exp +=
                                            &format!(r#"write!(f,"{}{}",field{});"#, r"{", r"}", n);

                                        n += 1;
                                    }

                                    TokenTree::Group(g) => {
                                        if g.delimiter() == Delimiter::Brace {
                                            // regex
                                            val += "field";
                                            val += &format!("{}", n);

                                            let gvv: Vec<TokenTree> =
                                                g.stream().into_iter().collect();
                                            if let None = gvv.get(2) {
                                                // String
                                                exp += &format!(
                                                    r#"write!(f,"{}{}",field{});"#,
                                                    r"{", r"}", n
                                                );
                                            } else {
                                                // (String, T)
                                                if let Some(TokenTree::Ident(_)) = gvv.get(3) {
                                                    exp += &format!(
                                                        r#"write!(f,"{}{}",field{}.0);"#,
                                                        r"{", r"}", n
                                                    );
                                                } else {
                                                    panic!("syntax error7.");
                                                }
                                            }
                                        } else if g.delimiter() == Delimiter::Parenthesis {
                                            // Vec<T>
                                            val += "field";
                                            val += &format!("{}", n);

                                            if let Some(TokenTree::Ident(_)) =
                                                g.stream().into_iter().next()
                                            {
                                                exp += &format!(
                                                    r#"for (i, tmp) in field{}.into_iter().enumerate() {}if i>0 {}write!(f," ");{}write!(f,"{}{}",tmp);{}"#,
                                                    n, r"{", r"{", r"}", r"{", r"}", r"}"
                                                );
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
                            panic!("syntax error5.");
                        }
                    } else {
                        panic!("syntax error4.");
                    }
                }
            }

            s += r"}";
        } else {
            panic!("syntax error2.");
        }
    } else {
        panic!("syntax error1.");
    }

    s.parse().unwrap()
}
