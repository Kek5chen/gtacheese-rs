extern crate embed_resource;

fn main() {
    embed_resource::compile("the-cheese-manifest.rc", embed_resource::NONE);
}