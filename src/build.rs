extern crate dotenv;
extern crate serde;
extern crate serde_yaml;

extern crate indexmap;
use indexmap::IndexMap;
use indexmap::IndexSet;

use std::io::prelude::*;
use std::fs::File;

// VariantMap is a strong, independent type who don't need no simplification
type VariantMap = IndexMap<String, IndexMap<String, Vec<String>>>;

fn main() {
    match dotenv::dotenv() {
        Err(e) => println!("Error reading .env file: {}", e),
        _ => (),
    }
    let filename = std::env::var("VARIANT_MAP_FILEPATH")
        .expect("VARIANT_MAP_FILEPATH must be set (check `.env`).");
    let mut fh = File::open(filename.as_str())
        .expect(format!("Error while opening `{}`", filename).as_str());
    let mut variant_map_yaml = String::new(); //"{ map:".to_string();
    fh.read_to_string(&mut variant_map_yaml)
        .expect(format!("Error while reading `{}`", filename).as_str());
    let variant_map: VariantMap = serde_yaml::from_str(variant_map_yaml.as_str()).expect("Bad yaml");
    let mut variants = IndexMap::<&str, IndexSet<&str>>::new();
    let mut suits = IndexSet::<&str>::new();
    let mut colors = IndexSet::<&str>::new();
    for (variant, variant_def) in variant_map.iter() {
        variants.insert(variant, IndexSet::new());
        let mut variant_colors = variants.get_mut(variant.as_str()).unwrap(); //not sure why as_str is needed here
        for (suit, suit_colors) in variant_def.iter() {
            if suit != "colors" { suits.insert(suit); }
            for color in suit_colors.iter() {
                colors.insert(color);
                variant_colors.insert(color);
            }
        }
    }
    let mut outfile_text = "pub enum Variant {\n".to_string();
    for (variant, _) in variants.iter() {
        outfile_text.push_str("    ");
        outfile_text.push_str(variant);
        outfile_text.push_str(",\n");
    }
    outfile_text.push_str("}\n\npub enum Suit {\n");
    for suit in suits.iter() {
        outfile_text.push_str("    ");
        outfile_text.push_str(suit);
        outfile_text.push_str(",\n");
    }
    outfile_text.push_str("}\n\npub enum Color {\n");
    for color in colors.iter() {
        outfile_text.push_str("    ");
        outfile_text.push_str(color);
        outfile_text.push_str(",\n");
    }
    outfile_text.push_str(
"}

pub type ColorIndex = u8;
pub type ColorResult = Result<Color, ColorIndex>;

impl Variant {
    fn color(&self, i: ColorIndex) ->  ColorResult {
        match self {
"
    );
    for (variant, variant_colors) in variants.iter() {
        outfile_text.push_str("            Variant::");
        outfile_text.push_str(variant);
        outfile_text.push_str(" => {\n                match i {\n");
        for (i, color) in variant_colors.iter().enumerate() {
            outfile_text.push_str("                    ");
            outfile_text.push_str(i.to_string().as_str());
            outfile_text.push_str(" => ");
            outfile_text.push_str("Ok(Color::");
            outfile_text.push_str(color);
            outfile_text.push_str("),\n");
        }
        outfile_text.push_str(
"                    _ => Err(i),
                }
            },
"
        );
    }
    outfile_text.push_str("        }\n");
    outfile_text.push_str("    }\n");
    outfile_text.push_str("}");
    panic!("{}", outfile_text);
}
