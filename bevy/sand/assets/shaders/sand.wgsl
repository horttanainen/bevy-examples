@group(0) @binding(0)
var texture: texture_storage_2d<rgba8unorm, read_write>;

struct Time {
    seconds_since_startup: f32,
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

const empty = vec4<f32>(0.0, 0.0, 0.0, 0.0);
const sand = vec4<f32>(1.0, 1.0, 0.0, 1.0);

@compute @workgroup_size(8, 8, 1)
fn init(@builtin(global_invocation_id) invocation_id: vec3<u32>, @builtin(num_workgroups) num_workgroups: vec3<u32>) {
    let location = vec2<i32>(invocation_id.xy);

    let isSand = randomFloat(invocation_id.x * invocation_id.y * u32(time.seconds_since_startup * 1000.0)) > 0.99;
    
    var color = empty;
    if (isSand) {
      color = sand;
    } 
    textureStore(texture, location, color);
}

fn sandNextLocation(cur_location: vec2<i32>) -> vec2<i32> {
  return vec2<i32>(cur_location.x, cur_location.y + 1);
}

const epsilon = 0.001;

@compute @workgroup_size(8, 8, 1)
fn update(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    let location = vec2<i32>(invocation_id.xy);
    let cur_color = textureLoad(texture, location);

    var new_location = location;
    if (distance(cur_color, sand) < epsilon) {
      new_location = sandNextLocation(location);
    }

    if (any(location != new_location)) {
      storageBarrier();
      textureStore(texture, location, empty);
      textureStore(texture, new_location, cur_color);
    }
}
