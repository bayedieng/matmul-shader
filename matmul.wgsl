@binding(0) @group(0) var<storage, read> array_a: <vec3<f32, 2>>
@binding(1) @group(0) var<storage, read> array_b: <vec2<f32, 3>>
@binding(1) @group(0) var<storage, read_write> array_c: <vec2<f32, 2>>

@compute @workgroup_size(1)
fn main() {
    for (var i: u32 = 0; i < 2u; i++) {
        for(var k: u32; k < 3u; k++) {
            array_c[i][k] += array_a[i][i] * array_b[k][i];
        }
    }
}