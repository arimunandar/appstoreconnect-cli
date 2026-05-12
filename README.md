# apple-cli

A full-featured CLI for the Apple App Store Connect API, built in Rust.

JSON-only output. Covers apps, builds, TestFlight (beta testers & groups), certificates, provisioning profiles, devices, bundle IDs, App Store versions, and users.

## Installation

### Build from source

Requires [Rust](https://rustup.rs/) 1.85+.

```sh
git clone <repo-url>
cd apple-developer-cli
cargo build --release
```

The binary will be at `target/release/apple-cli`. Move it somewhere on your `$PATH`:

```sh
cp target/release/apple-cli /usr/local/bin/
```

## Getting Started

### 1. Generate an API Key

1. Log into [App Store Connect](https://appstoreconnect.apple.com)
2. Go to **Users and Access** > **Integrations** > **Team Keys**
3. Click **+** to generate a new key
4. Choose a name and role (Admin for full access)
5. Click **Generate**
6. Note the **Issuer ID** (top of the page) and **Key ID** (in the key row)
7. **Download the .p8 file** immediately -- you can only download it once

### 2. Configure the CLI

```sh
apple-cli config init \
  --issuer-id YOUR_ISSUER_ID \
  --key-id YOUR_KEY_ID \
  --key-path /path/to/AuthKey_XXXXXX.p8
```

This saves credentials to `~/.apple-cli/config.toml`.

### 3. Verify it works

```sh
apple-cli apps list
```

## Authentication

Credentials are resolved in this order (first wins):

1. **CLI flags**: `--issuer-id`, `--key-id`, `--key-path`
2. **Environment variables**: `APPLE_CLI_ISSUER_ID`, `APPLE_CLI_KEY_ID`, `APPLE_CLI_KEY_PATH`
3. **Config file**: `~/.apple-cli/config.toml`

A fresh JWT (ES256) is generated per invocation. No tokens are cached.

## Output

- **stdout**: JSON data (pipe to `jq` for filtering)
- **stderr**: JSON errors with structured `error.type` and `error.message`
- **Exit code**: `0` on success, `1` on error

### Pipe examples

```sh
# Get just app names
apple-cli apps list | jq '.data[].attributes.name'

# Get build versions for an app
apple-cli builds list --filter-app APP_ID | jq '.data[].attributes.version'

# Get device UDIDs
apple-cli devices list | jq '.data[].attributes.udid'
```

## Pagination

All `list` commands support:

- `--limit N` -- number of results per page (default: 50, max: 200)
- `--all` -- automatically fetch all pages and return the combined result

```sh
# First 10 builds
apple-cli builds list --limit 10

# All builds (auto-paginate)
apple-cli builds list --all
```

---

## Commands

### config -- Manage CLI Configuration

```sh
# Save credentials
apple-cli config init \
  --issuer-id UUID \
  --key-id KEY_ID \
  --key-path /path/to/key.p8

# Show current config
apple-cli config show

# Update a single value
apple-cli config set issuer-id NEW_UUID
apple-cli config set key-id NEW_KEY_ID
apple-cli config set key-path /new/path/to/key.p8
```

---

### apps -- List and Inspect Apps

```sh
# List all apps
apple-cli apps list

# Filter by name
apple-cli apps list --filter-name "MyApp"

# Filter by bundle ID
apple-cli apps list --filter-bundle-id "com.example.myapp"

# Sort results
apple-cli apps list --sort name
apple-cli apps list --sort -name   # descending

# Get a single app by ID
apple-cli apps get APP_ID
```

---

### builds -- List and Inspect Builds

```sh
# List recent builds
apple-cli builds list

# Filter by app
apple-cli builds list --filter-app APP_ID

# Filter by version string
apple-cli builds list --filter-version 42

# Filter by processing state (PROCESSING, FAILED, INVALID, VALID)
apple-cli builds list --filter-processing-state VALID

# Combine filters
apple-cli builds list --filter-app APP_ID --filter-processing-state VALID --limit 5

# Get all builds (auto-paginate)
apple-cli builds list --filter-app APP_ID --all

# Get a single build
apple-cli builds get BUILD_ID
```

---

### beta-testers -- Manage TestFlight Testers

```sh
# List all beta testers
apple-cli beta-testers list

# Filter by email
apple-cli beta-testers list --filter-email user@example.com

# Filter by app
apple-cli beta-testers list --filter-app APP_ID

# Get all testers (auto-paginate)
apple-cli beta-testers list --all

# Get a single tester
apple-cli beta-testers get TESTER_ID

# Add a new tester to one or more beta groups
apple-cli beta-testers add \
  --email user@example.com \
  --first-name Jane \
  --last-name Doe \
  --beta-group-ids GROUP_ID_1,GROUP_ID_2

# Remove a tester entirely
apple-cli beta-testers remove TESTER_ID
```

#### TestFlight Workflow Example

```sh
# 1. Find your beta group ID
apple-cli beta-groups list | jq '.data[] | {id, name: .attributes.name}'

# 2. Add a tester to that group
apple-cli beta-testers add \
  --email tester@example.com \
  --first-name Test \
  --last-name User \
  --beta-group-ids 84c42bbf-fbfa-40d1-8745-11c7e6a588d4

# 3. Verify the tester was added
apple-cli beta-testers list --filter-email tester@example.com

# 4. Remove the tester when done
TESTER_ID=$(apple-cli beta-testers list --filter-email tester@example.com | jq -r '.data[0].id')
apple-cli beta-testers remove $TESTER_ID
```

---

### beta-groups -- Manage TestFlight Groups

```sh
# List all beta groups
apple-cli beta-groups list

# Filter by app
apple-cli beta-groups list --filter-app APP_ID

# Filter by name
apple-cli beta-groups list --filter-name "External Testers"

# Get a single group
apple-cli beta-groups get GROUP_ID

# Create a new beta group
apple-cli beta-groups create --app-id APP_ID --name "My Test Group"

# Update a beta group
apple-cli beta-groups update GROUP_ID --name "Renamed Group"
apple-cli beta-groups update GROUP_ID --public-link-enabled true
apple-cli beta-groups update GROUP_ID --public-link-limit 100

# Delete a beta group
apple-cli beta-groups delete GROUP_ID

# Add builds to a group (makes them available to testers)
apple-cli beta-groups add-builds GROUP_ID --build-ids BUILD_ID_1,BUILD_ID_2

# Add existing testers to a group
apple-cli beta-groups add-testers GROUP_ID --tester-ids TESTER_ID_1,TESTER_ID_2
```

#### Distribute a Build to TestFlight

```sh
# 1. Find the latest build
BUILD_ID=$(apple-cli builds list --filter-app APP_ID --limit 1 | jq -r '.data[0].id')

# 2. Find or create a beta group
GROUP_ID=$(apple-cli beta-groups list --filter-name "External Testers" | jq -r '.data[0].id')

# 3. Add the build to the group
apple-cli beta-groups add-builds $GROUP_ID --build-ids $BUILD_ID
```

---

### bundle-ids -- Manage Bundle IDs

```sh
# List all bundle IDs
apple-cli bundle-ids list

# Filter by identifier
apple-cli bundle-ids list --filter-identifier "com.example.*"

# Filter by platform (IOS, MAC_OS, UNIVERSAL)
apple-cli bundle-ids list --filter-platform IOS

# Get a single bundle ID
apple-cli bundle-ids get BUNDLE_ID_RESOURCE_ID

# Register a new bundle ID
apple-cli bundle-ids create \
  --identifier com.example.newapp \
  --name "My New App" \
  --platform IOS

# Delete a bundle ID
apple-cli bundle-ids delete BUNDLE_ID_RESOURCE_ID
```

---

### certificates -- Manage Signing Certificates

```sh
# List all certificates
apple-cli certificates list

# Filter by type (DEVELOPMENT, IOS_DISTRIBUTION, DEVELOPER_ID_APPLICATION, etc.)
apple-cli certificates list --filter-type IOS_DISTRIBUTION

# Get all certificates
apple-cli certificates list --all

# Get a single certificate
apple-cli certificates get CERT_ID

# Create a certificate from a CSR file
apple-cli certificates create \
  --type IOS_DISTRIBUTION \
  --csr-path /path/to/CertificateSigningRequest.certSigningRequest

# Revoke a certificate
apple-cli certificates delete CERT_ID
```

#### Certificate Types

| Type | Description |
|------|-------------|
| `IOS_DEVELOPMENT` | iOS app development |
| `IOS_DISTRIBUTION` | iOS App Store / Ad Hoc distribution |
| `DEVELOPMENT` | Apple Development (universal) |
| `DISTRIBUTION` | Apple Distribution (universal) |
| `DEVELOPER_ID_APPLICATION` | Developer ID Application (macOS) |
| `DEVELOPER_ID_INSTALLER` | Developer ID Installer (macOS) |

---

### devices -- Manage Registered Devices

```sh
# List all devices
apple-cli devices list

# Filter by name
apple-cli devices list --filter-name "iPhone"

# Filter by platform (IOS, MAC_OS)
apple-cli devices list --filter-platform IOS

# Filter by UDID
apple-cli devices list --filter-udid "00008120-XXXXXXXXXXXX"

# Get all devices (auto-paginate)
apple-cli devices list --all

# Get a single device
apple-cli devices get DEVICE_ID

# Register a new device
apple-cli devices register \
  --name "John's iPhone 16" \
  --platform IOS \
  --udid 00008140-XXXXXXXXXXXX

# Update a device (rename or change status)
apple-cli devices update DEVICE_ID --name "John's New Phone"
apple-cli devices update DEVICE_ID --status DISABLED
```

#### Finding a Device UDID

- **On device**: Settings > General > About > tap and hold the serial number area until UDID appears
- **In Xcode**: Window > Devices and Simulators > select device > Identifier
- **In Finder**: Connect device > click device name > click until UDID appears

---

### profiles -- Manage Provisioning Profiles

```sh
# List all profiles
apple-cli profiles list

# Filter by name
apple-cli profiles list --filter-name "MyApp Development"

# Filter by type (IOS_APP_DEVELOPMENT, IOS_APP_STORE, IOS_APP_ADHOC, etc.)
apple-cli profiles list --filter-type IOS_APP_STORE

# Get all profiles
apple-cli profiles list --all

# Get a single profile
apple-cli profiles get PROFILE_ID

# Create a development profile
apple-cli profiles create \
  --name "MyApp Development" \
  --type IOS_APP_DEVELOPMENT \
  --bundle-id BUNDLE_ID_RESOURCE_ID \
  --certificate-ids CERT_ID_1,CERT_ID_2 \
  --device-ids DEVICE_ID_1,DEVICE_ID_2

# Create a distribution profile (no device IDs needed)
apple-cli profiles create \
  --name "MyApp App Store" \
  --type IOS_APP_STORE \
  --bundle-id BUNDLE_ID_RESOURCE_ID \
  --certificate-ids CERT_ID

# Delete a profile
apple-cli profiles delete PROFILE_ID
```

#### Profile Types

| Type | Description |
|------|-------------|
| `IOS_APP_DEVELOPMENT` | Development (requires device IDs) |
| `IOS_APP_STORE` | App Store distribution |
| `IOS_APP_ADHOC` | Ad Hoc distribution (requires device IDs) |
| `IOS_APP_INHOUSE` | Enterprise in-house distribution |
| `MAC_APP_DEVELOPMENT` | macOS development |
| `MAC_APP_STORE` | Mac App Store distribution |
| `MAC_APP_DIRECT` | Developer ID distribution |

#### Full Provisioning Workflow

```sh
# 1. Register a bundle ID
apple-cli bundle-ids create \
  --identifier com.example.myapp \
  --name "My App" \
  --platform IOS

# 2. Find your certificate ID
CERT_ID=$(apple-cli certificates list --filter-type IOS_DISTRIBUTION | jq -r '.data[0].id')

# 3. Find your bundle ID resource ID
BUNDLE_ID=$(apple-cli bundle-ids list --filter-identifier com.example.myapp | jq -r '.data[0].id')

# 4. Create the profile
apple-cli profiles create \
  --name "My App Store Profile" \
  --type IOS_APP_STORE \
  --bundle-id $BUNDLE_ID \
  --certificate-ids $CERT_ID
```

---

### versions -- List and Inspect App Store Versions

```sh
# List versions (requires filter by app)
apple-cli versions list --filter-app APP_ID

# Filter by platform (IOS, MAC_OS, TV_OS, VISION_OS)
apple-cli versions list --filter-app APP_ID --filter-platform IOS

# Get all versions
apple-cli versions list --filter-app APP_ID --all

# Get a single version
apple-cli versions get VERSION_ID
```

---

### users -- Manage App Store Connect Users

```sh
# List all users
apple-cli users list

# Filter by username (email)
apple-cli users list --filter-username user@example.com

# Filter by role (ADMIN, DEVELOPER, MARKETING, SALES, etc.)
apple-cli users list --filter-roles ADMIN

# Get all users
apple-cli users list --all

# Get a single user
apple-cli users get USER_ID

# Update user roles
apple-cli users update USER_ID --roles DEVELOPER,MARKETING

# Remove a user
apple-cli users remove USER_ID
```

#### User Roles

| Role | Description |
|------|-------------|
| `ADMIN` | Full access to App Store Connect |
| `FINANCE` | Financial reports and agreements |
| `TECHNICAL` | Technical role (deprecated) |
| `ACCOUNT_HOLDER` | Account holder (read-only via API) |
| `SALES` | Sales and trends data |
| `MARKETING` | Marketing and metadata |
| `DEVELOPER` | Development and TestFlight |
| `APP_MANAGER` | App management |
| `CUSTOMER_SUPPORT` | Customer support and reviews |
| `CREATE_APPS` | Can create new apps |

---

## Environment Variables

| Variable | Description |
|----------|-------------|
| `APPLE_CLI_ISSUER_ID` | Issuer ID for JWT authentication |
| `APPLE_CLI_KEY_ID` | API Key ID |
| `APPLE_CLI_KEY_PATH` | Path to the .p8 private key file |

Use environment variables for CI/CD pipelines or to avoid passing flags on every call:

```sh
export APPLE_CLI_ISSUER_ID="d2bc1deb-c23f-40a7-b653-17233d4bcdd4"
export APPLE_CLI_KEY_ID="SNG22UXS9T"
export APPLE_CLI_KEY_PATH="/path/to/AuthKey.p8"

apple-cli apps list
```

---

## Scripting Examples

### List all apps with their bundle IDs

```sh
apple-cli apps list | jq '.data[] | {name: .attributes.name, bundleId: .attributes.bundleId}'
```

### Get the latest build number for an app

```sh
apple-cli builds list --filter-app APP_ID --limit 1 | jq -r '.data[0].attributes.version'
```

### Export all device UDIDs to a CSV

```sh
echo "name,udid,platform,model"
apple-cli devices list --all | jq -r '.data[] | [.attributes.name, .attributes.udid, .attributes.platform, .attributes.model] | @csv'
```

### List all expired certificates

```sh
apple-cli certificates list --all | jq '.data[] | select(.attributes.expirationDate < now | todate) | {name: .attributes.name, expires: .attributes.expirationDate}'
```

### Bulk add testers to a beta group

```sh
GROUP_ID="your-group-id"
for email in alice@example.com bob@example.com carol@example.com; do
  apple-cli beta-testers add \
    --email "$email" \
    --first-name "${email%%@*}" \
    --last-name "Tester" \
    --beta-group-ids "$GROUP_ID"
done
```

### Find a tester by email and remove them

```sh
TESTER_ID=$(apple-cli beta-testers list --filter-email user@example.com | jq -r '.data[0].id')
apple-cli beta-testers remove "$TESTER_ID"
```

### Check which profiles are expired or invalid

```sh
apple-cli profiles list --all | jq '.data[] | select(.attributes.profileState != "ACTIVE") | {name: .attributes.name, state: .attributes.profileState}'
```

---

## Error Handling

Errors are returned as JSON on stderr:

```json
{
  "error": {
    "type": "config_error",
    "message": "Configuration error: issuer-id not set (use --issuer-id, env APPLE_CLI_ISSUER_ID, or `apple-cli config init`)"
  }
}
```

| Error Type | Cause |
|------------|-------|
| `config_error` | Missing or invalid configuration |
| `auth_error` | Cannot read key file or sign JWT |
| `api_error` | App Store Connect API returned an error |
| `http_error` | Network/connection failure |
| `json_error` | Unexpected API response format |
| `io_error` | File system error |

### Common API Errors

| Status | Meaning | Fix |
|--------|---------|-----|
| 401 | Unauthorized | Check issuer-id, key-id, and key-path |
| 403 | Forbidden | API key lacks the required role |
| 404 | Not Found | Resource ID doesn't exist |
| 409 | Conflict | Resource already exists or state conflict |
| 429 | Rate Limited | Wait and retry (Apple allows ~3600 req/hour) |

---

## Tips

- Use `jq` to filter and format JSON output
- Use `--limit 1` to quickly get the most recent resource
- Use `--all` sparingly on large datasets (e.g., 500+ builds) as it makes multiple API calls
- Resource IDs from App Store Connect are stable -- save them for scripts
- The `--filter-*` flags map directly to App Store Connect API query parameters
