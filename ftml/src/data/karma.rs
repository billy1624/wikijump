/*
 * data/karma.rs
 *
 * ftml - Library to parse Wikidot text
 * Copyright (C) 2019-2021 Wikijump Team
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

use std::convert::TryFrom;
use std::fmt::{self, Display};

/// Represents the Karma level a user has.
#[derive(Serialize_repr, Deserialize_repr, Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum KarmaLevel {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
}

impl KarmaLevel {
    #[inline]
    pub fn new(value: u8) -> Option<Self> {
        KarmaLevel::try_from(value).ok()
    }

    #[inline]
    pub fn value(self) -> u8 {
        self.into()
    }
}

impl From<KarmaLevel> for u8 {
    #[inline]
    fn from(level: KarmaLevel) -> u8 {
        match level {
            KarmaLevel::Zero => 0,
            KarmaLevel::One => 1,
            KarmaLevel::Two => 2,
            KarmaLevel::Three => 3,
            KarmaLevel::Four => 4,
            KarmaLevel::Five => 5,
        }
    }
}

impl TryFrom<u8> for KarmaLevel {
    type Error = u8;

    fn try_from(value: u8) -> Result<KarmaLevel, u8> {
        match value {
            0 => Ok(KarmaLevel::Zero),
            1 => Ok(KarmaLevel::One),
            2 => Ok(KarmaLevel::Two),
            3 => Ok(KarmaLevel::Three),
            4 => Ok(KarmaLevel::Four),
            5 => Ok(KarmaLevel::Five),
            _ => Err(value),
        }
    }
}

impl Display for KarmaLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}
