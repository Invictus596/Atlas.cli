# 🚀 Atlas Project Architecture

This diagram represents the end-to-end flow of the Atlas ecosystem, from stakeholder registration to AI-driven automated notifications.

```mermaid
graph TD
    %% Global Styling
    classDef web fill:#f9f,stroke:#333,stroke-width:2px;
    classDef db fill:#00f,stroke:#fff,stroke-width:2px,color:#fff;
    classDef cli fill:#00ff00,stroke:#333,stroke-width:2px;
    classDef ai fill:#ff9900,stroke:#333,stroke-width:2px;
    classDef comms fill:#ff0000,stroke:#fff,stroke-width:2px,color:#fff;

    %% Onboarding Layer
    subgraph "Onboarding (Next.js + Supabase)"
        A[CEO / Manager] -->|Enter Contact Info| B(Next.js Web Portal)
        B -->|Write Data| C[(Supabase Database)]
        C -->|Return| D{Atlas API Key}
    end

    %% Developer Layer
    subgraph "Development (Rust CLI + TUI)"
        E[Developer] -->|atlas login KEY| F[Rust CLI Engine]
        F <-->|Handshake & Auth| C
        F -->|Local Cache| G[.atlas.toml]
        
        E -->|atlas scout| F
        F -->|Scan Local Files| H[File Aggregator]
    end

    %% Intelligence Layer
    subgraph "Intelligence (Gemini 1.5 Pro)"
        H -->|Context Payload| I[Gemini AI]
        I -->|Arch Analysis| J[Technical Report]
        I -->|Impact Summary| K[Executive Brief]
        J -->|Render| L[TUI Dashboard]
    end

    %% Communication Layer
    subgraph "Notification (Twilio API)"
        M[atlas ship] -->|Trigger| F
        F -->|Fetch Secrets| N[.env File]
        F -->|Load Brief| K
        F -->|POST Request| O[Twilio API]
        
        O -->|Voice Call| P[CEO Mobile]
        O -->|WhatsApp| Q[Client Message]
    end

    %% Apply Styles
    class B web;
    class C db;
    class F,L cli;
    class I ai;
    class O,P,Q comms;
