mod builder;
mod types;

use std::sync::Arc;
use std::error::Error;
use builder::PipelineBuilder;
use types::{Pipeline};

struct Context {
    number: i32
}

fn main() {
    let pipeline = PipelineBuilder::new()
        .next(Arc::new(a))
        .next(Arc::new(b))
        .build();

    let result = pipeline.call(Context{number: 0}).expect("Failed");
    println!("Result: {}", result.number);
}

fn a(context: Context, next: Arc<Pipeline<Context>>) -> Result<Context, Box<Error>> {
    println!("Begin A");
    let result = next.call(Context{number: context.number * 2});
    println!("Complete A");
    return result;
}

fn b(context: Context, next: Arc<Pipeline<Context>>) -> Result<Context, Box<Error>> {
    println!("Begin B");
    let result = next.call(Context{number: context.number + 1});
    println!("Complete B");
    return result;
}
