use crate::common::number_literal_parser::number_literal_to_u32;

use super::node::Node;

pub struct Evaluator {

}

impl Evaluator {

    pub fn new() -> Evaluator {
        Evaluator {
            // labels: HashMap::new(),
            // encoding_success: true,
        }
    }

    pub fn evaluate(&mut self, expression: Option<Box<Node<String>>>) -> u32 {
        println!("evaluate {:?}", expression);

        // if the option is empty, return 0
        if expression.is_none() 
        {
            return 0u32;
        }

        // retrieve value from option
        let expr = expression.unwrap();

        // if the expression contains a direct value and is not an operator, return that value
        if !expr.expression {
            //return expr.value.parse::<u32>().unwrap();
            return number_literal_to_u32(&expr.value);
        }

        0u32

        // Option::expect(expression.left(), "no node in expression");

        // if let Some(Node) = expression.left() {

        // }

        // if (expression.get().left == None && expression.right == None) 
        // {

        // }
    }

}