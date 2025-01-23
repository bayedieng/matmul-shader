@binding(0) @group(0) var<storage, read> array_a: array<array<f32, 3>, 2>;
@binding(1) @group(0) var<storage, read> array_b: array<array<f32, 2>, 3>;
@binding(2) @group(0) var<storage, read_write> array_c: array<array<f32, 2>, 2>;

@compute @workgroup_size(1)
fn main() {
  for (var i = 0; i < 2; i++) {
    for (var j = 0; j < 2; j++) {
        for (var k = 0; k < 3; k++) {
            array_c[i][j] += array_a[i][k] * array_b[k][j];
        }
    }
  }
  }
