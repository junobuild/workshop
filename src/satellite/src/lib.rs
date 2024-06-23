mod http;
mod http_request;
mod http_response;

use crate::http::transform_response;
use http::query;
use ic_cdk::api::management_canister::http_request::{HttpResponse, TransformArgs};
use junobuild_macros::{
    assert_delete_asset, assert_delete_doc, assert_set_doc, assert_upload_asset, on_delete_asset,
    on_delete_doc, on_delete_many_assets, on_delete_many_docs, on_set_doc, on_set_many_docs,
    on_upload_asset,
};
use junobuild_satellite::{
    include_satellite, set_doc_store, AssertDeleteAssetContext, AssertDeleteDocContext,
    AssertSetDocContext, AssertUploadAssetContext, OnDeleteAssetContext, OnDeleteDocContext,
    OnDeleteManyAssetsContext, OnDeleteManyDocsContext, OnSetDocContext, OnSetManyDocsContext,
    OnUploadAssetContext, SetDoc,
};
use junobuild_utils::{decode_doc_data, encode_doc_data};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct NotesData {
    text: String,
    url: Option<String>,
}

#[on_set_doc(collections = ["notes"])]
async fn on_set_doc(context: OnSetDocContext) -> Result<(), String> {
    let mut data: NotesData = decode_doc_data(&context.data.data.after.data)?;

    let image_url = query(&context.data.key, &data.text).await?;

    data.url = Some(image_url);

    let encode_data = encode_doc_data(&data)?;

    let doc: SetDoc = SetDoc {
        data: encode_data,
        description: context.data.data.after.description,
        version: context.data.data.after.version,
    };

    set_doc_store(
        context.caller,
        context.data.collection,
        context.data.key,
        doc,
    )?;

    Ok(())
}

#[on_set_many_docs]
async fn on_set_many_docs(_context: OnSetManyDocsContext) -> Result<(), String> {
    Ok(())
}

#[on_delete_doc]
async fn on_delete_doc(_context: OnDeleteDocContext) -> Result<(), String> {
    Ok(())
}

#[on_delete_many_docs]
async fn on_delete_many_docs(_context: OnDeleteManyDocsContext) -> Result<(), String> {
    Ok(())
}

#[on_upload_asset]
async fn on_upload_asset(_context: OnUploadAssetContext) -> Result<(), String> {
    Ok(())
}

#[on_delete_asset]
async fn on_delete_asset(_context: OnDeleteAssetContext) -> Result<(), String> {
    Ok(())
}

#[on_delete_many_assets]
async fn on_delete_many_assets(_context: OnDeleteManyAssetsContext) -> Result<(), String> {
    Ok(())
}

#[assert_set_doc]
fn assert_set_doc(_context: AssertSetDocContext) -> Result<(), String> {
    Ok(())
}

#[assert_delete_doc]
fn assert_delete_doc(_context: AssertDeleteDocContext) -> Result<(), String> {
    Ok(())
}

#[assert_upload_asset]
fn assert_upload_asset(_context: AssertUploadAssetContext) -> Result<(), String> {
    Ok(())
}

#[assert_delete_asset]
fn assert_delete_asset(_context: AssertDeleteAssetContext) -> Result<(), String> {
    Ok(())
}

#[ic_cdk_macros::query]
fn transform(raw: TransformArgs) -> HttpResponse {
    transform_response(raw)
}

include_satellite!();
