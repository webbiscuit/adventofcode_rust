use std::io::{self, BufRead};

#[derive(Debug, PartialEq, Eq, PartialOrd)]
enum Signal {
    Value(i32),
    List(Vec<Signal>),
}

impl From<i32> for Signal {
    fn from(value: i32) -> Self {
        Signal::Value(value)
    }
}

impl From<Vec<i32>> for Signal {
    fn from(value: Vec<i32>) -> Self {
        Signal::List(value.into_iter().map(|v| v.into()).collect())
    }
}

// impl From<&str> for Signal {
//     fn from(value: &str) -> Self {
//         Signal::List(
//             value
//                 .chars()
//                 .map(|c| c.to_digit(10).unwrap() as i32)
//                 .map(|v| v.into())
//                 .collect(),
//         )
//     }
// }

// impl<T> From<Vec<Vec<T>>> for Signal
// where
//     Signal: From<Vec<T>>,
// {
//     fn from(value: Vec<Vec<T>>) -> Self {
//         Signal::List(value.into_iter().map(|v| v.into()).collect())
//     }
// }

impl Ord for Signal {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Signal::Value(a), Signal::Value(b)) => a.cmp(b),
            (Signal::List(a), Signal::List(b)) => a.cmp(b),
            _ => std::cmp::Ordering::Equal,
        }
    }
}

// impl PartialEq for Signal {
//     fn eq(&self, other: &Self) -> bool {
//         match (self, other) {
//             (Signal::Value(a), Signal::Value(b)) => a == b,
//             (Signal::List(a), Signal::List(b)) => a == b,
//             _ => false,
//         }
//     }
// }

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|l| l.unwrap()).collect::<Vec<_>>();

    let sum_of_indices = 13;

    println!("The sum of indices of pairs is {}.", sum_of_indices);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_compare() {
        // TODO writ estring parser for this
        assert!(Signal::from(vec![1, 1, 3, 1, 1]) < Signal::from(vec![1, 1, 5, 1, 1]));
    }

    #[test]
    fn test_list_of_list_compare() {
        let l1 = Signal::from(vec![1]);
        let l2 = Signal::from(vec![2, 3, 4]);
        // let l3 = Signal::from(vec![l1, l2]);
        //assert!(Signal::from(vec![Signal::from(vec![1])]) < Signal::from(vec![1, 1, 5, 1, 1]));
    }
}
