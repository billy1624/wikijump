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

#[derive(
    Serialize, Deserialize, Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq,
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

impl Default for UserDetails {
    #[inline]
    fn default() -> Self {
        UserDetails::Identity
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserDetailsQuery {
    #[serde(default)]
    pub detail: UserDetails,
}
