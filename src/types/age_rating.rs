use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgeRatingDeclarationAttributes {
    pub alcohol_tobacco_or_drug_use_or_references: Option<String>,
    pub contests: Option<String>,
    pub gambling_and_contests: Option<String>,
    pub gambling: Option<bool>,
    pub gambling_simulated: Option<String>,
    pub horror_or_fear_themes: Option<String>,
    pub mature_or_suggestive_themes: Option<String>,
    pub medical_or_treatment_information: Option<String>,
    pub profanity_or_crude_humor: Option<String>,
    pub sexual_content_graphic_and_nudity: Option<String>,
    pub sexual_content_or_nudity: Option<String>,
    pub violence_cartoon_or_fantasy: Option<String>,
    pub violence_realistic: Option<String>,
    pub violence_realistic_prolonged_graphic_or_sadistic: Option<String>,
    pub seventeen_plus: Option<bool>,
    pub unresolved_gambling_apps: Option<bool>,
    pub kids_age_band: Option<String>,
}
