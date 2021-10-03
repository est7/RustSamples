pub fn main() {}

fn match_wildcard() {
    let u8: u8 = 100;
    match u8 {
        1 => println!("value 1"),
        _ => println!("value other")
    }
}

fn generic_associated_function() {
    #[derive(Debug)]
    enum Opt<T> {
        Null,
        Value(T),
    }

    impl Opt<i32> {
        fn plus_one(self) -> Opt<i32> {
            match self {
                Opt::Null => Opt::Null,
                Opt::Value(value) => Opt::Value(value + 1)
            }
        }
    }

    let a = Opt::Value(0);
    let a = a.plus_one();

    // Rust是真泛型
    // let b = Opt::Value(str("abc"));
    // b.plus_one();

    println!("a = {:?}", a);
}

fn option_match() {
    let opt: Option<i32> = Option::Some(1);
    match opt {
        None => println!("none"),
        Some(value) => println!("some:{}", value)
    }
}

fn enum_match_with_value() {
    enum Match {
        First,
        Second(String),
    }

    let sec = Match::Second(str("abc"));
    match sec {
        Match::First => {
            println!("First")
        }
        Match::Second(info) => {
            println!("Second {}", info)
        }
    }
}

fn enum_with_argument() {
    #[derive(Debug)]
    enum Ip {
        V4(String),
        V6(String),
    }

    impl Ip {
        fn print(&self) {
            println!("{:?}", self)
        }
    }

    Ip::V4(str("192.168.0.1")).print()
}

fn enum_with_struct() {
    #[derive(Debug)]
    enum IpKind {
        V4,
        V6,
    }

    #[derive(Debug)]
    struct Address {
        kind: IpKind,
        info: String,
    }

    impl Address {
        fn new(kind: IpKind, info: String) -> Address {
            Address { kind, info }
        }
    }

    let address = Address::new(IpKind::V4, str("192.168.0.1"));

    println!("{:?}", address);
}

fn str(slice: &str) -> String {
    String::from(slice)
}



