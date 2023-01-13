# Faster-Pest

Welcome to `faster-pest`, a **high-performance** code generator for [Parsing Expression Grammars](https://pest.rs/book/grammars/peg.html). `faster-pest` is an unofficial pro-macro providing next-level implementations of `pest` parsers. It uses low-level optimization tricks under the hood to generate highly optimized code which minimizes the overhead of the AST recognition process, resulting in much faster parsing.

`faster-pest` is **compatible** with the standard Pest syntax, so you can easily switch to it without having to change your existing grammar. It also provides detailed performance metrics and debugging information, so you can easily identify and optimize bottlenecks in your parsers.

With `faster-pest`, you can enjoy the **convenience and expressiveness** of Pest while getting the performance of a low-level parsing library. It's the perfect choice for large-scale or performance-critical parsing projects. Give it a try and experience the difference for yourself!

## Benchmarks

Only a week after its creation, `faster-pest` already parses Json at **819%** the speed of Pest and **183%** the speed of Nom. This places `faster-pest` on par with `serde_json`. `faster-pest` allows you to approach limits that only SIMD-powered parsers can overcome.

[Benchmark yourself](https://github.com/Mubelotix/pestvsnom)

## Limitations

`faster-pest` is still in its early stages of development, so it has some limitations. Here are the most important ones:

- Limited syntax support (Missing: stack, insens, range, pospred)
- The tokens API of Pest is not supported (you probably didn't use that)
- Error printing is made for Linux
- Errors can be obscure when a repetition ends prematurely
- Not everything has been tested and there could be incorrect parsing behavior

## Optimization tricks used (for curious people)

- `faster-pest` generates two versions of every parsing component that exists. One version has error support, the other doesn't. There are so many places where error support is not needed because it would be discarded rightaway (like a failing branch). `faster-pest` will only retrieve errors if parsing completely fails, so any valid input will only result in calls of completely error-unaware code. From the developer point of view, this optimization is completely transparent.
- Groups of rules are sometimes grouped into a single rule
- Repetitions of simple character rules use iterator adapters instead of loops
- Every unnecessary check is bypassed
- Allocations are made in bulk which makes them fairly sporadic
- Code is so small it is likely to get inlined often by the compiler
- Parsing itself is entirely zero-copy
- Iteration over parsed identifiers is almost free
