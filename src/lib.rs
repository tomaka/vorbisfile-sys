#![allow(non_snake_case)]

extern crate libc;
extern crate "ogg-sys" as ogg;
extern crate "vorbis-sys" as vorbis_sys;

#[repr(C)]
pub struct ov_callbacks {
    pub read_func: extern fn(*mut libc::c_void, libc::size_t, libc::size_t, *mut libc::c_void)
        -> libc::size_t,
    pub seek_func: extern fn(*mut libc::c_void, ogg::ogg_int64_t, libc::c_int) -> libc::c_int,
    pub close_func: extern fn(*mut libc::c_void) -> libc::c_int,
    pub tell_func: extern fn(*mut libc::c_void) -> libc::c_long,
}

// TODO: add static callbacks

pub const NOTOPEN: libc::c_int = 0;
pub const PARTOPEN: libc::c_int = 1;
pub const OPENED: libc::c_int = 2;
pub const STREAMSET: libc::c_int = 3;
pub const INITSET: libc::c_int = 4;

#[repr(C)]
pub struct OggVorbis_File {
    pub datasource: *mut libc::c_void,
    pub seekable: libc::c_int,
    pub offset: ogg::ogg_int64_t,
    pub end: ogg::ogg_int64_t,
    pub oy: ogg::ogg_sync_state,

    pub links: libc::c_int,
    pub offsets: *mut ogg::ogg_int64_t,
    pub dataoffsets: *mut ogg::ogg_int64_t,
    pub serialnos: *mut libc::c_long,
    pub pcmlengths: *mut ogg::ogg_int64_t,
    pub vi: *mut vorbis_sys::vorbis_info,
    pub vc: *mut vorbis_sys::vorbis_comment,

    pub pcm_offset: ogg::ogg_int64_t,
    pub ready_state: libc::c_int,
    pub current_serialno: libc::c_long,
    pub current_link: libc::c_int,

    pub bittrack: libc::c_double,
    pub samptrack: libc::c_double,

    pub os: ogg::ogg_stream_state,
    pub vd: vorbis_sys::vorbis_dsp_state,
    pub vb: vorbis_sys::vorbis_block,

    pub callbacks: ov_callbacks,
}

extern {
    pub fn ov_clear(vf: *mut OggVorbis_File) -> libc::c_int;
    pub fn ov_fopen(path: *const libc::c_char, vf: *mut OggVorbis_File) -> libc::c_int;
    pub fn ov_open(f: *mut libc::FILE, vf: *mut OggVorbis_File, initial: *const libc::c_char,
        ibytes: libc::c_long) -> libc::c_int;
    pub fn ov_open_callbacks(datasource: *mut libc::c_void, vf: *mut OggVorbis_File,
        initial: *const libc::c_char, ibytes: libc::c_long, callbacks: ov_callbacks)
        -> libc::c_int;

    pub fn ov_test(f: *mut libc::FILE, vf: *mut OggVorbis_File, initial: *const libc::c_char,
        ibytes: libc::c_long) -> libc::c_int;
    pub fn ov_test_callbacks(datasource: *mut libc::c_void, vf: *mut OggVorbis_File,
        initial: *const libc::c_char, ibytes: libc::c_long, callbacks: ov_callbacks)
        -> libc::c_int;
    pub fn ov_test_open(vf: *mut OggVorbis_File) -> libc::c_int;

    pub fn ov_bitrate(vf: *mut OggVorbis_File, i: libc::c_int) -> libc::c_long;
    pub fn ov_bitrate_instant(vf: *mut OggVorbis_File) -> libc::c_long;
    pub fn ov_streams(vf: *mut OggVorbis_File) -> libc::c_long;
    pub fn ov_seekable(vf: *mut OggVorbis_File) -> libc::c_long;
    pub fn ov_serialnumber(vf: *mut OggVorbis_File, i: libc::c_int) -> libc::c_long;

    pub fn ov_raw_total(vf: *mut OggVorbis_File, i: libc::c_int) -> ogg::ogg_int64_t;
    pub fn ov_pcm_total(vf: *mut OggVorbis_File, i: libc::c_int) -> ogg::ogg_int64_t;
    pub fn ov_time_total(vf: *mut OggVorbis_File, i: libc::c_int) -> libc::c_double;

    pub fn ov_raw_seek(vf: *mut OggVorbis_File, pos: ogg::ogg_int64_t) -> libc::c_int;
    pub fn ov_pcm_seek(vf: *mut OggVorbis_File, pos: ogg::ogg_int64_t) -> libc::c_int;
    pub fn ov_pcm_seek_page(vf: *mut OggVorbis_File, pos: ogg::ogg_int64_t) -> libc::c_int;
    pub fn ov_time_seek(vf: *mut OggVorbis_File, pos: libc::c_double) -> libc::c_int;
    pub fn ov_time_seek_page(vf: *mut OggVorbis_File, pos: libc::c_double) -> libc::c_int;

    pub fn ov_raw_seek_lap(vf: *mut OggVorbis_File, pos: ogg::ogg_int64_t) -> libc::c_int;
    pub fn ov_pcm_seek_lap(vf: *mut OggVorbis_File, pos: ogg::ogg_int64_t) -> libc::c_int;
    pub fn ov_pcm_seek_page_lap(vf: *mut OggVorbis_File, pos: ogg::ogg_int64_t) -> libc::c_int;
    pub fn ov_time_seek_lap(vf: *mut OggVorbis_File, pos: libc::c_double) -> libc::c_int;
    pub fn ov_time_seek_page_lap(vf: *mut OggVorbis_File, pos: libc::c_double) -> libc::c_int;

    pub fn ov_raw_tell(vf: *mut OggVorbis_File) -> ogg::ogg_int64_t;
    pub fn ov_pcm_tell(vf: *mut OggVorbis_File) -> ogg::ogg_int64_t;
    pub fn ov_time_tell(vf: *mut OggVorbis_File) -> libc::c_double;

    pub fn ov_info(vf: *mut OggVorbis_File, link: libc::c_int) -> *mut vorbis_sys::vorbis_info;
    pub fn ov_comment(vf: *mut OggVorbis_File, link: libc::c_int) -> *mut vorbis_sys::vorbis_comment;

    pub fn ov_read_float(vf: *mut OggVorbis_File, pcm_channels: *mut *mut *mut libc::c_float,
        samples: libc::c_int, bitstream: *mut libc::c_int) -> libc::c_long;
    pub fn ov_read_filter(vf: *mut OggVorbis_File, buffer: *mut libc::c_char, length: libc::c_int,
        bigendianp: libc::c_int, word: libc::c_int, sgned: libc::c_int,
        bitstream: *mut libc::c_int,
        filter: extern fn(*mut *mut libc::c_float, libc::c_long, libc::c_long, *mut libc::c_void),
        filter_param: *mut libc::c_void) -> libc::c_long;
    pub fn ov_read(vf: *mut OggVorbis_File, buffer: *mut libc::c_char, length: libc::c_int,
        bigendianp: libc::c_int, word: libc::c_int, sgned: libc::c_int,
        bitstream: *mut libc::c_int) -> libc::c_long;
    pub fn ov_crosslap(vf1: *mut OggVorbis_File, vf2: *mut OggVorbis_File) -> libc::c_int;

    pub fn ov_halfrate(vf: *mut OggVorbis_File, flag: libc::c_int) -> libc::c_int;
    pub fn ov_halfrate_p(vf: *mut OggVorbis_File) -> libc::c_int;
}
