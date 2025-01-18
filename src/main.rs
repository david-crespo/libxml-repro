use samael::metadata::{EntityDescriptor, NameIdFormat};
use samael::service_provider::ServiceProviderBuilder;

// from https://github.com/oxidecomputer/omicron/blob/9093ac615027f7050eaaa14d0ec914ae3abcfb33/nexus/tests/integration_tests/data/saml_response_idp_descriptor.xml
const SAML_RESPONSE_IDP_DESCRIPTOR: &str = include_str!("../saml_response_idp_descriptor.xml");
// copied from debug print in test
const SAML_RESPONSE: &str = include_str!("../base64_saml_response.txt");

fn main() {
    let mut sp_builder = ServiceProviderBuilder::default();
    let idp_metadata: EntityDescriptor = SAML_RESPONSE_IDP_DESCRIPTOR.parse().unwrap();
    sp_builder.idp_metadata(idp_metadata);

    let sp = sp_builder.build().unwrap();

    let _ = dbg!(sp.parse_base64_response(SAML_RESPONSE.trim(), None));
}

// func=xmlSecXPathDataExecute:file=xpath.c:line=245:obj=unknown:subj=xmlXPtrEval:error=5:libxml2 library function failed:expr=xpointer(id('_f2f02ca12d7292f8a4e03b7b99d71a45f8896c2677')); xml error: 0: NULL
// func=xmlSecXPathDataListExecute:file=xpath.c:line=332:obj=unknown:subj=xmlSecXPathDataExecute:error=1:xmlsec library function failed:
// func=xmlSecTransformXPathExecute:file=xpath.c:line=430:obj=xpointer:subj=xmlSecXPathDataListExecute:error=1:xmlsec library function failed:
// func=xmlSecTransformDefaultPushXml:file=transforms.c:line=2146:obj=xpointer:subj=xmlSecTransformExecute:error=1:xmlsec library function failed:
// func=xmlSecTransformCtxXmlExecute:file=transforms.c:line=1080:obj=xpointer:subj=xmlSecTransformPushXml:error=1:xmlsec library function failed:
// func=xmlSecTransformCtxExecute:file=transforms.c:line=1127:obj=unknown:subj=xmlSecTransformCtxXmlExecute:error=1:xmlsec library function failed:
// func=xmlSecDSigReferenceCtxProcessNode:file=xmldsig.c:line=1472:obj=unknown:subj=xmlSecTransformCtxExecute:error=1:xmlsec library function failed:
// func=xmlSecDSigCtxProcessReferences:file=xmldsig.c:line=783:obj=Reference:subj=xmlSecDSigReferenceCtxProcessNode:error=1:xmlsec library function failed:
// func=xmlSecDSigCtxProcessSignatureNode:file=xmldsig.c:line=546:obj=unknown:subj=xmlSecDSigCtxProcessReferences:error=1:xmlsec library function failed:
// func=xmlSecDSigCtxVerify:file=xmldsig.c:line=357:obj=unknown:subj=xmlSecDSigCtxProcessSignatureNode:error=1:xmlsec library function failed:
// [src/main.rs:16:13] sp.parse_base64_response(SAML_RESPONSE.trim(), None) = Err(
//     FailedToValidateSignature,
// )


// This is the line that throws FailedToValidateSignature
// https://github.com/njaremko/samael/blob/4324241fdd926b99fc1454289c2e481dda247d88/src/service_provider/mod.rs#L321-L328
