use crate::commands::*;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "apple-cli", about = "CLI for Apple App Store Connect API")]
pub struct Cli {
    /// Issuer ID (overrides config and env)
    #[arg(long, global = true, env = "APPLE_CLI_ISSUER_ID")]
    pub issuer_id: Option<String>,

    /// Key ID (overrides config and env)
    #[arg(long, global = true, env = "APPLE_CLI_KEY_ID")]
    pub key_id: Option<String>,

    /// Path to .p8 private key file (overrides config and env)
    #[arg(long, global = true, env = "APPLE_CLI_KEY_PATH")]
    pub key_path: Option<String>,

    /// Named profile to use (loads ~/.apple-cli/profiles/<name>.toml)
    #[arg(long, global = true, env = "APPLE_CLI_PROFILE")]
    pub profile: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Initialize project directory (.apple/config.toml + Claude Code skill)
    Init {
        /// App ID to link this project to (auto-detected if only one app)
        #[arg(long)]
        app_id: Option<String>,
    },
    /// Manage CLI configuration
    Config {
        #[command(subcommand)]
        command: config_cmd::ConfigCommands,
    },
    /// List and inspect apps
    Apps {
        #[command(subcommand)]
        command: apps::AppsCommands,
    },
    /// List and inspect builds
    Builds {
        #[command(subcommand)]
        command: builds::BuildsCommands,
    },
    /// Manage beta testers
    #[command(name = "beta-testers")]
    BetaTesters {
        #[command(subcommand)]
        command: beta_testers::BetaTestersCommands,
    },
    /// Manage beta groups
    #[command(name = "beta-groups")]
    BetaGroups {
        #[command(subcommand)]
        command: beta_groups::BetaGroupsCommands,
    },
    /// Manage bundle IDs
    #[command(name = "bundle-ids")]
    BundleIds {
        #[command(subcommand)]
        command: bundle_ids::BundleIdsCommands,
    },
    /// Manage certificates
    Certificates {
        #[command(subcommand)]
        command: certificates::CertificatesCommands,
    },
    /// Manage devices
    Devices {
        #[command(subcommand)]
        command: devices::DevicesCommands,
    },
    /// Manage provisioning profiles
    Profiles {
        #[command(subcommand)]
        command: profiles::ProfilesCommands,
    },
    /// Manage App Store versions
    Versions {
        #[command(subcommand)]
        command: versions::VersionsCommands,
    },
    /// Manage App Store version localizations
    #[command(name = "version-localizations")]
    VersionLocalizations {
        #[command(subcommand)]
        command: version_localizations::VersionLocalizationsCommands,
    },
    /// Manage users
    Users {
        #[command(subcommand)]
        command: users::UsersCommands,
    },
    /// Manage customer reviews
    #[command(name = "customer-reviews")]
    CustomerReviews {
        #[command(subcommand)]
        command: customer_reviews::CustomerReviewsCommands,
    },
    /// Manage in-app purchases
    #[command(name = "in-app-purchases")]
    InAppPurchases {
        #[command(subcommand)]
        command: in_app_purchases::InAppPurchasesCommands,
    },
    /// Manage subscription groups
    #[command(name = "subscription-groups")]
    SubscriptionGroups {
        #[command(subcommand)]
        command: subscription_groups::SubscriptionGroupsCommands,
    },
    /// Manage subscriptions
    Subscriptions {
        #[command(subcommand)]
        command: subscriptions::SubscriptionsCommands,
    },
    /// Manage subscription localizations
    #[command(name = "subscription-localizations")]
    SubscriptionLocalizations {
        #[command(subcommand)]
        command: subscription_localizations::SubscriptionLocalizationsCommands,
    },
    /// Manage review submissions to App Store Review
    #[command(name = "review-submissions")]
    ReviewSubmissions {
        #[command(subcommand)]
        command: review_submissions::ReviewSubmissionsCommands,
    },
    /// Manage phased releases for App Store versions
    #[command(name = "phased-releases")]
    PhasedReleases {
        #[command(subcommand)]
        command: phased_releases::PhasedReleasesCommands,
    },
    /// Manage user invitations
    #[command(name = "user-invitations")]
    UserInvitations {
        #[command(subcommand)]
        command: user_invitations::UserInvitationsCommands,
    },
    /// Manage sandbox testers (v2 endpoints)
    #[command(name = "sandbox-testers")]
    SandboxTesters {
        #[command(subcommand)]
        command: sandbox_testers::SandboxTestersCommands,
    },
    /// Manage bundle ID capabilities
    #[command(name = "bundle-id-capabilities")]
    BundleIdCapabilities {
        #[command(subcommand)]
        command: bundle_id_capabilities::BundleIdCapabilitiesCommands,
    },
    /// Manage beta app review submissions
    #[command(name = "beta-app-review-submissions")]
    BetaAppReviewSubmissions {
        #[command(subcommand)]
        command: beta_app_review_submissions::BetaAppReviewSubmissionsCommands,
    },
    /// Manage app encryption declarations
    #[command(name = "encryption-declarations")]
    EncryptionDeclarations {
        #[command(subcommand)]
        command: encryption_declarations::EncryptionDeclarationsCommands,
    },
    /// Manage App Store version release requests
    #[command(name = "release-requests")]
    ReleaseRequests {
        #[command(subcommand)]
        command: release_requests::ReleaseRequestsCommands,
    },
    /// List territories
    Territories {
        #[command(subcommand)]
        command: territories::TerritoriesCommands,
    },
    /// Manage App Store review details
    #[command(name = "review-details")]
    ReviewDetails {
        #[command(subcommand)]
        command: review_details::ReviewDetailsCommands,
    },
    /// Manage analytics reports
    #[command(name = "analytics")]
    Analytics {
        #[command(subcommand)]
        command: analytics_reports::AnalyticsReportsCommands,
    },
    /// Manage age rating declarations
    #[command(name = "age-rating-declarations")]
    AgeRatingDeclarations {
        #[command(subcommand)]
        command: age_rating_declarations::AgeRatingDeclarationsCommands,
    },
    /// Manage app info localizations
    #[command(name = "app-info-localizations")]
    AppInfoLocalizations {
        #[command(subcommand)]
        command: app_info_localizations::AppInfoLocalizationsCommands,
    },
    /// Download sales and finance reports
    #[command(name = "sales-reports")]
    SalesReports {
        #[command(subcommand)]
        command: sales_reports::SalesReportsCommands,
    },
    /// Performance and power metrics, diagnostics
    #[command(name = "perf-metrics")]
    PerfMetrics {
        #[command(subcommand)]
        command: perf_metrics::PerfMetricsCommands,
    },
}
