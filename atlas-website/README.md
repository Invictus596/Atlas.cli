<div align="center">

# 🗺️ Atlas.cli

### The AI-Powered Developer Onboarding & Release Orchestration Suite

> **One CLI. Two superpowers.** Onboard new developers in seconds. Ship releases to stakeholders in one command.

[![Next.js](https://img.shields.io/badge/Next.js-16-black?style=for-the-badge&logo=next.js)](https://nextjs.org/)
[![TypeScript](https://img.shields.io/badge/TypeScript-5-blue?style=for-the-badge&logo=typescript)](https://www.typescriptlang.org/)
[![Tailwind CSS](https://img.shields.io/badge/Tailwind-v4-38bdf8?style=for-the-badge&logo=tailwind-css)](https://tailwindcss.com/)
[![Gemini](https://img.shields.io/badge/Gemini-API-8e75b0?style=for-the-badge&logo=google-gemini)](https://ai.google.dev/)
[![License](https://img.shields.io/badge/License-MIT-green?style=for-the-badge)](LICENSE)

⚡ Built for the Hackathon · Enterprise-Grade AI Workflow Automation

</div>

---

## 🎯 The Vision

Every enterprise bleeds time in two places: **onboarding new developers** to unfamiliar codebases and **coordinating release updates** across fragmented stakeholder channels. Atlas.cli solves both — permanently.

By harnessing the power of **Google Gemini AI**, Atlas.cli transforms messy repository structures into beautifully documented onboarding guides, and turns raw git diffs into polished, multi-channel release briefings. What used to take hours of manual documentation and communication now happens in **a single command**.

---

## 🚀 Core Features

### 🗺️ `atlas map` — The Onboarding Engine

> *"Never spend day one reading source code again."*

When a developer runs `atlas map`, the magic unfolds instantly:

1. **Directory Scan** — The Node.js script recursively reads the entire project tree, analyzing file structure, dependencies, and key entry points.
2. **AI-Powered Analysis** — The directory tree and metadata are piped to the **Google Gemini API**, which understands the architecture, patterns, and relationships within the codebase.
3. **Instant Documentation** — Gemini generates a comprehensive, beautifully formatted `ONBOARDING.md` file complete with:
   - 📐 **Mermaid.js Architecture Diagrams** — Visual system architecture rendered from code analysis.
   - 📂 **Module Breakdown** — Clear explanations of each subsystem.
   - 🔗 **Dependency Maps** — How services, APIs, and components interconnect.
   - 🏗️ **Conventions & Patterns** — Coding standards and architectural decisions discovered in the codebase.

```bash
atlas map
# → ONBOARDING.md generated in 12 seconds ⚡
```

---

### ⚡ `atlas ship` — The Release Orchestrator

> *"Ship features, not Slack messages."*

When a release lead runs `atlas ship --sprint 4`, the entire stakeholder communication pipeline fires automatically:

1. **Git Diff Capture** — Grabs all changes since the last sprint tag, analyzing commits, file changes, and commit messages.
2. **AI-Powered Summaries** — Gemini processes the diff and generates three distinct stakeholder reports:
   - 🔧 **Tech Summary** — Deep-dive into code changes, refactors, and architectural decisions for engineering teams.
   - 🧪 **QA Summary** — Risk assessment, test impact analysis, and areas requiring regression testing for QA teams.
   - 💼 **Business Summary** — Plain-language feature descriptions and business impact for non-technical stakeholders.
3. **Documentation Updates** — Auto-updates release notes, changelogs, and sprint documentation.
4. **Stakeholder Notifications** — Triggers **Twilio voice calls** and **WhatsApp messages** to designated stakeholders, ensuring zero communication gaps.

```bash
atlas ship --sprint 4
# → Tech, QA & Business summaries written. Stakeholders notified. ✅
```

---

## 🛠️ Tech Stack

### Frontend — Atlas Web Dashboard

| Technology       | Purpose                           |
|------------------|-----------------------------------|
| **Next.js 16**   | App Router framework for the web dashboard |
| **Tailwind CSS v4** | Utility-first styling with the latest engine |
| **Shadcn/UI**    | Accessible, composable UI components |
| **Lucide Icons** | Clean, consistent iconography |
| **JetBrains Mono** | Developer-grade typography |

### Backend & CLI

| Technology         | Purpose                                    |
|--------------------|--------------------------------------------|
| **Node.js**        | CLI runtime & directory scanning engine    |
| **Google Gemini API** | AI-powered code analysis & content generation |
| **Twilio**         | Voice call & SMS notifications for stakeholders |
| **Mermaid.js**     | Auto-generated architecture diagrams in Markdown |

---

## 📦 Getting Started

### Prerequisites

- **Node.js** `20+`
- **npm** or your preferred package manager
- A **Google AI Studio API Key** (for Gemini API access)
- A **Twilio Account SID & Auth Token** (for `atlas ship` notifications)

### Installation

```bash
# 1. Clone the repository
git clone https://github.com/your-org/atlas.cli.git
cd atlas.cli/atlas-web

# 2. Install dependencies
npm install

# 3. Set up environment variables
cp .env.example .env.local
# Edit .env.local with your API keys
```

### Run the Frontend Dashboard

```bash
npm run dev
```

Open [http://localhost:3000](http://localhost:3000) in your browser to access the Atlas web dashboard.

The page auto-updates as you edit files in `src/`.

### Available Scripts

| Command        | Description                       |
|----------------|-----------------------------------|
| `npm run dev`  | Start the development server      |
| `npm run build`| Build for production              |
| `npm run start`| Start the production server       |
| `npm run lint` | Run ESLint for code quality       |

---

## 📂 Project Structure

```
atlas-web/
├── src/
│   ├── app/            # Next.js App Router pages
│   │   ├── dashboard/  # Main dashboard route
│   │   ├── setup/      # Initial setup wizard
│   │   └── page.tsx    # Landing page
│   ├── components/     # Reusable UI components
│   │   └── ui/         # Shadcn/UI primitives
│   └── lib/            # Shared utilities
├── public/             # Static assets
└── package.json
```

---

## 🔮 Roadmap

- [ ] **`atlas sync`** — AI-powered code review bot that comments on PRs
- [ ] **Slack / Teams Integration** — Multi-channel stakeholder notifications
- [ ] **Self-Hosted LLM Support** — Run Gemini or Llama locally for air-gapped enterprises
- [ ] **Onboarding Quizzes** — Auto-generated knowledge checks from `ONBOARDING.md`
- [ ] **Release Analytics Dashboard** — Track sprint velocity and stakeholder engagement

---

## 🏆 Hackathon

Built in a sprint. Designed for production. Atlas.cli proves that developer experience **is** the product.

---

<div align="center">

**Atlas.cli** — *Map the code. Ship with confidence.*

Made with ⚡ and too much coffee.

</div>
