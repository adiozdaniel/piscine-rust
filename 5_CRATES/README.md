# Variables

Welcome to the Rust Piscine! This repository contains a series of exercises designed to help you get hands-on experience with Rust programming. Each exercise will help you understand various core concepts of the language, including basic syntax, data structures, algorithms, and more.

## Table of Contents

- [Overview](#overview)
- [Exercises](#exercises)
  - [groceries](#groceries)
  - [reverse_string](#reverse_string)
  - [find_factorial](#find_factorial)
  - [fibonacci2](#fibonacci2)
  - [matrix_transposition](#matrix_transposition)
  - [division_and_remainder](#division_and_remainder)
  - [tuples_refs](#tuples_refs)
- [Notions](#notions)
- [Contributing](#contributing)

## Overview

The Rust Piscine is a collection of exercises designed to help you build and strengthen your understanding of the Rust programming language. Each exercise includes:

- **Instructions**: Describes the problem and what the function should accomplish.
- **Usage**: Provides example code to test your solution.
- **Notions**: Key concepts covered by the exercise.

The exercises span multiple levels, each becoming more challenging as you progress. The final goal is to deepen your understanding of Rust and its capabilities.

## Exercises

### groceries

This exercise involves working with a vector of strings (grocery items). You will write two functions: one to insert a new item at the end of the vector and another to return an item at a specific index.

#### Functions:

- `insert`: Adds a new element at the end of the vector.
- `at_index`: Returns the value at the given index.

### reverse_string

In this exercise, you'll implement a function that takes a string and returns its reversed version.

#### Function:

- `rev_str`: Reverses the string passed as an argument.

### find_factorial

This exercise is about calculating the factorial of a given number. The factorial of a number is the product of all integers from 1 up to that number.

#### Function:

- `factorial`: Returns the factorial of a given number.

### fibonacci2

Here, you will calculate the nth number in the Fibonacci series. The Fibonacci series starts with 0 and 1, with each subsequent number being the sum of the two preceding ones.

#### Function:

- `fibonacci`: Returns the nth Fibonacci number.

### matrix_transposition

In this exercise, you will work with matrices, specifically with a 2x2 matrix. You will create a function that calculates the transposition of a matrix, swapping its rows and columns.

#### Function:

- `transpose`: Returns the transpose of a 2x2 matrix.

### division_and_remainder

This exercise asks you to write a function that divides two integers and returns both the division result and the remainder.

#### Function:

- `divide`: Returns a tuple with the result of the division and the remainder.

### tuples_refs

In this exercise, you'll define a tuple struct to represent a student. The struct will store the student's ID, first name, and last name. Then, you will write functions to return each piece of information.

#### Functions:

- `id`: Returns the student's ID.
- `first_name`: Returns the student's first name.
- `last_name`: Returns the student's last name.

## Notions

Throughout these exercises, you'll encounter the following Rust notions:

- **Primitives**: Basic types like integers, floats, and characters.
- **Functions**: Defining and using functions, passing arguments, and returning values.
- **Tuples**: Working with tuples and tuple structs to store multiple values.
- **Vectors**: Understanding the use of `Vec` for dynamic arrays.
- **Structs**: Defining and using structs, including tuple structs.
- **Debugging**: Using derived traits like `Debug`, `PartialEq`, and `Eq` to make structs easier to inspect.
- **Algorithms**: Implementing basic algorithms such as calculating factorials and Fibonacci numbers.

## Contributing

If you want to contribute to this project, feel free to fork it and submit a pull request. Make sure to follow the style guide and test your changes before submitting.

Happy coding, and good luck with the exercises!
