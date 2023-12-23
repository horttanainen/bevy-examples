@group(0) @binding(0)
var texture: texture_storage_2d<rgba8unorm, read_write>;

struct Time {
    time_since_startup: f32,
};
@group(0) @binding(1)
var<uniform> time: Time;

@group(0) @binding(2)
var<uniform> cue_ball_pos: vec2<f32>;

struct BallStatus {
   position: vec3<f32>,
   selected: i32
};

const number_of_balls = #NUMBER_OF_BALLS;
@group(0) @binding(3)
var<uniform> balls: array<BallStatus, number_of_balls>;

struct PocketStatus {
   position: vec3<f32>,
   selected: i32
};

@group(0) @binding(4)
var<uniform> pockets: array<PocketStatus, 6>;

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

fn pocketIsSelected() -> bool {
  for (var i: i32 = 0; i < 6; i++) {
    if (bool(pockets[i].selected)) {
      return true;
    }
  }
  return false;
}

fn findSelectedPocketPosition() -> vec2<f32> {
  for (var i: i32 = 0; i < 6; i++) {
    if (bool(pockets[i].selected)) {
      return pockets[i].position.xy;
    }
  }
  return vec2<f32>(0.0,0.0);
}

fn ballIsSelected() -> bool {
  for (var i: i32 = 0; i < number_of_balls; i++) {
    if (bool(balls[i].selected)) {
      return true;
    }
  }
  return false;
}

fn findSelectedBallPosition() -> vec2<f32> {
  for (var i: i32 = 0; i < number_of_balls; i++) {
    if (bool(balls[i].selected)) {
      return balls[i].position.xy;
    }
  }
  return vec2<f32>(0.0,0.0);
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

    storageBarrier();

    var color = baize;
    if (!ballIsSelected()) {
      textureStore(texture, coordinate, color);
      return;
    }

    if (!pocketIsSelected()) {
      textureStore(texture, coordinate, color);
      return;
    }

    let selected_ball_position = findSelectedBallPosition();
    let distance_from_selected_ball = distance(vec2<f32>(coordinate), selected_ball_position);
    let selected_pocket_position = findSelectedPocketPosition();


    let distance_from_cue_ball_to_selected_ball = distance(cue_ball_pos, selected_ball_position);
    let distance_from_selected_ball_to_pocket = distance(selected_ball_position, selected_pocket_position);
    let distance_from_selected_pocket = distance(vec2<f32>(coordinate), selected_pocket_position);

    let distance_from_cue_ball_to_pixel_via_selected_ball_and_pocket = distance_from_cue_ball_to_selected_ball + distance_from_selected_ball_to_pocket + distance_from_selected_pocket;

    color = vec4<f32>(distance_from_cue_ball_to_pixel_via_selected_ball_and_pocket / (1280.0 * 2.0), 0.0, 0.0, 1.0);

    textureStore(texture, coordinate, color);
}
