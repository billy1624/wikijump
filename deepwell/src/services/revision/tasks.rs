/*
 * services/revision/tasks.rs
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

use super::structs::CreateRevisionBody;
use crate::models::page_revision::Model as PageRevisionModel;
use crate::services::TextService;
use crate::web::ProvidedValue;

/// A representation of the updating tasks to do for a revision.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct RevisionTasks {
    pub render: bool,
    pub links_incoming: bool,
    pub links_outgoing: bool,
    pub rename: bool,
    pub rerender_included: bool,
    pub process_navigation: bool,
    pub process_templates: bool,
}

impl RevisionTasks {
    pub fn determine(revision: &PageRevisionModel, changes: &CreateRevisionBody) -> Self {
        let mut tasks = RevisionTasks {
            render: false,
            links_incoming: false,
            links_outgoing: false,
            rename: false,
            rerender_included: false,
            process_navigation: false,
            process_templates: false,
        };

        if let ProvidedValue::Set(ref wikitext) = changes.wikitext {
            if revision.wikitext_hash.as_slice() != TextService::hash(wikitext).as_slice()
            {
                tasks.render = true;
                tasks.links_incoming = true;
                tasks.links_outgoing = true;
                tasks.rerender_included = true;
                tasks.process_navigation = true;
                tasks.process_templates = true;
            }
        }

        // Don't need to check changes.hidden

        if let ProvidedValue::Set(ref title) = changes.title {
            if &revision.title != title {
                tasks.render = true;
                tasks.links_incoming = true;
                tasks.links_outgoing = true;
            }
        }

        if let ProvidedValue::Set(ref alt_title) = changes.alt_title {
            if &revision.alt_title != alt_title {
                tasks.render = true;
                tasks.links_incoming = true;
                tasks.links_outgoing = true;
            }
        }

        if let ProvidedValue::Set(ref slug) = changes.slug {
            if &revision.slug != slug {
                tasks.render = true;
                tasks.links_incoming = true;
                tasks.rename = true;
                tasks.rerender_included = true;
                tasks.process_navigation = true;
                tasks.process_templates = true;
            }
        }

        if let ProvidedValue::Set(ref _tags) = changes.tags {
            // TODO check tags
            if false {
                tasks.render = true;
            }
        }

        if let ProvidedValue::Set(ref _metadata) = changes.metadata {
            // TODO check metadata
            if false {
                tasks.render = true;
            }
        }

        tasks
    }

    #[inline]
    pub fn is_empty(self) -> bool {
        !self.render
            && !self.links_incoming
            && !self.links_outgoing
            && !self.rename
            && !self.rerender_included
            && !self.process_navigation
            && !self.process_templates
    }
}

/*
 * TODO: Tasks for other page changes:
 *
 * page delete:
 * - links_incoming
 * - rerender_included
 * - process_navigation
 * - process_templates
 * - outdate page cache
 *
 * page parent change:
 * - outdate page cache
 *
 * page file change:
 * - render
 * - rerender_included
 * - outdate page cache
 *
 * page vote:
 * - outdate page cache
 */
