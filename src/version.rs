use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

/// Версия `major.minor.patch`.
#[derive(
    Clone, Copy, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
pub struct Version(pub u64, pub u64, pub u64);

/// Компактный вывод: нулевые `major` и `minor` опускаются.
///
/// Внимание: вывод неоднозначен (`1.0.5` и `0.1.5` оба дают `1.5`) и не
/// разбирается обратно. Поведение сохранено как есть, но его стоит пересмотреть.
impl Display for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let Self(major, minor, patch) = *self;
        if major != 0 {
            write!(f, "{major}")?;
        }
        if minor != 0 {
            if major != 0 {
                f.write_str(".")?;
            }
            write!(f, "{minor}")?;
        }
        if major != 0 || minor != 0 {
            f.write_str(".")?;
        }
        write!(f, "{patch}")
    }
}
