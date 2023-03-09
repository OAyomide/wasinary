use std::fs::*;
use std::path::*;
fn main() {
    let npr = wasinary::Transformation {
        width: 10,
        height: 10,
    };
    let mut ps = wasinary::WasinaryImage::new(
        "https://thumbs.dreamstime.com/b/tree-white-background-no6-14621137.jpg",
    );
    let img = ps.download()
    // .sepia()
    // .write_text("Sid!")
        .background_color([67, 12, 245, 255])
    .done();
    // .resize(200, 200)
    // .overlay("https://images.genius.com/c89349ae9941cfe3fc5bc34f9934fa21.1000x1000x1.jpg", 10, 20)
    // .watermark("https://images.assetsdelivery.com/compings_v2/siridhata/siridhata1701/siridhata170100010.jpg")
    // .done();


    let fout = &mut File::create(&Path::new(&format!("{}.png", "dummy"))).unwrap();
    img.write_to(fout, image::ImageOutputFormat::Png).unwrap();
}
