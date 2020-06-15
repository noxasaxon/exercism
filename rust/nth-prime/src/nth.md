Hi @noxasaxon,

Good work. I like that you factored out is_prime. Here are my thoughts about your solution.

1. Your code could be faster. You don't need to check factors all the way to n to determine if n is prime.
2. The use of loops and mutable vars in this exercise makes code that is less idiomatic Rust; in general, Rust programmers consider state to be risky and prefer to manage it with well-tested abstractions like iterators when necessary. Because of this, a functional solution may read more cleanly. If you are interested in using functional Rust to solve this one, the basic functional idiom is iterator().adapter().adapter()...consumer(). First work on is_prime. The iterator in this case can be a Range: (2..=limit_of_divisors), now we need a consumer that returns a bool and checks each of the factors coming from the iterator. Look at any and all. Can you make them work?
3. Now, take a look at nth. Here, for an adapter, think about using filter with a call to is_prime.