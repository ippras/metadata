use crate::{Metadata, Parameter, Version};
use itertools::{Either, Itertools as _};
use jiff::civil::Date;

pub fn join<'a>(iter: impl Iterator<Item = &'a Metadata> + Clone) -> Metadata {
    let mut meta = Metadata::default();
    meta.authors = authors(iter.clone());
    meta.dates = dates(iter.clone());
    meta.name = name(iter.clone());
    meta.parameters = parameters(iter.clone());
    meta.versions = versions(iter.clone(), true);

    // meta.insert(AUTHORS.to_owned(), authors(frames));
    // meta.insert(DATE.to_owned(), date(frames));
    // meta.insert(DESCRIPTION.to_owned(), description(frames));
    // meta.insert(NAME.to_owned(), name(frames));
    // meta.insert(PARAMETERS.to_owned(), parameters(frames));
    // meta.insert(VERSION.to_owned(), DEFAULT_VERSION.to_owned());
    meta
}

pub fn authors<'a>(iter: impl Iterator<Item = &'a Metadata>) -> Vec<String> {
    iter.flat_map(|meta| &meta.authors)
        .unique()
        .sorted()
        .cloned()
        .collect()
}

pub fn dates<'a>(iter: impl Iterator<Item = &'a Metadata>) -> Vec<Date> {
    iter.flat_map(|meta| &meta.dates)
        .unique()
        .sorted()
        .cloned()
        .collect()
}

// pub fn description(frames: &[HashedMetaDataFrame]) -> String {
//     let descriptions = frames
//         .iter()
//         .flat_map(|frame| frame.meta.get(DESCRIPTION))
//         .map(String::as_str)
//         .collect();
//     longest_common_prefix(descriptions).to_owned()
// }

pub fn name<'a>(iter: impl Iterator<Item = &'a Metadata>) -> String {
    iter.map(|meta| &meta.name).unique().join(" & ")
}

pub fn parameters<'a>(iter: impl Iterator<Item = &'a Metadata>) -> Vec<Parameter> {
    iter.flat_map(|meta| &meta.parameters)
        .unique()
        .sorted()
        .cloned()
        .collect()
}

pub fn versions<'a>(iter: impl Iterator<Item = &'a Metadata>, unique: bool) -> Vec<Version> {
    let iter = iter.flat_map(|meta| &meta.versions);
    let iter = if unique {
        Either::Right(iter.unique())
    } else {
        Either::Left(iter)
    };
    iter.copied().collect()
}

pub fn longest_common_prefix(strings: Vec<&str>) -> &str {
    if strings.is_empty() {
        return "";
    }
    let mut prefix = strings[0];
    for string in strings {
        while !string.starts_with(prefix) {
            if prefix.is_empty() {
                return "";
            }
            prefix = prefix
                .trim_end_matches(|c| c != '\n')
                .trim_end_matches('\n');
        }
    }
    prefix
}
