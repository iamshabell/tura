## Overview

A simple conceptual model inspired by the Turing machine. The Turing machine is a theoretical device that manipulates symbols on a strip of tape according to a set of rules and is a fundamental concept in computer science.
This program follows a similar principle, processing an input tape based on user-defined rules to perform state transitions until a halt condition is reached.

To learn more about Turing machines visit the [Wikipedia page on Turing machines](https://en.wikipedia.org/wiki/Turing_machine).

### Running the program

1. **Compile the program**:
   ```bash
   cargo build --release
   ```
2. **Run the program**:
   ```bash
   ./tura <source.tura> <input.tape>
   ```
   - `<source.tura>`: A file containing the rules for the machine
   - `<input.tape>`: It contains the initial tape configurations

### Example
Given the following rules in `source.tura`:
```javascript
Inc 0 1 R Halt
Inc 1 0 R Inc
```
And the following input tape in `input.tape`:
```
  1 1 1 1 0
```

Running the program will process the tape according to the rules and
output the tape's state at each step, showing the head's position with a `^` until
the machine reaches a halt state, like so:
```
Inc: 1 1 1 1 0
     ^
Inc: 0 1 1 1 0
       ^
Inc: 0 0 1 1 0
         ^
Inc: 0 0 0 1 0
           ^
Inc: 0 0 0 0 0
             ^
Halt: 0 0 0 0 1 0
                ^
```

## Acknowledgments

This project was developed while watching Tsoding's video series, [Turing Machine in Rust](https://www.youtube.com/playlist?list=PLpM-Dvs8t0VbbGLS6ISdZnUoOAIL8H4ET).
