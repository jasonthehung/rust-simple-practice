use std::str::FromStr;

struct Equation {
    left: String,
    right: String,
    is_bool: bool,
}

struct ParseEquationError;

impl FromStr for Equation {
    type Err = ParseEquationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_once('=')
            .and_then(|(l, r)| {
                let left = sum(l);
                let right = sum(r);

                Some(Equation {
                    left: l.trim().to_string(),
                    right: r.trim().to_string(),
                    is_bool: left == right,
                })
            })
            .ok_or(ParseEquationError)
    }
}

fn sum(s: &str) -> usize {
    s.split('+')
        .map(|x| x.trim().parse::<usize>().unwrap_or(0))
        .sum()
}

fn test_eq(s: &str) {
    match s.parse::<Equation>() {
        Ok(Equation {
            left,
            right,
            is_bool,
        }) => {
            if is_bool {
                println!("{left} does equal {right}");
            } else {
                println!("{left} does not equal {right}");
            }
        }
        Err(_) => println!("boom!"),
    }
}

fn main() {
    test_eq("1 + 2 + 3 + 4 = 10");
    test_eq("10 + 1 = 11");
    test_eq("2 + 4 = 4 + 3");
}
