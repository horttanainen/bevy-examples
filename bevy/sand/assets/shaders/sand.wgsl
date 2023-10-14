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

fn randomFloat(value: i32) -> f32 {
    return f32(hash(u32(value))) / 4294967295.0;
}

const empty = vec4<f32>(0.0, 0.0, 0.0, 0.0);
const sand = vec4<f32>(1.0, 1.0, 0.0, 1.0);
const brick = vec4<f32>(1.0, 0.0, 0.2, 1.0);
const water = vec4<f32>(0.0, 0.0, 1.0, 1.0);

fn drawBricks() {
  for (var y = 0; y < 1280; y++) {
    for (var x = 0; x < 1280; x++) {
      let roughX = x / 200;
      let roughY = y / 100;
      let m = roughY % 2;
      let k = roughY % 3;
      if (k == 0 && randomFloat(roughX * roughY) > 0.4 && roughX % 2 == m && roughY % 2 == m) {
        var xCor = x + i32((0.5 - randomFloat(roughX * roughY * i32(time.seconds_since_startup * 1000.0))) * 100.0);
        textureStore(texture, vec2<i32>(xCor, y), brick);
      }
    }
  }
}

fn clear() {
  for (var y = 0; y < 1280; y++) {
    for (var x = 0; x < 1280; x++) {
      let location = vec2<i32>(x, y);
      textureStore(texture, location, empty);
    }
  }
}

fn drawSand() {
  for (var y = 0; y < 1280; y++) {
    for (var x = 0; x < 1280; x++) {
      let location = vec2<i32>(x, y);
      let cur_color = textureLoad(texture, location);
      if(isEmpty(cur_color) && randomFloat(location.x * location.y * i32(time.seconds_since_startup * 1000.0)) > 0.9) {
        textureStore(texture, location, sand);
      }
    }
  }
}

fn drawWater() {
  for (var y = 0; y < 1280; y++) {
    for (var x = 0; x < 1280; x++) {
      let location = vec2<i32>(x, y);
      let cur_color = textureLoad(texture, location);
      if(isEmpty(cur_color) && randomFloat(location.x / location.y * i32(time.seconds_since_startup * 1000.0)) > 0.9) {
        textureStore(texture, location, water);
      }
    }
  }
}

fn restart(location: vec2<i32>) {
  drawWater();
  drawSand();
  drawBricks();
}

@compute @workgroup_size(8, 8, 1)
fn init(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
  let location = vec2<i32>(invocation_id.xy);
  if(location.x == 0 && location.y == 0) {
    restart(location);
  }
}

fn inBounds(location: vec2<i32>) -> bool {
  return location.x < 1280 && location.x >= 0 && location.y < 1280 && location.y >= 0;
}

fn sandNextLocation(cur_location: vec2<i32>) -> vec2<i32> {
  let below = vec2<i32>(cur_location.x, cur_location.y + 1);
  let below_right = vec2<i32>(cur_location.x + 1, cur_location.y + 1);
  let below_left = vec2<i32>(cur_location.x - 1, cur_location.y + 1);
  let color_below = textureLoad(texture, below);
  let color_below_right = textureLoad(texture, below_right);
  let color_below_left = textureLoad(texture, below_left);
  if (isEmptyOrWater(color_below) && inBounds(below)) {
    return below;
  } else if (isEmptyOrWater(color_below_right) && inBounds(below_right)) {
    return below_right;
  } else if (isEmptyOrWater(color_below_left) && inBounds(below_left)) {
    return below_left;
  }
  return cur_location;
}

fn waterNextLocation(cur_location: vec2<i32>) -> vec2<i32> {
  let below = vec2<i32>(cur_location.x, cur_location.y + 1);
  let below_right = vec2<i32>(cur_location.x + 1, cur_location.y + 1);
  let below_left = vec2<i32>(cur_location.x - 1, cur_location.y + 1);
  let right = vec2<i32>(cur_location.x + 1, cur_location.y);
  let left = vec2<i32>(cur_location.x - 1, cur_location.y);
  let color_below = textureLoad(texture, below);
  let color_below_right = textureLoad(texture, below_right);
  let color_below_left = textureLoad(texture, below_left);
  let color_right = textureLoad(texture, right);
  let color_left = textureLoad(texture, left);
  if (isEmpty(color_below) && inBounds(below)) {
    return below;
  } else if (isEmpty(color_below_right) && inBounds(below_right)) {
    return below_right;
  } else if (isEmpty(color_below_left) && inBounds(below_left)) {
    return below_left;
  } else if (isEmpty(color_right) && inBounds(right)) {
    return right;
  } else if (isEmpty(color_left) && inBounds(left)) {
    return left;
  }
  return cur_location;
}

const epsilon = 0.001;
fn isWater(color: vec4<f32>) -> bool {
  return distance(color, water) < epsilon;
}
fn isSand(color: vec4<f32>) -> bool {
  return distance(color, sand) < epsilon;
}
fn isBrick(color: vec4<f32>) -> bool {
  return distance(color, brick) < epsilon;
}
fn isEmpty(color: vec4<f32>) -> bool {
  return distance(color, empty) < epsilon;
}
fn isEmptyOrWater(color: vec4<f32>) -> bool {
  return isEmpty(color) || isWater(color);
}

fn locationOfNextEmptyUpwards(location: vec2<i32>) -> vec2<i32> {
  for (var y = location.y; y >= 0; y--) {
    let loc = vec2<i32>(location.x, y);
    let color = textureLoad(texture, loc);
    if(isEmpty(color)) {
      return loc;
    }
  }
  return vec2<i32>(location.x, 0);
}

@compute @workgroup_size(8, 8, 1)
fn update(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    let location = vec2<i32>(invocation_id.xy);

    let cur_color = textureLoad(texture, location);

    var new_location = location;
    if (isSand(cur_color)) {
      new_location = sandNextLocation(location);
    } else if (isWater(cur_color)) {
      new_location = waterNextLocation(location);
    }

    if (any(location != new_location)) {
      workgroupBarrier();
      let color_to_be_replaced = textureLoad(texture, new_location);
      textureStore(texture, location, color_to_be_replaced);
      textureStore(texture, new_location, cur_color);
    } 
}
