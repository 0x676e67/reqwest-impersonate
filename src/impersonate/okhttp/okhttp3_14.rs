use super::base_ssl_builder;
use crate::impersonate::{
    BoringTlsConnector, Http2Data, ImpersonateSettings,
};
use boring::{
    error::ErrorStack,
    ssl::SslConnectorBuilder,
};
use http::{
    header::{ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, USER_AGENT},
    HeaderMap, HeaderValue,
};

pub(crate) fn get_settings(headers: HeaderMap) -> ImpersonateSettings {
    ImpersonateSettings {
        tls_connector: BoringTlsConnector::new(ssl_builder),
        http2: Http2Data {
            initial_stream_window_size: Some(16777216),
            initial_connection_window_size: Some(16777216),
            max_concurrent_streams: None,
            max_header_list_size: None,
            header_table_size: None,
            enable_push: None,
        },
        headers: create_headers(headers),
        gzip: true,
        brotli: true,
    }
}

fn ssl_builder() -> Result<SslConnectorBuilder, ErrorStack> {
    let mut builder = base_ssl_builder()?;

    let cipher_list = [
        "TLS_AES_128_GCM_SHA256",
        "TLS_AES_256_GCM_SHA384",
        "TLS_CHACHA20_POLY1305_SHA256",
        "TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256",
        "TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256",
        "TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384",
        "TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384",
        "TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256",
        "TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256",
        "TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA",
        "TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA",
        "TLS_RSA_WITH_AES_128_GCM_SHA256",
        "TLS_RSA_WITH_AES_256_GCM_SHA384",
        "TLS_RSA_WITH_AES_128_CBC_SHA",
        "TLS_RSA_WITH_AES_256_CBC_SHA",
        "TLS_RSA_WITH_3DES_EDE_CBC_SHA",
    ];

    builder.set_cipher_list(&cipher_list.join(":"))?;

    Ok(builder)
}

fn create_headers(mut headers: HeaderMap) -> HeaderMap {
    headers.insert(ACCEPT, HeaderValue::from_static("*/*"));
    headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US,en;q=0.9"));
    headers.insert(USER_AGENT, HeaderValue::from_static("DS podcast/2.0.1 (be.standaard.audio; build:9; Android 11; Sdk:30; Manufacturer:samsung; Model: SM-A405FN) OkHttp/3.14.0"));
    headers.insert(
        ACCEPT_ENCODING,
        HeaderValue::from_static("gzip, deflate, br"),
    );

    headers
}
