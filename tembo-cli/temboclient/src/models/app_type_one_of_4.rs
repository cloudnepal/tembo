/*
 * Tembo Cloud
 *
 * Platform API for Tembo Cloud             </br>             </br>             To find a Tembo Data API, please find it here:             </br>             </br>             [AWS US East 1](https://api.data-1.use1.tembo.io/swagger-ui/)
 *
 * The version of the OpenAPI document: v1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppTypeOneOf4 {
    #[serde(rename = "custom")]
    pub custom: Box<crate::models::AppService>,
}

impl AppTypeOneOf4 {
    pub fn new(custom: crate::models::AppService) -> AppTypeOneOf4 {
        AppTypeOneOf4 {
            custom: Box::new(custom),
        }
    }
}