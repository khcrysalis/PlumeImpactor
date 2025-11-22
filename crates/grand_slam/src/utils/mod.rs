mod certificate;
mod provision;
mod macho;
mod signer;
mod bundle;

pub use macho::MachO;
pub use provision::MobileProvision;
pub use certificate::CertificateIdentity;
pub use signer::Signer;
pub use bundle::Bundle;
pub use bundle::BundleType;

pub trait PlistInfoTrait {
    fn get_name(&self) -> Option<String>;
    fn get_executable(&self) -> Option<String>;
    fn get_bundle_identifier(&self) -> Option<String>;
    fn get_version(&self) -> Option<String>;
    fn get_build_version(&self) -> Option<String>;
}
