// nxm * mxp = n*p
// 2x3* 3x2 = 2x2
#[tokio::main]
async fn main() {
    let a = [[3, 7, 9], [8, 4, 1]];
    let b = [[46, 24], [10, 7], [9, 8]];
    let mut c = [[0; 2]; 2];
    let mut c_gpu = c.clone();

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..3 {
                c[i][j] += a[i][k] * b[k][j]
            }
        }
    }

    let instance = wgpu::Instance::default();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await
        .unwrap();

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::downlevel_defaults(),
                memory_hints: wgpu::MemoryHints::MemoryUsage,
            },
            None,
        )
        .await
        .unwrap();
}
