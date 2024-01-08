use openssl::cms::{CmsContentInfo, CMSOptions};

use openssl::pkey::PKey;
use openssl::x509::X509;
use std::fs;


fn main() {
    let certificate_contents = fs::read("files/certificate.pem").unwrap();
    let private_key_contents = fs::read("files/private_key.pem").unwrap();
    let image_contents = fs::read("files/image.png").unwrap();
    let signcert = X509::from_pem(&certificate_contents).unwrap();
    let pkey = PKey::private_key_from_pem(&private_key_contents).unwrap();
    

  // Sign the image with image included in the signature
    let cms = CmsContentInfo::sign(
      Some(&signcert),
      Some(&pkey),
      None, 
      Some(&image_contents),
      CMSOptions::BINARY | CMSOptions::DETACHED
    ).unwrap();

    fs::write("files/signature.pem", cms.to_pem().unwrap()).unwrap();
    // to verify the signature
    // openssl cms -verify -in signature.pem -inform PEM -content image.png -CAfile certificate.pem -out verified_image.png -noverify -binary

    // Sign the image with image included in the signature
    // let cms = CmsContentInfo::sign(
    //   Some(&signcert),
    //   Some(&pkey),
    //   None, 
    //   Some(&image_contents),
    //   CMSOptions::BINARY
    // ).unwrap();
    

    // fs::write("files/signature.cms", cms.to_der().unwrap()).unwrap();
    // openssl cms -verify -in signature.cms -inform DER -CAfile certificate.pem -out verified_image2.png -noverify
}
