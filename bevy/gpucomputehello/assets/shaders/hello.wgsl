@group(0) @binding(0)
var texture: texture_storage_2d<rgba8unorm, read_write>;

struct Time {
    time_since_startup: f32,
};
@group(0) @binding(1)
var<uniform> time: Time;

fn hash(value: u32) -> u32 {
    var state = value;
    state = state ^ 2747636419u;
    state = state * 2654435769u;
    state = state ^ state >> 16u;
    state = state * 2654435769u;
    state = state ^ state >> 16u;
    state = state * 2654435769u;
    return state;
}

fn randomFloat(value: u32) -> f32 {
    return f32(hash(value)) / 4294967295.0;
}

@compute @workgroup_size(8, 8, 1)
fn init(@builtin(global_invocation_id) invocation_id: vec3<u32>, @builtin(num_workgroups) num_workgroups: vec3<u32>) {
    let location = vec2<i32>(i32(invocation_id.x), i32(invocation_id.y));

    let color = vec4<f32>(f32(0), f32(255), f32(0), f32(1));

    textureStore(texture, location, color);
}

@compute @workgroup_size(8, 8, 1)
fn update(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    let location = vec2<i32>(i32(invocation_id.x), i32(invocation_id.y));
    let cur_color = textureLoad(texture, location);

    let red = randomFloat(u32(f32(invocation_id.y * invocation_id.x) * time.time_since_startup));
    let green = randomFloat(u32(f32(invocation_id.y * invocation_id.x) * (time.time_since_startup + 1.0)));
    let blue = randomFloat(u32(f32(invocation_id.y * invocation_id.x) * (time.time_since_startup - 1.0)));

    let color = vec4<f32>(f32(red), f32(green), f32(blue), f32(1));

    storageBarrier();

    textureStore(texture, location, color);
}
