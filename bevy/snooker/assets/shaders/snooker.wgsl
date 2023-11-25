@group(0) @binding(0)
var texture: texture_storage_2d<rgba8unorm, read_write>;

struct Time {
    time_since_startup: f32,
};
@group(0) @binding(1)
var<uniform> time: Time;

@group(0) @binding(2)
var<uniform> cue_ball_pos: vec2<f32>;

const number_of_balls = #NUMBER_OF_BALLS;
@group(0) @binding(3)
var<uniform> balls: array<vec4<f32>, number_of_balls>;

const baize = vec4<f32>(0.0, 1.0, 0.0, 1.0);

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

    textureStore(texture, location, baize);
}

fn is_visible(cue_ball: vec2<f32>, blocker: vec2<f32>, tile: vec2<f32>) -> bool {
  let blocker_to_cue_ball = cue_ball - blocker;
  let tile_to_blocker = blocker - tile;
  let tile_to_cue_ball = cue_ball - tile;
  
  if (length(tile_to_cue_ball) < length(tile_to_blocker) || length(tile_to_cue_ball) < length(blocker_to_cue_ball)) {
    return true;
  }
  let towards_cue_ball_until_blocker = dot(tile_to_blocker, tile_to_cue_ball) 
    / pow(length(tile_to_cue_ball), 2.0) 
    * tile_to_cue_ball;
  let distance = length(tile_to_blocker - towards_cue_ball_until_blocker);
  if (distance <= 5.0) {
    return false;
  }
  return true;
}

@compute @workgroup_size(8, 8, 1)
fn update(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    let coordinate = vec2<i32>(invocation_id.xy);
    let cur_color = textureLoad(texture, coordinate);

    let distance_from_cue_ball = distance(vec2<f32>(coordinate), cue_ball_pos) / 1280.0;

    var visible = true;
    for (var i: i32 = 0; i < number_of_balls; i++) {
      if (!is_visible(cue_ball_pos, balls[i].xy, vec2<f32>(coordinate))) {
        visible = false;
        break;
      }
    }

    var color = baize;
    if (visible) {
      color = vec4<f32>(1.0 - distance_from_cue_ball, 1.0 - (distance_from_cue_ball / 3.0), 0.0, 1.0);
    }

    storageBarrier();

    textureStore(texture, coordinate, color);
}
