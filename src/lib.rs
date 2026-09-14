pub mod l10n {
    use egui_l10n::ftl;

    pub const EN: &[&str] = &[ftl!("en/main.ftl")];

    pub const RU: &[&str] = &[ftl!("ru/main.ftl")];
}

pub mod r#const;
pub mod format;
pub mod rule;
// pub mod join;

#[cfg(feature = "egui")]
// pub mod egui;
#[cfg(feature = "polars")]
pub mod polars;

use crate::{
    r#const::{AUTHORS, DATES, DESCRIPTION, NAME, PARAMETERS, VERSIONS},
    rule::{Name, Rule, Value},
};
use jiff::civil::Date;
use ron::{extensions::Extensions, ser::PrettyConfig};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, HashSet},
    fmt::{Debug, Display, Formatter},
    ops::{Deref, DerefMut},
    slice::{Iter, IterMut},
    sync::LazyLock,
};

pub const ID_SALT: &str = "Metadata";

pub static PRETTY_CONFIG: LazyLock<PrettyConfig> = LazyLock::new(|| {
    PrettyConfig::new()
        .depth_limit(2)
        .extensions(Extensions::UNWRAP_NEWTYPES | Extensions::IMPLICIT_SOME)
        .new_line("\n")
});

// /// Metadata
// #[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
// pub struct Metadata {
//     #[serde(default, skip_serializing_if = "String::is_empty")]
//     pub name: String,
//     #[serde(default, skip_serializing_if = "String::is_empty")]
//     pub description: String,
//     #[serde(default, skip_serializing_if = "Vec::is_empty")]
//     pub authors: Vec<String>,
//     #[serde(default, skip_serializing_if = "Vec::is_empty")]
//     pub dates: Vec<Date>,
//     #[serde(default, skip_serializing_if = "Vec::is_empty")]
//     pub versions: Vec<Version>,
//     #[serde(default, skip_serializing_if = "Vec::is_empty")]
//     pub parameters: Vec<Parameter>,
// }

// impl TryFrom<Metadata> for BTreeMap<String, String> {
//     type Error = ron::Error;

//     fn try_from(value: Metadata) -> Result<Self, Self::Error> {
//         let mut map = BTreeMap::new();
//         if !value.name.is_empty() {
//             map.insert(NAME.to_owned(), value.name);
//         }
//         if !value.description.is_empty() {
//             map.insert(DESCRIPTION.to_owned(), value.description);
//         }
//         if !value.authors.is_empty() {
//             map.insert(AUTHORS.to_owned(), ron::ser::to_string(&value.authors)?);
//         }
//         if !value.parameters.is_empty() {
//             map.insert(
//                 PARAMETERS.to_owned(),
//                 ron::ser::to_string(&value.parameters)?,
//             );
//         }
//         if !value.versions.is_empty() {
//             map.insert(VERSIONS.to_owned(), ron::ser::to_string(&value.versions)?);
//         }
//         if !value.dates.is_empty() {
//             map.insert(DATES.to_owned(), ron::ser::to_string(&value.dates)?);
//         }
//         Ok(map)
//     }
// }

/// Metadata
pub type Metadata = Parameters;

/// Parameters
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Parameters(Vec<Parameter>);

