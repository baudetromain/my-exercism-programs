#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = vec!();

    for input in inputs
    {
        match input
        {
            CalculatorInput::Value(value) => stack.push(value.to_owned()),
            _ => {
                if stack.len() < 2
                {
                    return None;
                }
                else
                {
                    let operands = (stack.pop().unwrap(), stack.pop().unwrap());
                    match input
                    {
                        CalculatorInput::Add => stack.push(operands.1 + operands.0),
                        CalculatorInput::Subtract => stack.push(operands.1 - operands.0),
                        CalculatorInput::Divide => stack.push(operands.1 / operands.0),
                        CalculatorInput::Multiply => stack.push(operands.1 * operands.0),
                        _ => {}
                    }
                }
            }
        };
    }

    match stack.len()
    {
        1 => Some(stack[0].to_owned()),
        _ => None
    }
}
