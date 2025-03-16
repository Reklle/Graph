struct VertexInput {
    @location(0) t: f32,
};

struct ControlPoints {
    points: array<vec3<f32>, 4>,
};

struct Params {
    width: f32,
    height: f32,
};

@group(0) @binding(0)
var<uniform> control: ControlPoints;


struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) t: f32,
};

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    let p0 = control.points[0];
    let p1 = control.points[1];
    let p2 = control.points[2];
    let p3 = control.points[3];
    
    let u = 1.0 - input.t;
    let tt = input.t * input.t;
    let uu = u * u;
    let uuu = uu * u;
    let ttt = tt * input.t;

    var pos = uuu * p0 + 3.0 * uu * input.t * p1 + 3.0 * u * tt * p2 + ttt * p3;

    let x_ndc = pos.x / 600 * 2.0 - 1.0;
    let y_ndc = -(pos.y / 400 * 2.0 - 1.0);
    
    var output: VertexOutput;
    output.position = vec4<f32>(x_ndc, y_ndc, 0.0, 1.0);
    output.t = input.t;
    return output;
}

struct FragmentInput {
    @builtin(position) frag_pos: vec4<f32>,
    @location(0) t: f32,
};

@fragment
fn fs_main(input: FragmentInput) -> @location(0) vec4<f32> {
    let color = mix(vec3(1.0, 0.0, 0.0), vec3(0.0, 0.0, 1.0), input.t);

    // Базовая толщина линии
    let thickness = 0.02;
    let dynamic_thickness = thickness * (1.0 + 0.5 * sin(input.t * 10.0));

    // Расчет расстояния до линии
    let distance_to_line = abs(input.frag_pos.x - input.frag_pos.y); // Пример расчета
    let alpha = 1.0 - smoothstep(dynamic_thickness, dynamic_thickness + 0.01, distance_to_line);

    return vec4<f32>(color, alpha);
}