impl Parameters {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, name: impl Into<String>, value: Option<impl ToString>) {
        self.0.push(Parameter {
            name: name.into(),
            value: value.map(|value| value.to_string()),
        });
    }

    /// Оставляет только те параметры, которые попали хотя бы под одно правило,
    /// фильтрует дубликаты согласно `Rule::value` и выстраивает их в порядке правил.
    pub fn filter_and_sort(&mut self, rules: &[Rule]) {
        let parameters = std::mem::take(&mut self.0);

        // Фильтруем по имени
        // Находим первое подходящее правило для каждого параметра
        let rank_value_parameters: Vec<_> = parameters
            .into_iter()
            .filter_map(|parameter| {
                // Ищем подходящее правило
                let (rank, value) = rules.iter().enumerate().find_map(|(rank, rule)| {
                    if rule.name.matches(&parameter.name) {
                        Some((rank, rule.value))
                    } else {
                        None
                    }
                })?;
                Some((rank, value, parameter))
            })
            .collect();

        // Применяем политики First и Last
        let mut keep = vec![true; rank_value_parameters.len()];
        let mut seen_first = HashSet::new();
        let mut seen_last = HashSet::new();
        // Обрабатываем First (идем с начала)
        for (index, (_, value, parameter)) in rank_value_parameters.iter().enumerate() {
            if let Value::First = value {
                // Если имя уже есть в HashSet, значит это не первый элемент -> удаляем
                if !seen_first.insert(&parameter.name) {
                    keep[index] = false;
                }
            }
        }
        // Обрабатываем Last (идем с конца)
        for (index, (_, value, parameter)) in rank_value_parameters.iter().enumerate().rev() {
            if let Value::Last = value {
                // Если имя уже встречалось с конца, значит это не последний элемент -> удаляем
                if !seen_last.insert(&parameter.name) {
                    keep[index] = false;
                }
            }
        }

        // Фильтруем по значению
        // Оставляем только те параметры, которые не были отсеяны политиками First/Last
        let mut rank_parameters: Vec<_> = rank_value_parameters
            .into_iter()
            .enumerate()
            .filter_map(|(index, (rank, _, parameter))| {
                if keep[index] {
                    Some((rank, parameter))
                } else {
                    None
                }
            })
            .collect();

        // Сортируем
        rank_parameters.sort_by(
            |(left_rank, left_parameter), (right_rank, right_parameter)| {
                left_rank
                    .cmp(right_rank)
                    // Если попали под одно и то же правило — сортируем по алфавиту
                    .then_with(|| left_parameter.name.cmp(&right_parameter.name))
                    // Если имена одинаковые — по значению
                    .then_with(|| left_parameter.value.cmp(&right_parameter.value))
            },
        );

        // Убираем ранг и возвращаем обратно в self
        self.0 = rank_parameters
            .into_iter()
            .map(|(_, parameter)| parameter)
            .collect();
    }
}

impl Deref for Parameters {
    type Target = Vec<Parameter>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Parameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Parameter
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Parameter {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl Parameter {
    fn new() -> Self {
        Self {
            name: String::new(),
            value: None,
        }
    }
}

impl Display for Parameter {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name)?;
        if let Some(value) = &self.value {
            write!(f, "={value}")?;
        }
        Ok(())
    }
}

/// Version
#[derive(
    Clone, Copy, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
pub struct Version(pub u64, pub u64, pub u64);

impl Version {
    fn new() -> Self {
        Self(0, 0, 0)
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        if self.0 != 0 {
            write!(f, "{}", self.0)?;
        }
        if self.1 != 0 {
            if self.0 != 0 {
                f.write_str(".")?;
            }
            write!(f, "{}", self.1)?;
        }
        if self.0 != 0 || self.1 != 0 {
            f.write_str(".")?;
        }
        write!(f, "{}", self.2)?;
        Ok(())
    }
}

const GVK: &str = "Giorgi Vladimirovich Kazakov";
const RAS: &str = "Roman Alexandrovich Sidorov";

