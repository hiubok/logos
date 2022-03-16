use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Claims {
    pub claims: Option<Vec<Claim>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Claim {
    pub text: String,
    pub claimant: Option<String>,
    pub claim_date: Option<String>,
    pub claim_review: Vec<ClaimReview>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClaimReview {
    pub publisher: Publisher,
    pub url: String,
    pub title: Option<String>,
    pub textual_rating: String,
    pub language_code: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Publisher {
    pub name: Option<String>,
    pub site: Option<String>,
}
