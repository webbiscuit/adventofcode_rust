use std::error::Error;
use std::fmt;
use std::io::{self, prelude::*};

use assert_cmd::prelude::OutputOkExt;
use itertools::Itertools;

#[derive(Debug, Clone)]
enum Element{
    Empty,
    // Value((u8, u8)),
    Value(u8),
    Pair(Box<Pair>),
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        match self {
            // Element::Empty => (),
            Element::Value(v) => write!(f, "{}", v),
            Element::Pair(p) => write!(f, "[{},{}]", p.unwrap().0, p?.1),
        };
            
        

        // for &bit in self.message.iter() {
        //     write!(f, "{}", if bit { 1 } else { 0 })?;
        // }

        Ok(())
    }
}

impl Element {
        
}

#[derive(Debug, Clone)]
struct Pair {
    left: Element,
    right: Element
}

#[derive(Debug)]
struct SnailFishCalculator {
    head: Element,
}

impl SnailFishCalculator {
    fn new() -> Self { Self { head: Element::Empty } }

    fn add(&mut self, pair: (u8, u8))  {
        let (left, right) = pair;
        let new_pair = Pair { left: Element::Value(pair), right: Element::Value(right) };
        self.head = Element::Pair(Box::new(new_pair));
    }

    fn draw(&self) -> String {
        let output = self.draw_element(&self.head);
        output
    }

    fn draw_element(&self, element: &Element) -> String {
        match element {
            Element::Empty => "".to_string(),
            Element::Value(value) => value.to_string(),
            Element::Pair(ref pair) => {
                let left = self.draw_element(&pair.left);
                let right = self.draw_element(&pair.right);
                format!("[{},{}]", left, right)
            }
        }
    }



    // impl fmt::Display for SnailFishCalculator {
    //     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

    //         // for &bit in self.message.iter() {
    //         //     write!(f, "{}", if bit { 1 } else { 0 })?;
    //         // }
    
    //         Ok(())
    //     }
    // }
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // let hex = lines
    //     .next().unwrap()?;

    // println!("{}", hex);

    // let bits_system = BitsSystem::new(&hex);

    // println!("{}", bits_system);

    Ok(())
}


#[test]
fn check_super_simple_example() {
    let mut calculator = SnailFishCalculator::new();

    calculator.add((1, 1));
    calculator.add((2, 2));
    calculator.add((3, 3));
    calculator.add((4, 4));

    println!("{:?}", calculator);
    assert_eq!(calculator.draw(), "[[[[1,1],[2,2]],[3,3]],[4,4]]");

}