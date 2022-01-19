/*
 * methods/page.rs
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
use crate::models::page::Model as PageModel;
use crate::models::page_revision::Model as PageRevisionModel;
use crate::services::page::CreatePage;
use ftml::data::Backlinks;

#[derive(Serialize, Debug)]
struct PageOutput<'a> {
    page: &'a PageModel,
    revision: &'a PageRevisionModel,
}

pub async fn page_invalid(req: ApiRequest) -> ApiResponse {
    tide::log::warn!("Received invalid /page path: {}", req.url());
    Ok(Response::new(StatusCode::BadRequest))
}

pub async fn page_create(mut req: ApiRequest) -> ApiResponse {
    let txn = req.database().begin().await?;
    let ctx = ServiceContext::new(&req, &txn);

    let input: CreatePage = req.body_json().await?;
    let site_id = req.param("site_id")?.parse()?;
    let output = PageService::create(&ctx, site_id, input).await.to_api()?;
    let body = Body::from_json(&output)?;
    txn.commit().await?;

    Ok(body.into())
}

pub async fn page_head(req: ApiRequest) -> ApiResponse {
    let txn = req.database().begin().await?;
    let ctx = ServiceContext::new(&req, &txn);

    let site_id = req.param("site_id")?.parse()?;
    let reference = Reference::try_from(&req)?;
    let exists = PageService::exists(&ctx, site_id, reference)
        .await
        .to_api()?;
    txn.commit().await?;

    if exists {
        Ok(Response::new(StatusCode::NoContent))
    } else {
        Ok(Response::new(StatusCode::NotFound))
    }
}

pub async fn page_get(req: ApiRequest) -> ApiResponse {
    let txn = req.database().begin().await?;
    let ctx = ServiceContext::new(&req, &txn);

    let site_id = req.param("site_id")?.parse()?;
    let reference = Reference::try_from(&req)?;
    let page = PageService::get(&ctx, site_id, reference).await.to_api()?;
    let revision = RevisionService::get_latest(&ctx, site_id, page.page_id)
        .await
        .to_api()?;
    txn.commit().await?;

    build_page_response(&page, &revision, StatusCode::Ok)
}

pub async fn page_head_direct(req: ApiRequest) -> ApiResponse {
    let txn = req.database().begin().await?;
    let ctx = ServiceContext::new(&req, &txn);

    let page_id = req.param("page_id")?.parse()?;
    let exists = PageService::exists_direct(&ctx, page_id).await.to_api()?;
    txn.commit().await?;
    exists_status(exists)
}

pub async fn page_get_direct(req: ApiRequest) -> ApiResponse {
    let txn = req.database().begin().await?;
    let ctx = ServiceContext::new(&req, &txn);

    let page_id = req.param("page_id")?.parse()?;
    let page = PageService::get_direct(&ctx, page_id).await.to_api()?;
    let revision = RevisionService::get_latest(&ctx, page.site_id, page.page_id)
        .await
        .to_api()?;
    txn.commit().await?;

    build_page_response(&page, &revision, StatusCode::Ok)
}

pub async fn page_edit(req: ApiRequest) -> ApiResponse {
    let txn = req.database().begin().await?;
    let ctx = ServiceContext::new(&req, &txn);

    todo!()
}

pub async fn page_delete(req: ApiRequest) -> ApiResponse {
    let txn = req.database().begin().await?;
    let ctx = ServiceContext::new(&req, &txn);

    let site_id = req.param("site_id")?.parse()?;
    let reference = Reference::try_from(&req)?;
    PageService::delete(&ctx, site_id, reference)
        .await
        .to_api()?;

    txn.commit().await?;
    Ok(Response::new(StatusCode::NoContent))
}

pub async fn page_links_from_get(req: ApiRequest) -> ApiResponse {
    let txn = req.database().begin().await?;
    let ctx = ServiceContext::new(&req, &txn);

    let site_id = req.param("site_id")?.parse()?;
    let reference = Reference::try_from(&req)?;
    let output = LinkService::get_from(&ctx, site_id, reference)
        .await
        .to_api()?;

    let body = Body::from_json(&output)?;
    txn.commit().await?;

    Ok(body.into())
}

pub async fn page_links_to_get(req: ApiRequest) -> ApiResponse {
    let txn = req.database().begin().await?;
    let ctx = ServiceContext::new(&req, &txn);

    let site_id = req.param("site_id")?.parse()?;
    let reference = Reference::try_from(&req)?;
    let output = LinkService::get_to(&ctx, site_id, reference)
        .await
        .to_api()?;

    let body = Body::from_json(&output)?;
    txn.commit().await?;

    Ok(body.into())
}

pub async fn page_links_to_missing_get(req: ApiRequest) -> ApiResponse {
    let txn = req.database().begin().await?;
    let ctx = ServiceContext::new(&req, &txn);

    let site_id = req.param("site_id")?.parse()?;
    let page_slug = req.param("page_slug")?;
    let output = LinkService::get_to_missing(&ctx, site_id, page_slug)
        .await
        .to_api()?;

    let body = Body::from_json(&output)?;
    txn.commit().await?;

    Ok(body.into())
}

// TODO: remove separate endpoint, make part of revision changes
pub async fn page_links_put(mut req: ApiRequest) -> ApiResponse {
    let txn = req.database().begin().await?;
    let ctx = ServiceContext::new(&req, &txn);
    let backlinks: Backlinks = req.body_json().await?;

    let site_id = req.param("site_id")?.parse()?;
    let reference = Reference::try_from(&req)?;
    LinkService::update(&ctx, site_id, reference, &backlinks)
        .await
        .to_api()?;

    txn.commit().await?;

    Ok(Response::new(StatusCode::NoContent))
}

// TODO: remove separate endpoint, make part of revision changes
pub async fn page_links_missing_put(mut req: ApiRequest) -> ApiResponse {
    let txn = req.database().begin().await?;
    let ctx = ServiceContext::new(&req, &txn);
    let backlinks: Backlinks = req.body_json().await?;

    let site_id = req.param("site_id")?.parse()?;
    let slug = req.param("slug")?;
    LinkService::update_missing(&ctx, site_id, slug, &backlinks)
        .await
        .to_api()?;

    txn.commit().await?;

    Ok(Response::new(StatusCode::NoContent))
}

fn build_page_response(
    page: &PageModel,
    revision: &PageRevisionModel,
    status: StatusCode,
) -> ApiResponse {
    let body = Body::from_json(&PageOutput { page, revision })?;
    let response = Response::builder(status).body(body).into();
    Ok(response)
}
