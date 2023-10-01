use crate::common::{number_literal_parser::{number_literal_to_u32, number_literal_to_u16, is_number_literal_u16}, register_parser::{is_register_name, register_name_to_u16, register_name_to_u32}};

use super::node::Node;
use std::collections::HashMap;

pub struct Evaluator {

}

impl Evaluator {

    pub fn new() -> Evaluator {
        Evaluator {
            // labels: HashMap::new(),
            // encoding_success: true,
        }
    }

    pub fn evaluate(&mut self, symbol_table : &HashMap<String, u32>, expression: &Option<Box<Node<String>>>) -> u32 {
        println!("evaluate {:?}", expression);

        // if the option is empty, return 0
        if expression.is_none() 
        {
            return u32::default();
        }

        // retrieve value from option
        let expr = expression.clone().unwrap();

        // if the expression contains a direct value and is not an operator, return that value
        if !expr.expression {

            if is_register_name(&expr.value)
            {
                return register_name_to_u32(&expr.value);
            }

            if is_number_literal_u16(&expr.value)
            {
                return number_literal_to_u32(&expr.value);
            }

            let symbol_table_value: &u32 = symbol_table.get(&expr.value).unwrap();
            return *symbol_table_value;
        }

        match expr.value.as_str() {

            "LOW" => {
                let evaluated_value: u32 = self.evaluate(symbol_table, &expr.left);
                let low_value: u32 = crate::LOW_LOW_U32!(evaluated_value);
                return low_value;
            }

            "HIGH" => {
                let evaluated_value: u32 = self.evaluate(symbol_table, &expr.left);
                let low_value: u32 = crate::LOW_U32!(evaluated_value);
                return low_value;
            }

            "*" => {
                let lhs_evaluated_value: u32 = self.evaluate(symbol_table, &expr.left);
                let rhs_evaluated_value: u32 = self.evaluate(symbol_table, &expr.right);
                return lhs_evaluated_value * rhs_evaluated_value;
            }

            "/" => {
                let lhs_evaluated_value: u32 = self.evaluate(symbol_table, &expr.left);
                let rhs_evaluated_value: u32 = self.evaluate(symbol_table, &expr.right);
                return lhs_evaluated_value / rhs_evaluated_value;
            }

            "+" => {
                let lhs_evaluated_value: u32 = self.evaluate(symbol_table, &expr.left);
                let rhs_evaluated_value: u32 = self.evaluate(symbol_table, &expr.right);
                return lhs_evaluated_value + rhs_evaluated_value;
            }

            "-" => {
                let lhs_evaluated_value: u32 = self.evaluate(symbol_table, &expr.left);
                let rhs_evaluated_value: u32 = self.evaluate(symbol_table, &expr.right);
                return lhs_evaluated_value - rhs_evaluated_value;
            }

            "<<" => {
                let lhs_evaluated_value: u32 = self.evaluate(symbol_table, &expr.left);
                let rhs_evaluated_value: u32 = self.evaluate(symbol_table, &expr.right);
                return lhs_evaluated_value << rhs_evaluated_value;
            }

            ">>" => {
                let lhs_evaluated_value: u32 = self.evaluate(symbol_table, &expr.left);
                let rhs_evaluated_value: u32 = self.evaluate(symbol_table, &expr.right);
                return lhs_evaluated_value >> rhs_evaluated_value;
            }

            ">" => {
                panic!("Not implemented yet!")
            }

            "<" => {
                panic!("Not implemented yet!")
            }

            _ => { /*panic!("Unknown!") */ },

        }

        u32::default()

        // Option::expect(expression.left(), "no node in expression");

        // if let Some(Node) = expression.left() {

        // }

        // if (expression.get().left == None && expression.right == None) 
        // {

        // }
    }

}