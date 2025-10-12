# macOS Code Signing & Notarization Guide

This guide explains how to sign and notarize your AutoEQ app for distribution on macOS.

## Prerequisites

1. **Apple Developer Account** ($99/year) - [Sign up here](https://developer.apple.com/programs/)
2. **Xcode** installed (for certificate management)
3. **App-specific password** for notarization

## Step 1: Get Developer ID Certificate

### Option A: Via Xcode (Recommended)

1. Open **Xcode**
2. Go to **Xcode → Settings → Accounts**
3. Add your Apple ID if not already added
4. Select your account → Click **Manage Certificates**
5. Click **+** → Select **Developer ID Application**
6. Click **Done**

### Option B: Via Apple Developer Portal

1. Go to [Apple Developer Certificates](https://developer.apple.com/account/resources/certificates/list)
2. Click **+** to create a new certificate
3. Select **Developer ID Application**
4. Follow the instructions to create a Certificate Signing Request (CSR)
5. Upload CSR and download the certificate
6. Double-click to install in Keychain

### Verify Your Certificate

```bash
security find-identity -v -p codesigning
```

You should see something like:
```
"Developer ID Application: Your Name (TEAM_ID)"
```

## Step 2: Configure Tauri for Signing

Update `src-tauri/tauri.conf.json`:

```json
{
  "bundle": {
    "macOS": {
      "signingIdentity": "Developer ID Application: Your Name (TEAM_ID)",
      "entitlements": "./Entitlements.plist",
      "minimumSystemVersion": "10.15"
    }
  }
}
```

**Important**: Replace `"Your Name (TEAM_ID)"` with your actual certificate identity from Step 1.

## Step 3: Update Entitlements

Your app needs audio recording permissions. Update `src-tauri/Entitlements.plist`:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <!-- Required for sandboxed apps -->
    <key>com.apple.security.app-sandbox</key>
    <true/>
    
    <!-- Audio device access -->
    <key>com.apple.security.device.audio-input</key>
    <true/>
    
    <!-- Network access (for API calls) -->
    <key>com.apple.security.network.client</key>
    <true/>
    
    <!-- File access (for saving/loading files) -->
    <key>com.apple.security.files.user-selected.read-write</key>
    <true/>
</dict>
</plist>
```

## Step 4: Set Up Notarization

### Create App-Specific Password

1. Go to [appleid.apple.com](https://appleid.apple.com)
2. Sign in with your Apple ID
3. Go to **Security** → **App-Specific Passwords**
4. Click **Generate Password**
5. Name it "Tauri Notarization"
6. Save the generated password (you'll need it below)

### Store Credentials in Keychain

```bash
# Store your Apple ID credentials
xcrun notarytool store-credentials "autoeq-notarization" \
  --apple-id "your-apple-id@email.com" \
  --team-id "YOUR_TEAM_ID" \
  --password "your-app-specific-password"
```

Replace:
- `your-apple-id@email.com` - Your Apple ID email
- `YOUR_TEAM_ID` - Your 10-character Team ID (find it in [Apple Developer Account](https://developer.apple.com/account))
- `your-app-specific-password` - The password from above

### Configure Environment Variables

Add to your shell profile (`~/.zshrc` or `~/.bashrc`):

```bash
# Tauri Notarization
export APPLE_ID="your-apple-id@email.com"
export APPLE_TEAM_ID="YOUR_TEAM_ID"
export APPLE_SIGNING_IDENTITY="Developer ID Application: Your Name (TEAM_ID)"
export APPLE_CERTIFICATE="Developer ID Application: Your Name (TEAM_ID)"
```

Then reload:
```bash
source ~/.zshrc
```

## Step 5: Build and Sign

### Build with Signing

```bash
# Build for current architecture
npm run tauri build

# Build universal binary (Intel + Apple Silicon)
npm run tauri build -- --target universal-apple-darwin
```

Tauri will automatically:
1. Sign the app with your Developer ID
2. Create a DMG installer
3. Sign the DMG

### Manual Notarization (if needed)

If automatic notarization fails, you can manually notarize:

```bash
# Submit for notarization
xcrun notarytool submit \
  src-tauri/target/release/bundle/dmg/autoeq-app_*.dmg \
  --keychain-profile "autoeq-notarization" \
  --wait

# Check status
xcrun notarytool log <submission-id> \
  --keychain-profile "autoeq-notarization"

# Staple the notarization ticket (after approval)
xcrun stapler staple src-tauri/target/release/bundle/dmg/autoeq-app_*.dmg
```

## Step 6: Verify Signing

```bash
# Check code signature
codesign -dvv src-tauri/target/release/bundle/macos/autoeq-app.app

# Check notarization
spctl -a -vv src-tauri/target/release/bundle/macos/autoeq-app.app

# Check DMG signature
codesign -dvv src-tauri/target/release/bundle/dmg/autoeq-app_*.dmg
```

Expected output should show:
- `Authority=Developer ID Application: Your Name`
- `Status: accepted`
- `Notarization: accepted`

## Step 7: Distribution

Once signed and notarized, you can distribute:

1. **DMG file** - Users can download and install
2. **App bundle** - Can be zipped and distributed
3. **GitHub Releases** - Upload to your repository

### Test on Another Mac

1. Copy the DMG to another Mac
2. Open the DMG
3. Drag the app to Applications
4. Launch the app

If properly signed and notarized, it should open without security warnings.

## Troubleshooting

### "Developer cannot be verified" Error

- App is not notarized. Follow Step 4 and 5.
- User can temporarily bypass: Right-click → Open (first time only)

### "App is damaged" Error

- Code signature is broken
- Rebuild and ensure signing identity is correct
- Don't modify the app after signing

### Notarization Fails

```bash
# Check detailed logs
xcrun notarytool log <submission-id> \
  --keychain-profile "autoeq-notarization"
```

Common issues:
- Missing entitlements
- Unsigned binaries/libraries
- Invalid bundle structure

### Verify Entitlements

```bash
codesign -d --entitlements - src-tauri/target/release/bundle/macos/autoeq-app.app
```

## Automated Signing (CI/CD)

For GitHub Actions or other CI/CD:

1. Export your certificate:
   ```bash
   security find-identity -v -p codesigning
   security export -t identities -f pkcs12 -o certificate.p12
   ```

2. Add secrets to GitHub:
   - `APPLE_CERTIFICATE` - Base64 encoded .p12 file
   - `APPLE_CERTIFICATE_PASSWORD` - Certificate password
   - `APPLE_ID` - Your Apple ID
   - `APPLE_TEAM_ID` - Your Team ID
   - `APPLE_PASSWORD` - App-specific password

3. Use in workflow (see Tauri docs for full example)

## Resources

- [Tauri macOS Signing Guide](https://tauri.app/v1/guides/distribution/sign-macos)
- [Apple Code Signing Guide](https://developer.apple.com/library/archive/documentation/Security/Conceptual/CodeSigningGuide/)
- [Apple Notarization Guide](https://developer.apple.com/documentation/security/notarizing_macos_software_before_distribution)

## Quick Reference

```bash
# List certificates
security find-identity -v -p codesigning

# Build and sign
npm run tauri build

# Verify signature
codesign -dvv path/to/app

# Check notarization
spctl -a -vv path/to/app

# Submit for notarization
xcrun notarytool submit app.dmg --keychain-profile "autoeq-notarization" --wait

# Staple notarization
xcrun stapler staple app.dmg
```
