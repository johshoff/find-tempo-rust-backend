#[macro_use] extern crate bart_derive;
extern crate rustc_serialize;
use rustc_serialize::json;
use std::io::prelude::*;
use std::fs::File;

#[derive(RustcDecodable)]
struct Song {
    uri: String,
    artist: String,
    title: String,
    notes: Option<String>,
    bpm: f32,
}

#[derive(RustcDecodable)]
#[derive(BartDisplay)]
#[template = "songs.html"]
struct SongList {
    songs: Vec<Song>
}

fn main() {
    let mut f = File::open("example-bpms.json").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    let songs: Vec<Song> = json::decode(&s).unwrap();
    print!("{}", &SongList { songs: songs });
}

