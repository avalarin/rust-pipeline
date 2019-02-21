use std::sync::Arc;
use std::error::Error;

pub type PipelineFunction<T> = Fn(T, Arc<Pipeline<T>>) -> Result<T, Box<Error>>;

pub trait Pipeline<T> {
    fn call(&self, context: T) -> Result<T, Box<Error>>;
}

pub struct PipelineElement<T> {
    func: Arc<PipelineFunction<T>>,
    next: Arc<Pipeline<T>>
}

impl <T> PipelineElement<T> {
    pub fn new(func: Arc<PipelineFunction<T>>, next: Arc<Pipeline<T>>) -> Self {
        return PipelineElement{func, next};
    }
}

impl <T> Pipeline<T> for PipelineElement<T> {
    fn call(&self, context: T) -> Result<T, Box<Error>> {
        (self.func)(context, self.next.clone())
    }
}

pub struct PipelineTail { }

impl <T> Pipeline<T> for PipelineTail {
    fn call(&self, context: T) -> Result<T, Box<Error>> {
        return Result::Ok(context)
    }
}
