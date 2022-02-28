/*
 * web/user_details.rs
 *
 * DEEPWELL - Wikijump API provider and database manager
 * Copyright (C) 2021 Wikijump Team
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

use crate::services::Error as ServiceError;
use strum_macros::EnumIter;

#[derive(
    EnumIter,
    Serialize,
    Deserialize,
    Debug,
    Copy,
    Clone,
    Hash,
    PartialOrd,
    Ord,
    PartialEq,
    Eq,
)]
#[serde(rename_all = "camelCase")]
pub enum UserDetails {
    /// Basic level of information needed to describe a user.
    ///
    /// Associated with the struct `UserIdentityOutput`.
    Identity,

    /// Describes a user in an intermediate amount of detail.
    ///
    /// Associated with the struct `UserInfoOutput`.
    Info,

    /// Fully describes a user and their associated data.
    ///
    /// Associated with the struct `UserProfileOutput`.
    Profile,
}

impl UserDetails {
    #[inline]
    pub fn name(self) -> &'static str {
        match self {
            UserDetails::Identity => "identity",
            UserDetails::Info => "info",
            UserDetails::Profile => "profile",
        }
    }
}

impl TryFrom<&'_ str> for UserDetails {
    type Error = ServiceError;

    fn try_from(value: &'_ str) -> Result<UserDetails, ServiceError> {
        match value {
            "identity" => Ok(UserDetails::Identity),
            "info" => Ok(UserDetails::Info),
            "profile" => Ok(UserDetails::Profile),
            _ => Err(ServiceError::InvalidEnumValue),
        }
    }
}

impl Default for UserDetails {
    #[inline]
    fn default() -> Self {
        UserDetails::Identity
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct UserDetailsQuery {
    #[serde(default)]
    pub detail: UserDetails,
}

/// Ensure `UserDetails::name()` produces the same output as serde.
#[test]
fn name_serde() {
    use strum::IntoEnumIterator;

    for variant in UserDetails::iter() {
        let output = serde_json::to_string(&variant).expect("Unable to serialize JSON");
        let serde_name: String =
            serde_json::from_str(&output).expect("Unable to deserialize JSON");

        assert_eq!(
            &serde_name,
            variant.name(),
            "Serde name does not match variant name",
        );

        let converted: UserDetails = serde_name
            .as_str()
            .try_into()
            .expect("Could not convert item");

        assert_eq!(converted, variant, "Converted item does not match variant");
    }
}
