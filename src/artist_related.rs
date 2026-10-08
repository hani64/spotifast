//! Related artists from the playback session's artist overview.
//!
//! Spotify no longer exposes this list to personal Web API apps. The
//! Pathfinder operation follows Psst's `queryArtistOverview` implementation.

use librespot_core::Session;

use crate::api::ApiError;
use crate::api::models::{Artist, ExternalUrls, Image};
use crate::http::Http;

const ENDPOINT: &str = "https://api-partner.spotify.com/pathfinder/v2/query";
const QUERY_HASH: &str = "1ac33ddab5d39a3a9c27802774e6d78b9405cc188c6f75aed007df2a32737c72";

pub async fn related_artists(
    http: &Http,
    session: &Session,
    id: &str,
) -> Result<Vec<Artist>, ApiError> {
    let token = session
        .login5()
        .auth_token()
        .await
        .map_err(|_| session_error())?;
    let client_token = session
        .spclient()
        .client_token()
        .await
        .map_err(|_| session_error())?;
    let client = http.client().map_err(ApiError::Network)?;
    let response = client
        .post(ENDPOINT)
        .bearer_auth(&token.access_token)
        .header("client-token", &client_token)
        .json(&serde_json::json!({
            "operationName": "queryArtistOverview",
            "variables": {"locale": "", "uri": format!("spotify:artist:{id}")},
            "extensions": {"persistedQuery": {"version": 1, "sha256Hash": QUERY_HASH}}
        }))
        .send()
        .await
        .map_err(|error| ApiError::Network(error.without_url().to_string()))?;
    let status = response.status();
    if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
        return Err(ApiError::RateLimited);
    }
    if !status.is_success() {
        return Err(ApiError::Status {
            status: status.as_u16(),
            message: "Spotify couldn't load related artists through the playback session".into(),
        });
    }
    let body = response
        .json::<serde_json::Value>()
        .await
        .map_err(|_| response_error())?;
    parse_related(body)
}

fn session_error() -> ApiError {
    ApiError::Network("Couldn't authorize the playback session for related artists".into())
}

fn response_error() -> ApiError {
    ApiError::Decode("Related artists are unavailable in Spotify's response".into())
}

fn parse_related(body: serde_json::Value) -> Result<Vec<Artist>, ApiError> {
    if body.get("errors").is_some_and(|errors| {
        !errors.is_null() && errors.as_array().is_none_or(|errors| !errors.is_empty())
    }) {
        return Err(response_error());
    }
    let Some(artist) = body.pointer("/data/artistUnion") else {
        return Err(response_error());
    };
    // Sparse artists can have no relatedContent branch at all.
    let Some(items) = artist
        .pointer("/relatedContent/relatedArtists/items")
        .and_then(serde_json::Value::as_array)
    else {
        return Ok(Vec::new());
    };
    Ok(items
        .iter()
        .filter_map(|item| {
            let id = item.get("id")?.as_str()?;
            if id.len() != 22 || !id.bytes().all(|byte| byte.is_ascii_alphanumeric()) {
                return None;
            }
            let name = item.pointer("/profile/name")?.as_str()?;
            let images = item
                .pointer("/visuals/avatarImage/sources")
                .and_then(serde_json::Value::as_array)
                .into_iter()
                .flatten()
                .filter_map(|source| {
                    Some(Image {
                        url: source.get("url")?.as_str()?.to_owned(),
                        width: source
                            .get("width")
                            .and_then(serde_json::Value::as_u64)
                            .and_then(|width| width.try_into().ok()),
                        height: source
                            .get("height")
                            .and_then(serde_json::Value::as_u64)
                            .and_then(|height| height.try_into().ok()),
                    })
                })
                .collect();
            Some(Artist {
                id: id.into(),
                name: name.into(),
                uri: format!("spotify:artist:{id}"),
                images,
                external_urls: ExternalUrls {
                    spotify: Some(format!("https://open.spotify.com/artist/{id}")),
                },
                ..Artist::default()
            })
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn artist_overview_maps_related_artists_and_ignores_bad_rows() {
        let body = serde_json::json!({
            "data": {"artistUnion": {"relatedContent": {"relatedArtists": {"items": [
                {"id": "1234567890123456789012", "profile": {"name": "Example"},
                 "visuals": {"avatarImage": {"sources": [{"url": "https://i.scdn.co/image/test", "width": 64}]}}},
                {"id": "invalid", "profile": {"name": "No"}}
            ]}}}}
        });
        let artists = parse_related(body).unwrap();
        assert_eq!(artists.len(), 1);
        assert_eq!(artists[0].name, "Example");
        assert_eq!(artists[0].images[0].width, Some(64));
        assert_eq!(artists[0].uri, "spotify:artist:1234567890123456789012");
    }

    #[test]
    fn sparse_artist_without_related_content_is_empty() {
        assert!(
            parse_related(serde_json::json!({"data": {"artistUnion": {"profile": null}}}))
                .unwrap()
                .is_empty()
        );
        assert!(parse_related(serde_json::json!({"data": {}})).is_err());
    }
}
