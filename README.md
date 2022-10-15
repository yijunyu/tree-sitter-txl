# tree-sitter to txl

Tree-Sitter parser generates abstract syntax trees (AST) from Rust code
and the tree nodes can be traced back to the lineno and column numbers of
the original source, making it possible to relate them to the interal AST
structures of Rust/Clang compilers. 
However, it is not as easy to perform transformations on the Tree-Sitter
structures using a transformation system such as TXL.

TXL parser generates parsing tree from Rust code, these tree nodes can only be
traced back to linenos in code. Therefore, it is ideal to replace the parsing
trees with the ASTs while retaining the capability of transformations.

In order to use tree-sitter to parse Rust code into ASTs, then use TXL to
perform transformations on the ASTs, we need to create adapting one of them.

In this project, we aim to do two types of adaptations:

1. Generating TXL grammar from a Tree-Sitter JSON grammar;

2. Creating a TXL grammar for Tree-Sitter S-expressions, then adapt the TXL grammar
   according to Tree-Sitter's JSON grammar.

Approach 1 is more direct. It creates an equivalent TXL grammar of the
Tree-Sitter grammar, while it may be easier to relate each other. However,
since TXL grammar is a parsing tree, it does not have the same freedom of
introducing nested sequences into the production rules.  In other words, some
refactoring of the production rules are necessary. In terms of transformation,
the tokens are kept as part of the concrete syntax and may guarantee that all
the intermediate results complies to the grammar. 

Noted. At the moment, we have not yet achieve the goal through automation. The
tokens and immediate tokens still need some more work.

Solution: `src/main.rs` does the transformation by converting `examples/rust.json` into `rust.grm`
and `rust-seq.grm`. 

Approach 2 is incremental. It first introduces a tolerating grammar of the S-expressions,
where every tree node can have arbitrary number of subtree nodes. While consulting the real
product rules in the Tree-Sitter's JSON grammar, it is possible to restrict the number of 
subtree types and number of subtrees accordingly. This is corresponding to the redefinition
of production rules in TXL. As the adapted grammar is evolving, it may check the compliance of
the evolved tree to the AST structure.

Prototype: `tree-sitter-hover.grm` refines `tree-sitter.grm` so that it can parse the same S-expressions
into different AST's. Use `make` to see the effects, and use `txl -x` option to print out different XML
tags to debug the AST's.

Solution: `src/main.rs` does the transformation by converting `examples/rust.json` into `rust.grm`
which refines `tree-sitter.grm`. 

Note. Since the tokens of concrete syntax have been discarded, transforming the AST's may result in
correct AST but incorrect code. Hence, we will need to use the Tree-Sitter's editing capability
to check the transformed code conforms to the syntax when there is no error, and the reparsed
AST's is the same as the transformed AST.
