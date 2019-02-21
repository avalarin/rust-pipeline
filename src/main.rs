mod builder;
mod types;

use std::sync::Arc;
use std::error::Error;
use builder::PipelineBuilder;
use types::{PipelineContext, Pipeline};

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
