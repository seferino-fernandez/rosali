# Rosali - a Kubernetes Desktop Client

**Rosali** is a **Kubernetes** desktop client that allows you to view and manage your clusters and their resources.

![License](https://img.shields.io/badge/license-MIT-green)

# Contents

- [Contents](#contents)
  - [Prerequisites](#prerequisites)
  - [Development Setup](#development-setup)
    - [Installation](#installation)
  - [Contribution](#contribution)
  - [License](#license)

## Prerequisites

This project uses Tauri with Vite. Make sure to have the necessary prerequisites installed as per the Tauri setup instructions. You can find the guide here: [Tauri Prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites)

## Development Setup

### 1. Clone the repository

Clone the repository and move into the directory.

```bash
git clone https://github.com/seferino-fernandez/rosali-app.git
cd rosali-app
```

### 2. Install dependencies

Install the dependencies by running any of the following commands:

 **npm**:
```bash
npm install
```
**pnpm**:
```bash
pnpm install
```

### 3. Start the development server

Start the development server by running any of the following commands

**npm**:
```bash
npm run tauri dev
```

**pnpm**:
```bash
pnpm run tauri dev
```

**Cargo**:
```bash
cargo tauri dev
```

## Contribution
Contributions to Rosali are always welcome. Whether it's a bug report, new feature, correction, or additional documentation, we greatly value your feedback and contributions.

## License
[MIT](./LICENSE.md)