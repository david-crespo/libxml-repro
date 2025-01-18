use samael::metadata::{EntityDescriptor, NameIdFormat};
use samael::service_provider::ServiceProviderBuilder;

const SAML_RESPONSE_IDP_DESCRIPTOR: &str = include_str!(
    "../../omicron/nexus/tests/integration_tests/data/saml_response_idp_descriptor.xml"
);

const SAML_RESPONSE: &str = include_str!("../base64_saml_response.txt");

fn main() {
    let mut sp_builder = ServiceProviderBuilder::default();
    sp_builder.acs_url(Some("https://customer.site/oxide_rack/saml".to_string()));
    sp_builder.slo_url(Some("https://customer.site/oxide_rack/saml".to_string()));
    sp_builder.authn_name_id_format(Some(NameIdFormat::UnspecifiedNameIDFormat.value().into()));

    let idp_metadata: EntityDescriptor = SAML_RESPONSE_IDP_DESCRIPTOR.parse().unwrap();
    sp_builder.idp_metadata(idp_metadata);

    let sp = sp_builder.build().unwrap();

    let _ = dbg!(sp.parse_base64_response(SAML_RESPONSE.trim(), None));
}
