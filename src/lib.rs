pub use pmacro_ruly::*;
pub use regex::Regex;
pub use std::vec;

#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! __set_fun_sub {
    ( ($t:ty,0); $p:ident ) => {{
        let mut v = vec![];

        while let Ok(a) = <$t>::read($p) {
            v.push(a);
        }

        v
    }};

    ( ($t:ty,1); $p:ident ) => {{
        let mut v = vec![<$t>::read($p)?];

        while let Ok(a) = <$t>::read($p) {
            v.push(a);
        }

        v
    }};

    ( $t:ty; $p:ident ) => {{
        Box::new(<$t>::read($p)?)
    }};

    ( {  ( $t:expr ), $sort:ty, $c:expr }; $p:ident ) => {{
        let tmp = $p.get_current();
        let reg = Regex::new($t).unwrap();
        let closure = $c;

        match $p.find_at_top(reg) {
            None => {
                return Err((String::from("no match"), tmp));
            }

            Some((end, s)) => {
                if ReservedWords.contains(&{ &s.to_string() }) {
                    return Err((String::from("no match"), tmp));
                }

                $p.set_current(end);
                $p.skip();
                (s.to_string(), closure(s))
            }
        }
    }};

    ( {  Reserved ( $t:expr ), $sort:ty, $c:expr }; $p:ident ) => {{
        let tmp = $p.get_current();
        let reg = Regex::new($t).unwrap();
        let closure = $c;

        match $p.find_at_top(reg) {
            None => {
                return Err((String::from("no match"), tmp));
            }

            Some((end, s)) => {
                if !ReservedWords.contains(&{ &s.to_string() }) {
                    return Err((String::from("no match"), tmp));
                }

                $p.set_current(end);
                $p.skip();
                (s.to_string(), closure(s))
            }
        }
    }};

    ( { ( $t:expr ) }; $p:ident ) => {{
        let tmp = $p.get_current();
        let reg = Regex::new($t).unwrap();

        match $p.find_at_top(reg) {
            None => {
                return Err((String::from("no match"), tmp));
            }

            Some((end, s)) => {
                if ReservedWords.contains(&{ &s.to_string() }) {
                    return Err((String::from("no match"), tmp));
                }

                $p.set_current(end);
                $p.skip();
                s.to_string()
            }
        }
    }};

    ( { Reserved ( $t:expr ) }; $p:ident ) => {{
        let tmp = $p.get_current();
        let reg = Regex::new($t).unwrap();

        match $p.find_at_top(reg) {
            None => {
                return Err((String::from("no match"), tmp));
            }

            Some((end, s)) => {
                if !ReservedWords.contains(&{ &s.to_string() }) {
                    return Err((String::from("no match"), tmp));
                }

                $p.set_current(end);
                $p.skip();
                s.to_string()
            }
        }
    }};
}

#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! __set_fun {
    ( $v:ident ; $i:ident ; ( $($sort:tt),* ); $p:ident ) => {
        Ok( $v::$i( $( __set_fun_sub!($sort;$p) ),* ) )
    };
}

#[macro_export(local_inner_macros)]
macro_rules! add_rule {
    ( $v:ident => $( $i:ident $sorts:tt )|+ ) => {
        create_enum!($v [ $([$i $sorts]),* ] );

        impl<P: Parse> Product<P> for $v {
            fn read(parser: &mut P) -> Result<Self, (String, usize)>{
                let start_point = parser.get_current();

                $(
                if !std::stringify!($i).starts_with("Dummy") {
                    let tmp = |p: &mut P| -> Result<Self, (String, usize)> {
                        __set_fun!($v;$i;$sorts;p)
                    };
                    if let Ok(a) = tmp(parser){
                        return Ok(a);
                    }
                    parser.set_current(start_point);
                }
                )*

                Err((String::from("no match"), start_point))
            }
        }

        impl std::fmt::Display for $v {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
                create_match!($v [ $([$i $sorts]),* ] );
                std::write!(f, "")
            }
        }
    };
}

#[macro_export(local_inner_macros)]
macro_rules! reserved_words {
    ( $( $e:expr ),* ) => {
        const ReservedWords: [&str; 0 $(  + { $e ; 1 } )* ] = [ $( $e ),* ];
    };
}

pub trait Parse: Sized {
    #[doc(hidden)]
    fn skip(&mut self);

    #[doc(hidden)]
    fn get_current(&self) -> usize;

    #[doc(hidden)]
    fn set_current(&mut self, c: usize);

    #[doc(hidden)]
    fn find_at_top(&self, reg: Regex) -> Option<(usize, String)>;

    fn new() -> Self;

    fn set_input(&mut self, s: &str);

    fn set_skip_reg(&mut self, reg_str: &str);

    fn run<T: Product<Self>>(&mut self) -> Result<T, (String, usize)>;
}

pub trait Product<P: Parse>: Sized {
    fn read(p: &mut P) -> Result<Self, (String, usize)>;
}

#[derive(Debug)]
pub struct Ruly {
    input: String,
    current: usize,
    skip_reg: Regex,
}

impl Parse for Ruly {
    fn skip(&mut self) {
        if let Some(mat) = self.skip_reg.find_at(&self.input, self.current) {
            if self.current == mat.start() {
                self.current = mat.end();
            }
        }
    }

    fn get_current(&self) -> usize {
        self.current
    }

    fn set_current(&mut self, c: usize) {
        self.current = c;
    }

    fn find_at_top(&self, reg: Regex) -> Option<(usize, String)> {
        if let Some(mat) = reg.find_at(&self.input, self.current) {
            if self.current == mat.start() {
                return Some((mat.end(), mat.as_str().to_string()));
            }
        }

        None
    }

    fn new() -> Self {
        Ruly {
            input: String::new(),
            current: 0,
            skip_reg: Regex::new(r"").unwrap(),
        }
    }

    fn set_input(&mut self, s: &str) {
        self.input = s.to_string();
        self.current = 0;
    }

    fn set_skip_reg(&mut self, reg_str: &str) {
        self.skip_reg = Regex::new(reg_str).unwrap();
    }

    fn run<T: Product<Self>>(&mut self) -> Result<T, (String, usize)> {
        self.skip();
        let ret = T::read(self);
        if self.is_end() {
            ret
        } else {
            Err((self.get_next_chars(), self.get_current()))
        }
    }
}

#[doc(hidden)]
impl Ruly {
    fn is_end(&self) -> bool {
        self.current == self.input.len()
    }

    fn get_next_chars(&self) -> String {
        String::from(&self.input[self.current..std::cmp::min(self.input.len(), self.current + 20)])
    }
}

reserved_words!();

#[test]
fn test() {
    add_rule!(
        Nat => Zero ({("Z")})
            |   Succ ({("S")},{(r"\(")},Nat,{(r"\)")})
    );

    add_rule!(
        Judgement => Plus (Nat, {("plus")}, Nat, {("is")}, Nat)
            |   Times (Nat, {("times")}, Nat, {("is")}, Nat)
    );

    println!("reserved words: {:?}", ReservedWords);

    let s = "S(Z) plus S(S(S(Z))) is S(S(S(S(Z))))";
    let mut ruly = Ruly::new();
    ruly.set_skip_reg(r"[ \n\r\t]*");
    ruly.set_input(&s);

    match ruly.run::<Judgement>() {
        Ok(judgement) => {
            println!("{:?}", judgement);
            println!("{}", judgement);
        }

        err => {
            println!("{:?}", err);
        }
    }
}
