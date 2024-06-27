use procgen_templater::dictionary::dictionary::build_dictionary_from_folder;
pub mod city;
pub mod grammar;
fn main() {
    let dictionary = build_dictionary_from_folder("./data_files");
    dictionary.inspect();
}
