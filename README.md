# tree-sitter to txl

Tree-Sitter parser generates abstract syntax trees (AST) from Rust code and the
tree nodes can be traced back to the lineno and column number of the original
source, making it possible to relate them to the interal AST structures of
Rust/Clang compilers.  However, it is not as easy to perform transformations on
these Tree-Sitter ASTs using a transformation system such as TXL.

TXL parser generates a parsing tree from Rust code, where these tree nodes can
only be traced back to the linenos in code. It is hard to accurately relate to
the semantics information computed by the compilers because of missing column
numbers. It is also known by its design that such column numbers cannot be
stored.

Therefore, it is the best of the two worlds by replacing TXL's parsing trees
with tree-sitter's ASTs while retaining TXL's capability of transformations.

In order to use tree-sitter to parse Rust code into ASTs, then use TXL to
perform transformations on the ASTs, we need to adapting one of them.

In this project, we aim to do three types of adaptations:

1. Generating a TXL grammar from a Tree-Sitter JSON grammar;

2. Creating a TXL grammar for Tree-Sitter S-expressions, then adapt the TXL grammar
   according to Tree-Sitter's JSON grammar.

3. Creating a TXL grammar for Tree-Sitter's XML markups, then adapt the TXL grammar
   according to Tree-Sitter's JSON grammar.

## Option 1 is more direct. 
It creates a TXL grammar equivalent to the Tree-Sitter grammar, while making it
easier to relate each other. However, since TXL grammar is a parsing tree, it
does not have the same freedom of introducing nested sequences into the
production rules.  In other words, some refactoring of the production rules is
necessary. In terms of transformations, tokens can be kept as part of the
concrete syntax, which guarantees that all the intermediate results complies to
the TXL and Tree-Sitter grammar. 

*Note* At the moment, we have not yet achieved the goal automatically :-(
Although we have prepared the production rules, the token rules still need more
work.

*Prototype* `src/main.rs` does the transformation by converting
`examples/rust.json` into `rust1.grm` and `rust-seq1.grm` respectively. 

## Option 2 is incremental. 

It first introduces a tolerating grammar of S-expressions, where every tree
node can have arbitrary number of subtree nodes. While consulting the actual
product rules in the Tree-Sitter's JSON grammar, it is possible to restrict the
number of subtree types and number of subtrees accordingly. This is
corresponding to the redefinition of production rules in TXL. As the adapted
grammar is evolving, it may check the compliance of the evolved tree to the AST
structure.

*Prototype* `tree-sitter-hover.grm` refines `tree-sitter.grm` to parse the same
S-expressions into different AST's. Use `make` to see the effects, and use `txl
-x` option to print out different XML tags to debug the AST's.

*Solution* `src/main.rs` does the transformation by converting
`examples/rust.json` into `rust2.grm` which refines `tree-sitter.grm`. 

```bash
cargo test
```

*Note* Since the tokens of concrete syntax have been discarded, transforming
the AST's may result in correct AST but incorrect code. Hence, we will need to
use the Tree-Sitter's editing capability to ensure that the transformed code
conforms to the syntax rules when there is no error, and the reparsed AST's is
the same as the transformed AST. Otherwise, we will reject the transformation
and print a warning.

## Option 3 is less ambigious. 

The S-expressions neglect all concrete tokens, which can make the parser
ambigious to parse them,  because the same S-expression might call for
different production rules due to missing tokens in the concrete syntax.

Therefore, another way is to take into account of the concrete tokens
in-between the AST's node types in order to disambigue multiple applicable
production rules. 

*Prototype* `tree-sitter-xml-hover.grm` refines `XML/Txl/XML.Grammar` to parse the same
XML-based AST's into different AST's. Use `make` to see the effects, and use `txl
-x` option to print out different XML tags to debug the AST's.

In the following, the two grammars have no difference at the token level.
```bash
txl -q -s 2000 -in 0 -i XML/Txl t.xtst -o t.1
txl -q -s 2000 -in 0 -i XML/Txl XML/Txl/XML.Txl t.xtst -o t.2
diff t.1 t.2
```

However, in the following, the two grammars are different in terms of the recognition of
additional AST related production rules:
```bash
txl -q -s 2000 -in 0 -i XML/Txl t.xtst -x -o t.1
txl -q -s 2000 -in 0 -i XML/Txl XML/Txl/XML.Txl t.xtst -x -o t.2
diff t.1 t.2
```

In this way, we may generate such adapted AST rules from a tree-sitter's JSON grammar,
using a similar approach to Option 2. 

