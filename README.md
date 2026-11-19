# Helium

**Helium** is an experimental programming language built from the ground up, with its compiler written in Rust.

Helium combines dynamic typing with explicitly typed function signatures, allowing developers to write flexible code while maintaining clear interfaces between functions.

The compiler generates native x86-64 assembly, giving Helium a direct path from source code to machine code.

> **Note:** Helium is currently under development. Syntax, features, and compiler internals are subject to change.

## Example

```helium
function add(dec a : Integer, dec b : Integer) -> Integer
{
    return a + b;
}

function main()
{
    dec result = add(10, 20);
    print(result);
    return 0;
}
```

## Language Syntax

### Variables

Variables are declared using `dec`.

```helium
dec name = "Helium";
dec age = 10;
dec ch = 'a';
```

### Functions

Functions are declared using the `function` keyword.

Each parameter must have an explicitly declared type. Functions that return a value must also specify their return type using `->`.

```helium
function add(dec a : Integer, dec b : Integer) -> Integer
{
    return a + b;
}

function greet(dec name : String)
{
    print("Hello, ");
    print(name);
}
```

### Classes

Helium supports classes, fields, and methods.

```helium
class Player
{
    dec health : Integer;

    function take_damage(dec amount : Integer, dec this : Player)
    {
        this.health = this.health - amount;
    }
}
```

The `this` reference is explicitly passed as a parameter to methods.

### Arrays

Arrays are created by specifying their element type and size.

```helium
dec size = 10;
dec numbers = [Integer, size];
```

or by passing the values as a list.

```helium
dec numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
```

### Imports

The `fetch` keyword is used to import libraries.

```helium
fetch <std_lib>
```

## Typing System

Helium uses dynamic typing for variables and values, while requiring explicit type declarations for function parameters and return values.

This provides flexibility when working with variables while keeping function interfaces explicit.

For example:

```helium
function multiply(dec a : Integer, dec b : Integer) -> Integer
{
    return a * b;
}
```

## Compiler

The Helium compiler is written in Rust and currently targets Linux x86-64.

The compilation process consists of several stages:

1. **Lexical Analysis** – Converts source code into tokens.
2. **Parsing** – Checks for valid syntax and constructs the AST.
3. **Code Generation** - Generates the x86-64 assembly.


## Building & Installation

### Requirements
- Rust and Cargo
- GNU Binutils (`as`, `ld`)
- Make

### Build

Build the compiler:

    make compiler

Build the standard libraries:

    make libraries

Or build both:

    make

### Install

Install the compiler and built-in libraries:

    sudo make install

By default, Helium is installed to `/usr/local/helium`.

You can change the installation directory:

    sudo make install PREFIX=/your/custom/path