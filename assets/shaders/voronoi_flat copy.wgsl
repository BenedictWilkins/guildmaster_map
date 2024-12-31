// voronoi_flat.wgsl

@vertex
fn vs_main(
    @location(0) position: vec3<f32>
    //@location(1) index: f32
) -> @builtin(position) vec4<f32> {
    // Pass the position and index to the fragment shader
    return vec4<f32>(position, 1.0);
}

@fragment
fn fs_main(
    // @location(1) @flat index: f32
) -> @location(0) vec4<f32> {
    // Use the index to determine the color
    // Example: map index to a color
    //let color = vec3<f32>(
    //    (index * 0.1) % 1.0,
    //    (index * 0.2) % 1.0,
    //    (index * 0.3) % 1.0
    //);
    return vec4<f32>(1.0, 1.0, 1.0, 1.0);
}