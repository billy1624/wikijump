/*
 * methods/locales.rs
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

use super::prelude::*;
use crate::locales::MessageArguments;
use ref_map::*;
use unic_langid::LanguageIdentifier;

pub async fn locale_head(req: ApiRequest) -> ApiResponse {
    locale_get(req).await?;
    Ok(Response::new(StatusCode::NoContent))
}

#[derive(Serialize, Debug)]
struct LocaleOutput<'a> {
    language: &'a str,
    script: Option<&'a str>,
    region: Option<&'a str>,
    variants: Vec<String>,
}

pub async fn locale_get(req: ApiRequest) -> ApiResponse {
    let locale_str = req.param("locale")?;
    tide::log::info!("Getting locale information for {}", locale_str);

    let locale = LanguageIdentifier::from_bytes(locale_str.as_bytes())
        .map_err(|error| TideError::new(StatusCode::BadRequest, error))?;

    let output = LocaleOutput {
        language: locale.language.as_str(),
        script: locale.script.ref_map(|s| s.as_str()),
        region: locale.region.ref_map(|s| s.as_str()),
        variants: locale.variants().map(|v| v.as_str().into()).collect(),
    };

    let body = Body::from_json(&output)?;
    Ok(body.into())
}

pub async fn message_head(req: ApiRequest) -> ApiResponse {
    let locale_str = req.param("locale")?;
    let message_key = req.param("message_key")?;
    tide::log::info!(
        "Checking existence of message key {} in locale {}",
        message_key,
        locale_str,
    );

    let locale = LanguageIdentifier::from_bytes(locale_str.as_bytes())?;

    let result = req.state().localizations.has_message(&locale, message_key);
    if result {
        Ok(Response::new(StatusCode::NoContent))
    } else {
        Ok(Response::new(StatusCode::NotFound))
    }
}

pub async fn message_post(mut req: ApiRequest) -> ApiResponse {
    let input: MessageArguments = req.body_json().await?;
    let locale_str = req.param("locale")?;
    let message_key = req.param("message_key")?;
    tide::log::info!(
        "Formatting message key {} in locale {}",
        message_key,
        locale_str,
    );

    let locale = LanguageIdentifier::from_bytes(locale_str.as_bytes())?;
    let arguments = input.into_fluent_args();

    let result = req
        .state()
        .localizations
        .translate(&locale, message_key, &arguments);

    match result {
        Ok(message) => Ok(message.as_ref().into()),
        Err(error) => Err(ServiceError::from(error).into_tide_error()),
    }
}
