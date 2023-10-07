use bevy::{
    prelude::*,
    render::{*, renderer::*, render_resource::*},
};
use crate::{config::CONFIG, pipeline::SandPipeline, bind_group::SandBindGroup};

enum SandState {
    Loading,
    Init,
    Update,
}

pub struct SandNode {
    state: SandState,
}

impl Default for SandNode {
    fn default() -> Self {
        Self {
            state: SandState::Loading,
        }
    }
}

impl render_graph::Node for SandNode {
    fn update(&mut self, world: &mut World) {
        let pipeline = world.resource::<SandPipeline>();
        let pipeline_cache = world.resource::<PipelineCache>();

        // if the corresponding pipeline has loaded, transition to the next stage
        match self.state {
            SandState::Loading => {
                if let CachedPipelineState::Ok(_) =
                    pipeline_cache.get_compute_pipeline_state(pipeline.init_pipeline)
                {
                    self.state = SandState::Init;
                }
            }
            SandState::Init => {
                if let CachedPipelineState::Ok(_) =
                    pipeline_cache.get_compute_pipeline_state(pipeline.update_pipeline)
                {
                    self.state = SandState::Update;
                }
            }
            SandState::Update => {}
        }
    }

    fn run(
        &self,
        _graph: &mut render_graph::RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), render_graph::NodeRunError> {
        let texture_bind_group = &world.resource::<SandBindGroup>().0;
        let pipeline_cache = world.resource::<PipelineCache>();
        let pipeline = world.resource::<SandPipeline>();

        let mut pass = render_context
            .command_encoder()
            .begin_compute_pass(&ComputePassDescriptor::default());

        pass.set_bind_group(0, texture_bind_group, &[]);

        // select the pipeline based on the current state
        match self.state {
            SandState::Loading => {}
            SandState::Init => {
                let init_pipeline = pipeline_cache
                    .get_compute_pipeline(pipeline.init_pipeline)
                    .unwrap();
                pass.set_pipeline(init_pipeline);
                pass.dispatch_workgroups(CONFIG.size.0 / CONFIG.workgroup_size, CONFIG.size.1 / CONFIG.workgroup_size, 1);
            }
            SandState::Update => {
                let update_pipeline = pipeline_cache
                    .get_compute_pipeline(pipeline.update_pipeline)
                    .unwrap();
                pass.set_pipeline(update_pipeline);
                pass.dispatch_workgroups(CONFIG.size.0 / CONFIG.workgroup_size, CONFIG.size.1 / CONFIG.workgroup_size, 1);
            }
        }

        Ok(())
    }
}

