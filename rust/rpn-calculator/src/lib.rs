#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = Vec::new();
    if !inputs.is_empty() {
        for el in inputs {
            let x: i32;
            let y: i32;
            match el {
                CalculatorInput::Value(x) => stack.push(*x),
                _ => {
                    if stack.len() >= 2 {
                        y = stack.pop().unwrap();
                        x = stack.pop().unwrap();
                        match el {
                            CalculatorInput::Add => { stack.push(x + y) },
                            CalculatorInput::Subtract => { stack.push(x - y) },
                            CalculatorInput::Multiply => { stack.push(x * y) },
                            CalculatorInput::Divide => { stack.push(x / y) },
                            _ => return None
                        }
                    } else {
                        return None
                    }
                }
            }
        }
    }
    if !stack.get(0).is_none() && stack.len() == 1 {
        return Some(stack[0])
    } else {
        return None
    }
}
