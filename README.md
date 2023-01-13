# Faster-Pest

Welcome to `faster-pest`, a **high-performance** code generator for [Parsing Expression Grammars](https://pest.rs/book/grammars/peg.html). `faster-pest` is an unofficial pro-macro providing next-level implementations of [Pest](https://pest.rs/) parsers. It uses low-level optimization tricks under the hood to generate highly optimized code which minimizes the overhead of the AST recognition process, resulting in much faster parsing.

`faster-pest` is **compatible** with the standard [Pest syntax](https://pest.rs/book/grammars/syntax.html), so you can easily switch to it without having to change your existing grammar. It also provides detailed performance metrics and debugging information, so you can easily identify and optimize bottlenecks in your parsers.

With `faster-pest`, you can enjoy the **convenience and expressiveness** of Pest while getting the performance of a low-level parsing library. It's the perfect choice for large-scale or performance-critical parsing projects. Give it a try and experience the difference for yourself!

The parsing approach used under the hood has nothing in common with the original pest code. To be honest, I never looked at the pest codebase, because it was easier to start from scratch. There is still one thing that was not reimplemented: the parsing of the actual pest grammar. However, this might not last. I need to extend the grammar to enable more advanced tricks, like making it possible to define complex rules with Rust code and import them in a pest grammar.

## Benchmarks

Only a week after its creation, `faster-pest` already parses Json at **819%** the speed of Pest and **183%** the speed of Nom. This places `faster-pest` on par with `serde_json`. `faster-pest` allows you to approach limits that only SIMD-powered parsers can overcome.

[Benchmark yourself](https://github.com/Mubelotix/pestvsnom)

## Examples

See the [example folder](https://github.com/Mubelotix/faster-pest/tree/master/faster-pest/examples) for examples.

It contains two examples from the Pest book: [csv](https://github.com/Mubelotix/faster-pest/tree/master/faster-pest/examples/csv) and [ini](https://github.com/Mubelotix/faster-pest/tree/master/faster-pest/examples/ini).  
These use the exact same code as in the Pest book, showing that `faster-pest` is a drop-in replacement for Pest.

If you don't have any legacy Pest codebases, it is recommended to not use the pest compatibility layer. See other two examples: [json](https://github.com/Mubelotix/faster-pest/tree/master/faster-pest/examples/json) and [po](https://github.com/Mubelotix/faster-pest/tree/master/faster-pest/examples/po).
These are the most efficient and idiomatic uses of `faster-pest`. They work rather similarly to the pest compatibility layer, but their implementation is nicer.

## Limitations

`faster-pest` is still in its early stages of development, so it has some limitations. Here are the most important ones:

- Limited syntax support (Missing: stack, insens, range, pospred)
- The tokens API of Pest is not supported (you probably didn't use that)
- Error printing is made for Linux
- Errors can be obscure when a repetition ends prematurely
- Not everything has been tested and there could be incorrect parsing behavior

## Optimization tricks used (for curious people)

- `faster-pest` generates two versions of every parsing component that exists. One version has error support, the other doesn't. There are so many places where error support is not needed because it would be discarded rightaway (like a failing branch). `faster-pest` will only retrieve errors if parsing completely fails, so any valid input will only result in calls of completely error-unaware code. From the developer point of view, this optimization is completely transparent.
- Groups of rules are sometimes grouped into a single rule where pest would have split them
- Repetitions of simple character rules use iterator adapters instead of loops
- Every unnecessary check is bypassed
- Allocations are made in bulk which makes them fairly sporadic
- Code is so small it is likely to get inlined often by the compiler
- Parsing itself is entirely zero-copy
- Iteration over parsed identifiers is almost free
