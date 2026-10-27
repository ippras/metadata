use crate::{Metadata, rule::UNIQUE_NAMES_AND_UNIQUE_IDENTIFIERS_AND_DATES_AND_VERSIONS};

pub fn join<'a>(iter: impl Iterator<Item = &'a Metadata> + Clone) -> Metadata {
    let parameters = iter.flat_map(|metadata| &metadata.0).cloned().collect();
    let mut metadata = Metadata(parameters);
    metadata.filter_and_sort(&*UNIQUE_NAMES_AND_UNIQUE_IDENTIFIERS_AND_DATES_AND_VERSIONS);
    metadata
}

// pub fn unique_sorted_authors<'a>(iter: impl Iterator<Item = &'a Metadata>) -> Vec<Parameter> {
//     iter.flatten()
//         .filter(|parameter| parameter.name == AUTHOR)
//         .unique()
//         .sorted()
//         .cloned()
//         .collect()
// }

// pub fn unique_sorted_dates<'a>(iter: impl Iterator<Item = &'a Metadata>) -> Vec<Parameter> {
//     iter.flatten()
//         .filter(|parameter| parameter.name == DATE)
//         .unique()
//         .sorted()
//         .cloned()
//         .collect()
// }

// // pub fn description(frames: &[HashedMetaDataFrame]) -> String {
// //     let descriptions = frames
// //         .iter()
// //         .flat_map(|frame| frame.meta.get(DESCRIPTION))
// //         .map(String::as_str)
// //         .collect();
// //     longest_common_prefix(descriptions).to_owned()
// // }

// pub fn name<'a>(mut iter: impl Iterator<Item = &'a Metadata>, first_plus: bool) -> String {
//     if first_plus {
//         let name = iter.next().map(|meta| &*meta.name).unwrap_or_default();
//         let count = iter.count();
//         format!("{name} +{count}",)
//     } else {
//         iter.map(|meta| &meta.name).unique().join(" & ")
//     }
// }

// pub fn parameters<'a>(iter: impl Iterator<Item = &'a Metadata>) -> Vec<Parameter> {
//     iter.flat_map(|meta| &meta.parameters)
//         .unique()
//         .sorted()
//         .cloned()
//         .collect()
// }

// pub fn versions<'a>(iter: impl Iterator<Item = &'a Metadata>, unique: bool) -> Vec<Version> {
//     let iter = iter.flat_map(|meta| &meta.versions);
//     let iter = if unique {
//         Either::Right(iter.unique())
//     } else {
//         Either::Left(iter)
//     };
//     iter.copied().collect()
// }

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
