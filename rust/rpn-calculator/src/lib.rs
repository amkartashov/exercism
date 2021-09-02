#[derive(Clone, Copy, Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack = Vec::new();
    for &input in inputs {
        match input {
            CalculatorInput::Value(x) => stack.push(x),
            input => {
                let op2 = stack.pop()?;
                let op1 = stack.pop()?;
                match input {
                    CalculatorInput::Add => stack.push(op1 + op2),
                    CalculatorInput::Subtract => stack.push(op1 - op2),
                    CalculatorInput::Multiply => stack.push(op1 * op2),
                    CalculatorInput::Divide => stack.push(op1 / op2),
                    CalculatorInput::Value(_) => unreachable!(),
                }
            }
        };
    }

    if stack.len() != 1 {
        None
    } else {
        stack.pop()
    }
}
