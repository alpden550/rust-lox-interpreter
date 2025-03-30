# rlox - Lox Language Interpreter in Rust

## Description

`rlox` is an implementation of the Lox programming language interpreter in Rust. The project includes a complete interpreter with support for:

- Lexical analysis and parsing
- Expression and statement execution
- Functions and closures
- Variables and scopes
- Control structures (if/else, loops)
- Built-in functions

## Lox Language Features

- Dynamic typing
- First-class functions and closures
- Control structures (if/else, while, for)
- Built-in `clock()` function for measuring execution time


## Examples

### Closures

```lox
fun makeCounter() {
  var i = 0;
  fun count() {
    i = i + 1;
    print i;
  }

  return count;
}

var counter = makeCounter();
counter(); // "1".
counter(); // "2".
counter(); // "3".
counter(); // "4".
counter(); // "5".
```

### Fibonacci

```lox
var started = clock();

fun fib(n) {
  if (n <= 1) return n;
  return fib(n - 2) + fib(n - 1);
}

for (var i = 0; i < 25; i = i + 1) {
  print fib(i);
}

print "Execution time: " + (clock() - started) + "seconds";
```

## Running

```bash
# Execute a script
cargo run -- examples/fib.rlox
or 
./rlox.sh examples/fib.rlox

cargo run -- examples/closure.rlox
or 
./rlox.sh examples/closure.rlox
```

## Project Structure

- `src/` - interpreter source code
- `examples/` - example Lox language programs
