struct FragmentInput {
	@builtin(position) pos: vec4<f32>,
	@location(0) uv: vec2<f32>,
};

struct UniformInput {
	aspect: f32,
	time: f32,
	cursor: vec2<f32>
}

@group(0)
@binding(0)
var<uniform> ubo: UniformInput;

@fragment
fn main(fin: FragmentInput) -> @location(0) vec4<f32> {
  let uv = (fin.uv * 2.0 - 1.0) * vec2<f32>(ubo.aspect, 1.0);
  
  var dir = vec2<f32>(0.0);
  if ubo.cursor.x >= 0.0 {
    dir = -(ubo.cursor * 2.0 - 1.0);
  }
  
  let rot_z = mat3x3<f32>(vec3<f32>(
    cos(dir.x), 0.0, -sin(dir.x)), vec3<f32>(
    0.0, 1.0, 0.0), vec3<f32>(
    sin(dir.x), 0.0, cos(dir.x)));
  let rot_x = mat3x3<f32>(vec3<f32>(
    1.0, 0.0, 0.0), vec3<f32>(
    0.0, cos(dir.y), -sin(dir.y)), vec3<f32>(
    0.0, sin(dir.y), cos(dir.y)));

  let camera = vec3<f32>(0.0, 0.0, 0.0);
  let look_at = vec3<f32>(uv, -1.0);
  let ray_dir = rot_z * rot_x * (look_at - camera);
  
  let d = (20.0 - camera.y) / ray_dir.y;
  let p = camera + ray_dir * d;
  
  if d <= 0.0 {
    return vec4<f32>(0.0, sin(p.xz) * 0.5 + 0.5, 1.0);
  } else {
    return vec4<f32>(0.0, 0.0, 0.0, 1.0);
  }
}