a language must have a root sum type with which it is associated

language combinators:

- [ ] join over a single sum type (e.g., to augment a language that defines addition and multiplication with exponentiation, or to go from the language to a language that has strictly more sum types)
  - [ ] do not duplicate identical product types that are in common. They are identical if they can be traced back to the same definition
  - [ ] should come with reduct operations that take an expression (and a proof of success?) and give something in either language
  - [ ] should come with reduct operations that take an expression and a sum type that both languages have and infallibly go to the errorful version of the reduct of the sum type to either language
  - [ ] a CheaplyInto operation for the first of the two joined languages
  - [ ] an ExpensivelyInto operation for the second of the two languages
  - [ ] a CheaplyInto operation for a copy of the two languages, and a Bijection impl between the second of the two languages and the copy
- [ ] join over multiple sum types (e.g., to do the above, except in a language that already has overlap and needs to avoid redundancy)
  - [ ] should come with reduct operations (see above)
- [ ] augment all sum types with a language (e.g., the language of errors)
- [ ] associating a special element, Default, with a sum type (e.g., to ensure existence of a morphism)
- [ ] augment all product types with outgoing morphisms to a type (e.g., the language of spans)
  - [ ] if the added language implements Default, this should come with a morphism from the original language to the expanded language, and it should preserve patterns
  - [ ] if it doesn't implement Default, then patterns that do not give every created node a source node will break (require all patterns to assign created nodes to source nodes?)
- [ ] augment all product types appearing in a given sum type with outgoing morphisms to a type?????
- [ ] augment specific product types to outgoing morphisms to a type??? this will get really complex for lifting patterns if the type is not Default
- [ ] remove some sum or product types from a language, and all the morphisms that go into them

  - [ ] comes with a fallible reduct operation
  - [ ] and embedding operation

- [ ] make morphisms into sum types anonymous? that would prohibit duplicates. Should duplicates be prohibited? Yes, they should.
- [ ] require a product type to be used in at most one sum type? No, too restrictive.
- [ ] require outgoing morphisms of product types to have names? No, names can be provided
- [ ] require all product types to be variants of a sum type?

patterns:

- [ ] map from one tree to another if enabled
- [ ] are endofunctions of a language
- [ ] can have variables in them representing subtrees
- [ ] the variables can be quantified over using bounded quantifiers and can have their triggers be combined with boolean expressions
- [ ] are compatible with product augmentation if they associate with each node in the output a node (of the same sum type? too restrictive) in the input, or if augmentation is done by sth that implements Default

a langspec defines a group of languages that are trivially in bijection with each other and it has a function that produces its canonical language. it also has a function that returns the rust code that defines the trait that patterns on that language should use

implementation for language:

- pub fn join_over(self, other, vector of type names) { }
- pub fn sum_broadcast(self, other) { add other as a variant of every sum type. requires that other is not already part of every sum type because the outgoing maps of sum types are anonymous. this can be ensured by newtyping other }
- include impls of Default for specific sum types as part of the langspec
- pub fn prod_broadcast(self, other) { add other as a field of every product type. }
- pub fn exclude(self, other, vector of type names to remove) { }

these all return languages that have a special type that has a generic implementation including functions that can return instances of type Map<U, T> where U is the root type of one language and T is the root type of the other. languages must explicitly define their root type.

To improve error messages, it should also be possible to forget how a language was produced by converting it to the canonical language of a fresh langspec

the maps can be chained into a well-typed execution plan.
