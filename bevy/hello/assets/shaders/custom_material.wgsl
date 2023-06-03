#import bevy_sprite::mesh2d_view_bindings
#import bevy_pbr::utils

@fragment
fn fragment(
    @builtin(position) position: vec4<f32>,
    #import bevy_sprite::mesh2d_vertex_output
) -> @location(0) vec4<f32> {
    let uv = coords_to_viewport_uv(position.xy, view.viewport);
    return vec4(uv, .0, 1.0);
}
