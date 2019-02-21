use std::sync::Arc;
use std::error::Error;

pub type PipelineFunction = Fn(PipelineContext, Arc<Pipeline>) -> Result<PipelineContext, Box<Error>>;

pub struct PipelineContext {

}

pub trait Pipeline {
    fn call(&self, context: PipelineContext) -> Result<PipelineContext, Box<Error>>;
}

pub struct PipelineElement {
    func: Arc<PipelineFunction>,
    next: Arc<Pipeline>
}

impl PipelineElement {
    pub fn new(func: Arc<PipelineFunction>, next: Arc<Pipeline>) -> Self {
        return PipelineElement{func, next};
    }
}

impl Pipeline for PipelineElement {
    fn call(&self, context: PipelineContext) -> Result<PipelineContext, Box<Error>> {
        (self.func)(context, self.next.clone())
    }
}

pub struct PipelineTail { }

impl Pipeline for PipelineTail {
    fn call(&self, context: PipelineContext) -> Result<PipelineContext, Box<Error>> {
        return Result::Ok(context)
    }
}
