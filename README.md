# MailHub

MailHub is a modern, one-stop mailbox management system built with Tauri, React, TypeScript, and TailwindCSS.

## Features

- **Multi-Account Support**: Connect unlimited email accounts via IMAP, POP3, or OAuth2 (Gmail/Outlook)
- **Unified Inbox**: View all your emails from different accounts in one place
- **AI-Powered Classification**: Automatically categorize emails as marketing, important, verification, or normal
- **Smart Notifications**: Get notified only for important emails and verification codes
- **Verification Code Extraction**: Automatically detect and display verification codes
- **Modern UI**: Clean and beautiful interface built with RadixUI and TailwindCSS
- **Cross-Platform**: Available for Windows, macOS, and Linux

## Technology Stack

### Frontend
- React 19 + TypeScript
- TailwindCSS 4 for styling
- RadixUI for accessible components
- Vite for building

### Backend
- Rust with Tauri 2
- JSON-based storage
- AI integration support (OpenAI, Anthropic, Gemini)

## Development

### Prerequisites

- Node.js 24+
- Rust (latest stable)
- Platform-specific dependencies:
  - **Linux**: `libwebkit2gtk-4.1-dev`, `libgtk-3-dev`, `libayatana-appindicator3-dev`, `librsvg2-dev`, `patchelf`
  - **macOS**: Xcode Command Line Tools
  - **Windows**: WebView2

### Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/MournInk/MailHub.git
   cd MailHub
   ```

2. Install dependencies:
   ```bash
   npm install
   ```

3. Run in development mode:
   ```bash
   npm run tauri dev
   ```

4. Build for production:
   ```bash
   npm run tauri build
   ```

## Configuration

### Adding Email Accounts

1. Click "Add Account" in the sidebar
2. Select your protocol (IMAP, POP3, or OAuth2)
3. Enter your email credentials
4. Optionally set a display name and tags

### AI Configuration

1. Open Settings
2. Navigate to "AI Configuration"
3. Enable AI features
4. Select your provider (OpenAI, Anthropic, or Gemini)
5. Enter your API key
6. Optionally enable auto-delete for marketing emails

## Building

The project uses GitHub Actions to automatically build for all platforms:

- **Windows**: `.exe` installer and `.msi`
- **macOS**: Universal `.dmg` for both Intel and Apple Silicon
- **Linux**: `.AppImage`, `.deb`, and `.rpm`

## License

See LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
