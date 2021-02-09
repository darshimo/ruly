use regex::Regex;

#[macro_export]
#[doc(hidden)]
macro_rules! __set_enum {
    ( ($t:ty,0) ) => {
        Vec<$t>
    };

    ( ($t:ty,1) ) => {
        Vec<$t>
    };

    ( $t:ty ) => {
        Box<$t>
    };

    ( { $e:expr, $sort:ty, $c:expr } ) => {
        $sort
    };

    ( { $e:expr } ) => {
        String
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __set_fun {
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

    ( {  $t:expr, $sort:ty, $c:expr }; $p:ident ) => {{
        let tmp = $p.get_current();
        let reg = $t;
        let closure = $c;

        match $p.find_at_top(reg) {
            None => {
                return Err((String::from("no match"), tmp));
            }

            Some((end, s)) => {
                $p.set_current(end);
                $p.skip();
                closure(s)
            }
        }
    }};

    ( { $t:expr }; $p:ident ) => {{
        let tmp = $p.get_current();
        let reg = $t;

        match $p.find_at_top(reg) {
            None => {
                return Err((String::from("no match"), tmp));
            }

            Some((end, s)) => {
                $p.set_current(end);
                $p.skip();
                s.to_string()
            }
        }
    }};
}

#[macro_export]
macro_rules! add_rule {
    ( $v:ident => $( $i:ident ( $( $sort:tt ),* ) )|+ ) => {
        #[derive(Debug, Eq, PartialEq, Clone)]
        pub enum $v {
            $( $i( $( __set_enum!( $sort ) ),*  ) ),*
        }
        impl<P: Parse> Product<P> for $v {
            fn read(parser: &mut P) -> Result<Self, (String, usize)>{
                let start_point = parser.get_current();

                $(
                let tmp = |p: &mut P| -> Result<Self, (String, usize)> {
                    Ok($v::$i(
                        $( __set_fun!($sort; p) ,)*
                    ))
                };
                if let Ok(a) = tmp(parser){
                    return Ok(a);
                }
                parser.set_current(start_point);
                )*

                Err((String::from("no match"), start_point))
            }
        }
    };
}

pub trait Parse: Sized {
    fn new() -> Self;
    fn set_input(&mut self, s: &str);
    fn set_skip_reg(&mut self, reg: Regex);
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
impl Ruly {
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

    fn is_end(&self) -> bool {
        self.current == self.input.len()
    }

    fn get_next_chars(&self) -> String {
        String::from(&self.input[self.current..std::cmp::min(self.input.len(), self.current + 20)])
    }
}
impl Parse for Ruly {
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

    fn set_skip_reg(&mut self, reg: Regex) {
        self.skip_reg = reg;
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
