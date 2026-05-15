use crate::client::ApiClient;
use crate::error::CliError;
use crate::types::age_rating::AgeRatingDeclarationAttributes;
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum AgeRatingDeclarationsCommands {
    /// Get the age rating declaration for an app info
    Get {
        /// App Info ID
        #[arg(long)]
        app_info_id: String,
    },
    /// Update an age rating declaration
    Update {
        /// Age rating declaration ID
        #[arg(long)]
        id: String,
        /// Alcohol, tobacco, or drug use or references (NONE, INFREQUENT_OR_MILD, FREQUENT_OR_INTENSE)
        #[arg(long)]
        alcohol_tobacco_or_drug_use_or_references: Option<String>,
        /// Contests (NONE, INFREQUENT_OR_MILD, FREQUENT_OR_INTENSE)
        #[arg(long)]
        contests: Option<String>,
        /// Gambling and contests
        #[arg(long)]
        gambling_and_contests: Option<String>,
        /// Gambling
        #[arg(long)]
        gambling: Option<bool>,
        /// Simulated gambling (NONE, INFREQUENT_OR_MILD, FREQUENT_OR_INTENSE)
        #[arg(long)]
        gambling_simulated: Option<String>,
        /// Horror or fear themes (NONE, INFREQUENT_OR_MILD, FREQUENT_OR_INTENSE)
        #[arg(long)]
        horror_or_fear_themes: Option<String>,
        /// Mature or suggestive themes (NONE, INFREQUENT_OR_MILD, FREQUENT_OR_INTENSE)
        #[arg(long)]
        mature_or_suggestive_themes: Option<String>,
        /// Medical or treatment information (NONE, INFREQUENT_OR_MILD, FREQUENT_OR_INTENSE)
        #[arg(long)]
        medical_or_treatment_information: Option<String>,
        /// Profanity or crude humor (NONE, INFREQUENT_OR_MILD, FREQUENT_OR_INTENSE)
        #[arg(long)]
        profanity_or_crude_humor: Option<String>,
        /// Sexual content graphic and nudity (NONE, INFREQUENT_OR_MILD, FREQUENT_OR_INTENSE)
        #[arg(long)]
        sexual_content_graphic_and_nudity: Option<String>,
        /// Sexual content or nudity (NONE, INFREQUENT_OR_MILD, FREQUENT_OR_INTENSE)
        #[arg(long)]
        sexual_content_or_nudity: Option<String>,
        /// Violence cartoon or fantasy (NONE, INFREQUENT_OR_MILD, FREQUENT_OR_INTENSE)
        #[arg(long)]
        violence_cartoon_or_fantasy: Option<String>,
        /// Violence realistic (NONE, INFREQUENT_OR_MILD, FREQUENT_OR_INTENSE)
        #[arg(long)]
        violence_realistic: Option<String>,
        /// Violence realistic prolonged graphic or sadistic (NONE, INFREQUENT_OR_MILD, FREQUENT_OR_INTENSE)
        #[arg(long)]
        violence_realistic_prolonged_graphic_or_sadistic: Option<String>,
        /// Seventeen plus
        #[arg(long)]
        seventeen_plus: Option<bool>,
        /// Unresolved gambling apps
        #[arg(long)]
        unresolved_gambling_apps: Option<bool>,
        /// Kids age band
        #[arg(long)]
        kids_age_band: Option<String>,
    },
}

pub async fn execute(
    cmd: &AgeRatingDeclarationsCommands,
    client: &ApiClient,
) -> Result<(), CliError> {
    match cmd {
        AgeRatingDeclarationsCommands::Get { app_info_id } => {
            let doc = client
                .get::<AgeRatingDeclarationAttributes>(&format!(
                    "/appInfos/{app_info_id}/ageRatingDeclaration"
                ))
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        AgeRatingDeclarationsCommands::Update {
            id,
            alcohol_tobacco_or_drug_use_or_references,
            contests,
            gambling_and_contests,
            gambling,
            gambling_simulated,
            horror_or_fear_themes,
            mature_or_suggestive_themes,
            medical_or_treatment_information,
            profanity_or_crude_humor,
            sexual_content_graphic_and_nudity,
            sexual_content_or_nudity,
            violence_cartoon_or_fantasy,
            violence_realistic,
            violence_realistic_prolonged_graphic_or_sadistic,
            seventeen_plus,
            unresolved_gambling_apps,
            kids_age_band,
        } => {
            let mut attrs = serde_json::Map::new();
            if let Some(v) = alcohol_tobacco_or_drug_use_or_references {
                attrs.insert(
                    "alcoholTobaccoOrDrugUseOrReferences".into(),
                    serde_json::json!(v),
                );
            }
            if let Some(v) = contests {
                attrs.insert("contests".into(), serde_json::json!(v));
            }
            if let Some(v) = gambling_and_contests {
                attrs.insert("gamblingAndContests".into(), serde_json::json!(v));
            }
            if let Some(v) = gambling {
                attrs.insert("gambling".into(), serde_json::json!(v));
            }
            if let Some(v) = gambling_simulated {
                attrs.insert("gamblingSimulated".into(), serde_json::json!(v));
            }
            if let Some(v) = horror_or_fear_themes {
                attrs.insert("horrorOrFearThemes".into(), serde_json::json!(v));
            }
            if let Some(v) = mature_or_suggestive_themes {
                attrs.insert("matureOrSuggestiveThemes".into(), serde_json::json!(v));
            }
            if let Some(v) = medical_or_treatment_information {
                attrs.insert(
                    "medicalOrTreatmentInformation".into(),
                    serde_json::json!(v),
                );
            }
            if let Some(v) = profanity_or_crude_humor {
                attrs.insert("profanityOrCrudeHumor".into(), serde_json::json!(v));
            }
            if let Some(v) = sexual_content_graphic_and_nudity {
                attrs.insert(
                    "sexualContentGraphicAndNudity".into(),
                    serde_json::json!(v),
                );
            }
            if let Some(v) = sexual_content_or_nudity {
                attrs.insert("sexualContentOrNudity".into(), serde_json::json!(v));
            }
            if let Some(v) = violence_cartoon_or_fantasy {
                attrs.insert("violenceCartoonOrFantasy".into(), serde_json::json!(v));
            }
            if let Some(v) = violence_realistic {
                attrs.insert("violenceRealistic".into(), serde_json::json!(v));
            }
            if let Some(v) = violence_realistic_prolonged_graphic_or_sadistic {
                attrs.insert(
                    "violenceRealisticProlongedGraphicOrSadistic".into(),
                    serde_json::json!(v),
                );
            }
            if let Some(v) = seventeen_plus {
                attrs.insert("seventeenPlus".into(), serde_json::json!(v));
            }
            if let Some(v) = unresolved_gambling_apps {
                attrs.insert("unresolvedGamblingApps".into(), serde_json::json!(v));
            }
            if let Some(v) = kids_age_band {
                attrs.insert("kidsAgeBand".into(), serde_json::json!(v));
            }

            let body = serde_json::json!({
                "data": {
                    "type": "ageRatingDeclarations",
                    "id": id,
                    "attributes": attrs
                }
            });

            let doc = client
                .patch::<_, AgeRatingDeclarationAttributes>(
                    &format!("/ageRatingDeclarations/{id}"),
                    &body,
                )
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
    }
}
