### WASINARY

The aim of this library is to enable you to do image manipulation on images in your app across platforms. To be able to work both in the browser and server, it's compiled to WebAssembly.

The API/methods in this library are chainable and are mirrored after the cloudinary sdk.

This library is useful for indie hackers and people who just need quick image manipulation without the limitations of the cloudinary (basic) plans.

### How to use

Right now, the library is hugely WIP. I am using it to learn image manipulation and basic computer graphics and at the same time another way to explore using WebAssembly. For now, its not compiling to Wasm, its a rust library _and_ binary right now. The long aim is to make it both a rust library, binary and Wasm module.

The API looks like:

```rust
    let mut photo = wasinary::WasinaryImage::new(
        "https://c.files.bbci.co.uk/12A9B/production/_111434467_gettyimages-1143489763.jpg",
    );
    let out = photo
    .download()
    .sepia()
    .resize(200, 200)
    .write_text("Sid!")
    .done();
```

The following methods are supported:

- sepia
- monochrome
- blur
- resize
- crop
- rotate
- overlay
- watermark
- write_text

You can chain these methods. Docs on the parameters each of the methods take is coming soon but for now, if you are adventorous, pls simply check the source to see what each needs. You can also just check the types.

### Contributing

All contributions welcome. I'd appreciate everything from code reviews to pull request submissions.
