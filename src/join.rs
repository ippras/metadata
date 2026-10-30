use crate::{
    Metadata,
    rule::{
        ALL_DATES, ALL_VERSIONS, Rule, UNIQUE_AUTHORS, UNIQUE_IDENTIFIERS, UNIQUE_NAMES,
        UNIQUE_STEREOSPECIFIC_NUMBERS,
    },
};
use std::borrow::Borrow;

pub fn join<'a>(iter: impl Iterator<Item = &'a Metadata> + Clone) -> Metadata {
    let parameters = iter.flat_map(|metadata| &metadata.0).cloned().collect();
    let mut metadata = Metadata(parameters);
    metadata.filter_and_sort([
        &*UNIQUE_AUTHORS,
        &*ALL_DATES,
        &*UNIQUE_IDENTIFIERS,
        &*UNIQUE_STEREOSPECIFIC_NUMBERS,
        &*UNIQUE_NAMES,
        &*ALL_VERSIONS,
    ]);
    metadata
}

pub fn join_with<'a, T: Borrow<Rule>>(
    iter: impl Iterator<Item = &'a Metadata> + Clone,
    rules: impl AsRef<[T]>,
) -> Metadata {
    let parameters = iter.flat_map(|metadata| &metadata.0).cloned().collect();
    let mut metadata = Metadata(parameters);
    metadata.filter_and_sort(rules);
    metadata
}

// pub fn longest_common_prefix(strings: Vec<&str>) -> &str {
//     if strings.is_empty() {
//         return "";
//     }
//     let mut prefix = strings[0];
//     for string in strings {
//         while !string.starts_with(prefix) {
//             if prefix.is_empty() {
//                 return "";
//             }
//             prefix = prefix
//                 .trim_end_matches(|c| c != '\n')
//                 .trim_end_matches('\n');
//         }
//     }
//     prefix
// }
