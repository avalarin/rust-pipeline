use std::sync::Arc;

use crate::types::{PipelineFunction, Pipeline, PipelineElement, PipelineTail};

pub struct PipelineBuilder<T> {
    elements: Vec<Arc<PipelineFunction<T>>>
}

impl <T> PipelineBuilder<T> where T: 'static {
    pub fn new() -> Self {
        PipelineBuilder{
            elements: vec![]
        }
    }

    pub fn next(&mut self, func: Arc<PipelineFunction<T>>) -> &mut Self {
        self.elements.push(func);
        self
    }

    pub fn build(&self) -> Arc<Pipeline<T>> {
        return self.elements.iter().rev().fold(
            Arc::new(PipelineTail{}),
            |pipeline,func| {
                return Arc::new(PipelineElement::new(func.clone(), pipeline))
            }
        )
    }
}