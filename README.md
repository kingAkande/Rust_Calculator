# Rust Calculator

A simple calculator written in Rust that supports basic arithmetic operations: addition, subtraction, multiplication, and division.

## Features

- **Addition (+)**: Add two numbers.
- **Subtraction (-)**: Subtract one number from another.
- **Multiplication (*)**: Multiply two numbers.
- **Division (/)**: Divide one number by another.

## Getting Started

1. **Prerequisites**:
   - Make sure you have Rust installed. If not, you can download it from the [official Rust website](https://www.rust-lang.org/).

2. **Clone the Repository**:
   ```bash
   git clone https://github.com/your-username/rust-calculator.git
   cd rust-calculator
   ```

3. **Build and Run**:
   ```bash
   cargo build
   cargo run
   ```

4. **Usage**:
   - Call the `calc` function passing in any of the operators ("sum," "sub," "mul," or "div") and the corresponding numbers as arguments.
   - For example:
     - To add: `calc(10, 5, "sum")`
     - To subtract: `calc(15, 7, "sub")`
     - To multiply: `calc(3, 4, "mul")`
     - To divide: `calc(20, 4, "div")`

## Examples

- **Addition**:
  ```
  calc(10, 5, "sum") = 15
  ```

- **Subtraction**:
  ```
  calc(15, 7, "sub") = 8
  ```

- **Multiplication**:
  ```
  calc(3, 4, "mul") = 12
  ```

- **Division**:
  ```
  calc(20, 4, "div") = 5
  ```

## Contributing

Contributions are welcome! If you find any issues or want to add new features, feel free to open a pull request.
