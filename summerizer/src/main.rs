fn main() {
    println!("Hello, world!");
    // create new summerizer instance
    let summerizer = summerizer::new_summerizer().unwrap();
    // summarize text
    let text = "As we approach 2025, the software development landscape continues to evolve at a rapid pace. Among the programming languages that have gained significant traction in recent years, Rust stands out as a powerful and versatile option that offers numerous benefits for developers.

Created by Mozilla Research, Rust has quickly become a favorite among programmers for its unique combination of performance, safety, and concurrency, ranking as the top most desired language for subsequent years now in Stack Overflow surveys.

In this comprehensive guide, we’ll explore the many advantages of Rust and why it’s becoming an essential language for developers in 2025.

A Quick Note on Learning Rust
Rust is not an easy language to master, especially if you’ve been in “high programming language land” most of your career.

Time well spent in The Rust Book is a non-negotiable.

Here is my blueprint for learning Rust successfully.

Why Developers Should Embrace Rust in 2025
1. Memory Safety Without Sacrificing Performance
One of Rust’s most significant selling points is its ability to provide memory safety without compromising on performance. This is achieved through Rust’s ownership system and borrowing rules, which prevent common programming errors such as null or dangling pointer references, buffer overflows, and data races at compile time.

Example: s1 is moved into s2 (further explanation)

Example of Rust borrowing
Unlike languages that rely on garbage collection (like Java or Python) or manual memory management (like C or C++), Rust’s ownership model ensures that memory is managed efficiently and safely without runtime overhead.

This results in programs that are both fast and reliable, making Rust an excellent choice for systems programming, game development, and other performance-critical applications.

2. Concurrency Without Data Races
In an era where multi-core processors are the norm, writing concurrent code that is both efficient and safe is crucial. Rust’s ownership and type systems guarantee thread safety, eliminating data races by design. This allows developers to write concurrent code with confidence, knowing that the compiler will catch potential issues before they become runtime problems.

Rust’s approach to concurrency is not only safe but also flexible. It supports various concurrency models, including message-passing, shared-state concurrency, and async/await syntax for handling asynchronous operations.

This versatility makes Rust an excellent choice for building scalable, high-performance server applications and distributed systems.

3. Cross-Platform Development
As the tech ecosystem becomes increasingly diverse, the ability to write code that runs on multiple platforms is more valuable than ever.

Rust excels in this area, offering excellent support for cross-platform development. With Rust, you can write code that compiles natively for a wide range of platforms, including Windows, macOS, Linux, iOS, Android, and even WebAssembly.";
    let summary = summerizer::summarize_text(&summerizer, text).unwrap();
    println!("Summary: {}", summary);
}
