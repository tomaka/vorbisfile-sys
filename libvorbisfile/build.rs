extern crate "pkg-config" as pkg_config;
extern crate gcc;

fn main() {
    match pkg_config::find_library("vorbisfile") {
        Ok(()) => return,
        Err(..) => {}
    };

    gcc::Config::new()
                .file("libvorbisfile/vorbisfile.c")
                .define("_USRDLL", None)
                .define("LIBVORBIS_EXPORTS", None)
                .include(Path::new(std::env::var_string("DEP_VORBIS_INCLUDE").unwrap()))
                .include(Path::new(std::env::var_string("DEP_VORBIS_SRC").unwrap()))
                .include(Path::new(std::env::var_string("DEP_OGG_INCLUDE").unwrap()))
                .compile("libvorbisfile.a");
}
