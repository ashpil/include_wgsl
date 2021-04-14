# include_wgsl

A tiny proc macro to include a WGSL file in your binary, and verify that it is valid at compile time.

### Example

This is how you might create a [`wgpu`](https://github.com/gfx-rs/wgpu) shader module:

```rust
let shader_str = include_wgsl!("shader.wgsl");
device.create_shader_module(&ShaderModuleDescriptor {
    source: ShaderSource::Wgsl(Cow::Borrowed(&shader_str)),
    flags: ShaderFlags::default(),
    label: None,
})
```

This functions exactly as it would if you had used `include_str!("shader.wgsl")`, but it also makes sure at compile time that your WGSL is valid using [`naga`](https://github.com/gfx-rs/naga.git).

If your WGSL code is valid, compliation continues on, but if your WGSL is invalid, you will get a friendly naga error, and compliation will halt:

```rust
error: Unable to parse shader.wgsl

error: invalid field accessor `world_positoon`
   ┌─ wgsl:33:9
   │
33 │     out.world_positoon = world_position.xyz;
   │         ^^^^^^^^^^^^^^ invalid accessor
```

The syntax errors are typically pretty understandable, but at the moment naga validation errors are a bit dense. I do not consider improving them part of the scope of this project, but will happily update this crate once naga has made them more approachable.

### Nightly

This crate is currently nightly-only. If I have to choose between nightly or fewer dependencies, I choose nightly, and I have adhered to that philosophy in this crate. 