#[cfg(test)]
mod test {
    use super::*;
    use crate::{r#const::*, format::ParameterFormat, rule::Value};
    use jiff::civil::date;

    #[test]
    fn test() {
        let mut parameters = Parameters::new();
        parameters.push(NAME, Some("VIR-2699"));
        parameters.push(DATE, Some(date(2026, 09, 11)));
        parameters.push(DATE, Some(date(2026, 09, 09)));
        parameters.push(DESCRIPTION, Some("Cat. No. 2699, Прогресс, Россия"));
        parameters.push(AUTHOR, Some(GVK));
        parameters.push(AUTHOR, Some(RAS));
        parameters.push(VERSION, Some(Version(1, 2, 3)));
        parameters.push(VERSION, Some(Version(0, 1, 2)));
        parameters.push("CustomName", Some("CustomValue"));
        println!("parameters: {parameters:#?}");

        parameters.filter_and_sort(&[
            // 1. Сначала `DATE`
            Rule::new(Name::exact(DATE), Value::Last),
            // 2. Затем `NAME`
            Rule::new(Name::or([Name::exact(NAME)]), Value::All),
            // 3. Затем всё остальное, КРОМЕ `AUTHOR`, `DESCRIPTION`, `VERSION`
            Rule::new(
                Name::and([
                    Name::All,
                    Name::not(Name::or(vec![
                        Name::exact(AUTHOR),
                        Name::exact(DESCRIPTION),
                        Name::exact(VERSION),
                    ])),
                ]),
                Value::All,
            ),
            // 4. В самом конце `VERSION`
            Rule::new(Name::exact(VERSION), Value::All),
        ]);
        println!("parameters: {parameters:#?}");
        // parameters.clone().filter_and_sort(&[
        //     Rule::name(DATE),
        //     Rule::name(NAME),
        //     Rule::name(ID),
        //     Rule::name(AUTHOR),
        //     Rule::name(DESCRIPTION),
        //     Rule::name(VERSION),
        //     Rule::All,
        // ]);
        // let mut parameters_format = parameters.format();
        // parameters_format.push(ParameterFormat::new(Name::exact(DATE), Value::Last));
        // parameters_format.push_left(ParameterFormat::new(Filter::Name(DATE), Value::Last));
        // parameters_format.push_right(ParameterFormat::all(Filter::Not(&[
        //     AUTHOR,
        //     DATE,
        //     DESCRIPTION,
        //     NAME,
        //     VERSION,
        // ])));
        // parameters_format.push_right(ParameterFormat::all(Filter::Name(VERSION)));
        // println!(r#"parameters_format: "{parameters_format}""#);

        // let contents = ron::ser::to_string_pretty(&parameters, PRETTY_CONFIG.clone()).unwrap();
        // std::fs::write("path.ron", &contents).unwrap();
        // println!("contents: {contents}");
    }

    // #[test]
    // fn test() {
    //     let meta = Metadata {
    //         name: "VIR-2699".to_owned(),
    //         description: "Cat. No. 2699, Прогресс, Россия".to_owned(),
    //         authors: vec![GVK.to_owned(), RAS.to_owned()],
    //         parameters: Vec::new(),
    //         versions: vec![Version(1, 2, 3), Version(0, 1, 2)],
    //         dates: vec![date(2026, 09, 09), date(2026, 09, 11)],
    //     };
    //     println!("meta: {meta:#?}");

    //     // println!(r#"Dates::All: "{}""#, meta.format().date(Value::All).build());
    //     // println!(
    //     //     r#"Dates::First: "{}""#,
    //     //     meta.format().date(Value::First).build()
    //     // );
    //     // println!(
    //     //     r#"Dates::Last: "{}""#,
    //     //     meta.format().date(Value::Last).build()
    //     // );
    //     // println!(r#"default: "{}""#, meta.format().build());
    //     println!(r#"default: "{}""#, meta.format().build());
    //     println!(
    //         r#"All: "{}""#,
    //         meta.format()
    //             .parameter_format(Some(ParameterFormat::new(DATE, Value::All)))
    //             .parameter_format(Some(ParameterFormat::new(NAME, Value::All)))
    //             .parameter_format(None)
    //             .parameter_format(Some(ParameterFormat::new(VERSION, Value::All)))
    //     );

    //     let contents = ron::ser::to_string_pretty(&meta, PRETTY_CONFIG.clone()).unwrap();
    //     std::fs::write("path.ron", &contents).unwrap();
    //     println!("contents: {contents:?}");
    // }

    // #[test]
    // fn default() {
    //     let meta = Metadata::default();
    //     println!(
    //         r#"Dates::All: "{}""#,
    //         meta.format().date(Value::All).build()
    //     );
    //     println!(
    //         r#"Dates::First: "{}""#,
    //         meta.format().date(Value::First).build()
    //     );
    //     println!(
    //         r#"Dates::Last: "{}""#,
    //         meta.format().date(Value::Last).build()
    //     );
    //     println!(r#"meta: "{}""#, meta.format().build());
    // }
}
