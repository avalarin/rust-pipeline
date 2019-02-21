# Simple pipeline (middleware) written on Rust

Usage:
```rust
fn main() {
    let pipeline = PipelineBuilder::new()
        .next(Arc::new(a))
        .next(Arc::new(b))
        .build();

    let _ = pipeline.call(PipelineContext{});
}

fn a(context: PipelineContext, next: Arc<Pipeline>) -> Result<PipelineContext, Box<Error>> {
    println!("Begin A");
    let result = next.call(context);
    println!("Complete A");
    return result;
}

fn b(context: PipelineContext, next: Arc<Pipeline>) -> Result<PipelineContext, Box<Error>> {
    println!("Begin B");
    let result = next.call(context);
    println!("Complete B");
    return result;
}
```

Output:
```rust
Begin A
Begin B
Complete B
Complete A
```