use bevy::{
    prelude::*,
    render::{*, renderer::*, render_resource::*},
};
use crate::{config::CONFIG, pipeline::GpuComputePipeline, bind_group::GpuComputeBindGroup};

enum HelloState {
    Loading,
    Init,
    Update,
}

pub struct GpuComputeNode {
    state: HelloState,
}

impl Default for GpuComputeNode {
    fn default() -> Self {
        Self {
            state: HelloState::Loading,
        }
    }
}

impl render_graph::Node for GpuComputeNode {
    fn update(&mut self, world: &mut World) {
        let pipeline = world.resource::<GpuComputePipeline>();
        let pipeline_cache = world.resource::<PipelineCache>();

        // if the corresponding pipeline has loaded, transition to the next stage
        match self.state {
            HelloState::Loading => {
                if let CachedPipelineState::Ok(_) =
                    pipeline_cache.get_compute_pipeline_state(pipeline.init_pipeline)
                {
                    self.state = HelloState::Init;
                }
            }
            HelloState::Init => {
                if let CachedPipelineState::Ok(_) =
                    pipeline_cache.get_compute_pipeline_state(pipeline.update_pipeline)
                {
                    self.state = HelloState::Update;
                }
            }
            HelloState::Update => {}
        }
    }

    fn run(
        &self,
        _graph: &mut render_graph::RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), render_graph::NodeRunError> {
        let texture_bind_group = &world.resource::<GpuComputeBindGroup>().0;
        let pipeline_cache = world.resource::<PipelineCache>();
        let pipeline = world.resource::<GpuComputePipeline>();

        let mut pass = render_context
            .command_encoder()
            .begin_compute_pass(&ComputePassDescriptor::default());

        pass.set_bind_group(0, texture_bind_group, &[]);

        // select the pipeline based on the current state
        match self.state {
            HelloState::Loading => {}
            HelloState::Init => {
                let init_pipeline = pipeline_cache
                    .get_compute_pipeline(pipeline.init_pipeline)
                    .unwrap();
                pass.set_pipeline(init_pipeline);
                pass.dispatch_workgroups(CONFIG.table_size.x as u32 / CONFIG.workgroup_size, CONFIG.table_size.y as u32 / CONFIG.workgroup_size, 1);
            }
            HelloState::Update => {
                let update_pipeline = pipeline_cache
                    .get_compute_pipeline(pipeline.update_pipeline)
                    .unwrap();
                pass.set_pipeline(update_pipeline);
                pass.dispatch_workgroups(CONFIG.table_size.x as u32/ CONFIG.workgroup_size, CONFIG.table_size.y as u32 / CONFIG.workgroup_size, 1);
            }
        }

        Ok(())
    }
}

