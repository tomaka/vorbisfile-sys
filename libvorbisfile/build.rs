extern crate gcc;

fn main() {
    let config = gcc::Config {
        include_directories: vec![
            Path::new(std::os::getenv("DEP_VORBIS_INCLUDE").unwrap()),
            Path::new(std::os::getenv("DEP_VORBIS_SRC").unwrap()),
            Path::new(std::os::getenv("DEP_OGG_INCLUDE").unwrap()),
        ],
        definitions: vec![
            ("_USRDLL".to_string(), None),
            ("LIBVORBIS_EXPORTS".to_string(), None)
        ],
        .. std::default::Default::default()
    };

    gcc::compile_library("libvorbisfile.a", &config, &[
        "libvorbisfile/vorbisfile.c"
    ]);
}
