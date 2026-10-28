use crate::{
    Metadata,
    r#const::{AUTHOR, DATE, IDENTIFIER, KIND, NAME, VERSION},
    rule::{
        Name, Rule,
        UNIQUE_AUTHORS_AND_UNIQUE_NAMES_AND_UNIQUE_IDENTIFIERS_AND_UNIQUE_KINDS_AND_DATES_AND_VERSIONS,
        Value,
    },
};
use typed_builder::TypedBuilder;

/// Join options
#[derive(Clone, Copy, Debug, Default, TypedBuilder)]
pub struct JoinOptions {
    pub unique_names: bool,
    pub unique_kinds: bool,
}

pub fn join<'a>(iter: impl Iterator<Item = &'a Metadata> + Clone) -> Metadata {
    let parameters = iter.flat_map(|metadata| &metadata.0).cloned().collect();
    let mut metadata = Metadata(parameters);
    metadata.filter_and_sort(&*UNIQUE_AUTHORS_AND_UNIQUE_NAMES_AND_UNIQUE_IDENTIFIERS_AND_UNIQUE_KINDS_AND_DATES_AND_VERSIONS);
    metadata
}

pub fn join_with_options<'a>(
    iter: impl Iterator<Item = &'a Metadata> + Clone,
    options: JoinOptions,
) -> Metadata {
    let parameters = iter.flat_map(|metadata| &metadata.0).cloned().collect();
    let mut metadata = Metadata(parameters);
    let rules = [
        Rule::builder()
            .name(Name::exact(AUTHOR))
            .value(Value::Unique)
            .build(),
        Rule::builder()
            .name(Name::exact(DATE))
            .value(Value::All)
            .build(),
        Rule::builder()
            .name(Name::exact(IDENTIFIER))
            .value(Value::Unique)
            .build(),
        Rule::builder()
            .name(Name::exact(KIND))
            .value(if options.unique_kinds {
                Value::Unique
            } else {
                Value::All
            })
            .build(),
        Rule::builder()
            .name(Name::exact(NAME))
            .value(if options.unique_names {
                Value::Unique
            } else {
                Value::All
            })
            .build(),
        Rule::builder()
            .name(Name::exact(VERSION))
            .value(Value::All)
            .build(),
    ];
    metadata.filter_and_sort(&rules);
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
