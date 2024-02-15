use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TokopediaProduct {
    pub(crate) product_name: Option<String>,
    pub(crate) rating: Option<String>,
    pub(crate) store_name: Option<String>,
    pub(crate) store_location: Option<String>,
    pub(crate) purchase_amount: Option<String>,
    pub(crate) discount_info: Option<String>,
    pub(crate) product_image_url: Option<String>,
}