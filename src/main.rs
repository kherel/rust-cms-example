use openssl::cms::{CmsContentInfo, CMSOptions};

use openssl::pkey::PKey;
use openssl::x509::X509;
use std::fs::{self};


fn main() {
    let certificate_contents = fs::read("files/certificate.pem").unwrap();
    let private_key_contents = fs::read("files/private_key.pem").unwrap();
    let image_contents = fs::read("files/image.png").unwrap();

    let signcert = X509::from_pem(&certificate_contents).unwrap();
    let pkey = PKey::private_key_from_pem(&private_key_contents).unwrap();
    
    let flags = CMSOptions::DETACHED;

    
    let cms = CmsContentInfo::sign(
      Some(&signcert),
      Some(&pkey),
      None, 
      Some(&image_contents),
      flags
    ).unwrap();


    fs::write("files/signature.pem", cms.to_pem().unwrap()).unwrap();
}
