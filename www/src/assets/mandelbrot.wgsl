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

fn mandelbrot(p: vec2<f32>) -> f32 {
  var x = 0.0;
  var y = 0.0;
  var iteration = 0;
  let max_iteration = 1000;
  while x*x + y*y <= 2.0*2.0 && iteration < max_iteration {
    let xtemp = x*x - y*y + p.x;
    y = 2.0*x*y + p.y;
    x = xtemp;
    iteration = iteration + 1;
  }
 
  return sin(f32(iteration) * 10.0 / f32(max_iteration));
}

@fragment
fn main(fin: FragmentInput)
  -> @location(0) vec4<f32>
{
  var offset = vec2<f32>(0.6, 0.0);
  if ubo.cursor.x >= 0.0 {
    offset = ubo.cursor * 2.0 - 1.0;
  }

  let uv = (fin.uv * 2.5 - 1.25) * vec2<f32>(ubo.aspect, 1.0) - vec2<f32>(0.6, 0.0) + offset;
  let v = mandelbrot(uv);
  return vec4<f32>(v, v * 0.3, 0.0, 1.0);
}