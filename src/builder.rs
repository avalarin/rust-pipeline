use std::sync::Arc;

use crate::types::{PipelineFunction, Pipeline, PipelineElement, PipelineTail};

pub struct PipelineBuilder {
    elements: Vec<Arc<PipelineFunction>>
}

impl PipelineBuilder {
    pub fn new() -> Self {
        PipelineBuilder{
            elements: vec![]
        }
    }

    pub fn next(&mut self, func: Arc<PipelineFunction>) -> &mut Self {
        self.elements.push(func);
        self
    }

    pub fn build(&self) -> Arc<Pipeline> {
        return self.elements.iter().rev().fold(
            Arc::new(PipelineTail{}),
            |pipeline,func| {
                return Arc::new(PipelineElement::new(func.clone(), pipeline))
            }
        )
    }
}