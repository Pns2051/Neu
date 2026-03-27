# Publishing Guide - Neu Music Platform

This guide outlines how to publish Neu to Flathub and the Snap Store, and how to automate updates.

## 📦 1. Flathub (Flatpak)

### Initial Submission
1. **Fork the Flathub repo**: Visit [github.com/flathub/flathub](https://github.com/flathub/flathub) and fork it.
2. **Submit a Pull Request**: Add your manifest `org.neu.Neu.yml` and metadata. Detailed steps at [docs.flathub.org](https://docs.flathub.org/docs/for-app-authors/submission/).

### Automation
- Once accepted, Flathub will create a repo for your app (e.g., `flathub/org.neu.Neu`).
- Use the **Flathub GitHub App** or a webhook to trigger rebuilds whenever you push to your main repository.

---

## 🏗 2. Snap Store (Snapcraft)

### Registration
1. **Register the name**: Run `snapcraft register neu-music`.
2. **Login**: Run `snapcraft login`.

### Automation
- Neu is pre-configured with `snapcraft.yaml`.
- Use the **Snapcraft GitHub Action** (see `.github/workflows/release.yml`) to automatically push new builds to the `stable` channel.

---

## 🚀 3. Automated Update Flow (Antigravity One-Button)

To update all platforms (DEB, RPM, AppImage, Snap, Flatpak) simultaneously:

1. **Run push_update.sh**: This script bumps the version locally and creates a git tag.
2. **Push Tag to GitHub**: `git push origin v1.0.1`.
3. **GitHub Actions Trigger**: Our CI/CD pipeline starts automatically:
   - **GitHub Releases**: Uploads AppImage, DEB, and RPM.
   - **Snap Store**: Pushes the new snap to `stable`.
   - **Flathub**: Notifies Flathub to rebuild and publish.
   - **Update Checker**: The app's built-in `UpdateChecker` sees the new tag and notifies users.

---
**Neu is now a global platform—distributed everywhere, managed from one place.**
