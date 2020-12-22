/*
 * render/mod.rs
 *
 * ftml - Library to parse Wikidot code
 * Copyright (C) 2019-2020 Ammon Smith
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

mod prelude {
    pub use super::Render;
    pub use crate::tree::{Container, ContainerType, Element, SyntaxTree};
}

mod debug;
mod html;
mod json;
mod null;
mod object;

pub use self::debug::DebugRender;
pub use self::html::HtmlRender;
pub use self::json::JsonRender;
pub use self::null::NullRender;
pub use self::object::Render;