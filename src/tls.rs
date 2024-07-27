//! TLS configuration
//!
//! By default, a `Client` will make use of system-native transport layer
//! security to connect to HTTPS destinations. This means schannel on Windows,
//! Security-Framework on macOS, and OpenSSL on Linux.
//!
//! - Additional X509 certificates can be configured on a `ClientBuilder` with the
//!   [`Certificate`] type.
//! - Client certificates can be added to a `ClientBuilder` with the
//!   [`Identity`] type.
//! - Various parts of TLS can also be configured or even disabled on the
//!   `ClientBuilder`.

use boring::ssl::SslConnectorBuilder;
use std::fmt;
#[cfg(feature = "__boring")]
use std::sync::Arc;
#[cfg(feature = "__boring")]
use crate::impersonate::BoringTlsConnector;

/// A TLS protocol version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version(InnerVersion);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
enum InnerVersion {
    Tls1_0,
    Tls1_1,
    Tls1_2,
    Tls1_3,
}

// These could perhaps be From/TryFrom implementations, but those would be
// part of the public API so let's be careful
impl Version {
    /// Version 1.0 of the TLS protocol.
    pub const TLS_1_0: Version = Version(InnerVersion::Tls1_0);
    /// Version 1.1 of the TLS protocol.
    pub const TLS_1_1: Version = Version(InnerVersion::Tls1_1);
    /// Version 1.2 of the TLS protocol.
    pub const TLS_1_2: Version = Version(InnerVersion::Tls1_2);
    /// Version 1.3 of the TLS protocol.
    pub const TLS_1_3: Version = Version(InnerVersion::Tls1_3);
}

pub(crate) enum TlsBackend {
    #[cfg(feature = "__boring")]
    BoringTls(BoringTlsConnector),
    #[cfg(not(feature = "__boring"))]
    UnknownPreconfigured,
}

impl fmt::Debug for TlsBackend {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            #[cfg(feature = "__boring")]
            TlsBackend::BoringTls(_) => write!(f, "BoringTls"),
            #[cfg(not(feature = "__boring"))]
            TlsBackend::UnknownPreconfigured => write!(f, "UnknownPreconfigured"),
        }
    }
}

impl Default for TlsBackend {
    fn default() -> TlsBackend {
        #[cfg(feature = "__boring")]
        {
            use boring::ssl::{SslConnector, SslMethod};

            fn create_builder() -> SslConnectorBuilder {
                SslConnector::builder(SslMethod::tls()).unwrap()
            }
            TlsBackend::BoringTls(BoringTlsConnector::new(Arc::new(create_builder)))
        }
        #[cfg(not(feature = "__boring"))]
        {
            TlsBackend::UnknownPreconfigured
        }
    }
}
/// Hyper extension carrying extra TLS layer information.
/// Made available to clients on responses when `tls_info` is set.
#[derive(Clone)]
pub struct TlsInfo {
    pub(crate) peer_certificate: Option<Vec<u8>>,
}

impl TlsInfo {
    /// Get the DER encoded leaf certificate of the peer.
    pub fn peer_certificate(&self) -> Option<&[u8]> {
        self.peer_certificate.as_ref().map(|der| &der[..])
    }
}

impl std::fmt::Debug for TlsInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TlsInfo").finish()
    }
}
