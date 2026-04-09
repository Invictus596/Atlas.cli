"use client";

import Link from "next/link";
import { Terminal, Map, Rocket, ArrowRight, Sparkles } from "lucide-react";
import { useEffect, useState } from "react";
import { motion } from "framer-motion";

function TypingTerminal() {
  const [displayedLines, setDisplayedLines] = useState("");
  const [currentLineIndex, setCurrentLineIndex] = useState(0);
  const [charIndex, setCharIndex] = useState(0);

  const terminalLines = [
    { text: "$ atlas map", delay: 80 },
    { text: "", delay: 200 },
    { text: "[SCANNING] Mapping local directory tree...", delay: 60 },
    { text: "  ├── src/", delay: 100 },
    { text: "  │   ├── app/", delay: 100 },
    { text: "  │   ├── components/", delay: 100 },
    { text: "  │   └── lib/", delay: 100 },
    { text: "  ├── package.json", delay: 100 },
    { text: "  └── tsconfig.json", delay: 100 },
    { text: "", delay: 200 },
    { text: "✓ Dependency graph resolved (14 nodes)", delay: 60 },
    { text: "✓ Service topology generated", delay: 60 },
    { text: "✓ Mermaid chart rendered → /docs/architecture.md", delay: 60 },
    { text: "", delay: 100 },
    { text: "Onboarding map complete. Ready for new devs.", delay: 80 },
  ];

  useEffect(() => {
    if (currentLineIndex >= terminalLines.length) return;

    const currentLine = terminalLines[currentLineIndex];

    if (charIndex < currentLine.text.length) {
      const timeout = setTimeout(() => {
        setDisplayedLines(
          (prev) => prev + currentLine.text[charIndex]
        );
        setCharIndex((prev) => prev + 1);
      }, currentLine.delay);

      return () => clearTimeout(timeout);
    } else {
      const timeout = setTimeout(() => {
        setDisplayedLines((prev) => prev + "\n");
        setCurrentLineIndex((prev) => prev + 1);
        setCharIndex(0);
      }, 300);

      return () => clearTimeout(timeout);
    }
  }, [charIndex, currentLineIndex, terminalLines]);

  return (
    <motion.div
      className="w-full max-w-2xl mx-auto mt-16 rounded-xl overflow-hidden border border-slate-800/80 shadow-2xl shadow-sky-500/5"
      initial={{ opacity: 0, scale: 0.95 }}
      animate={{ opacity: 1, scale: 1 }}
      transition={{ duration: 0.8, ease: [0.25, 0.46, 0.45, 0.94], delay: 0.6 }}
    >
      <motion.div
        animate={{ y: [0, -10, 0] }}
        transition={{
          duration: 4,
          ease: "easeInOut",
          repeat: Infinity,
          repeatType: "mirror",
        }}
      >
        {/* macOS-style header */}
        <div className="bg-slate-900/95 px-4 py-3 flex items-center gap-2 border-b border-slate-800">
          <div className="w-3 h-3 rounded-full bg-[#ff5f57]" />
          <div className="w-3 h-3 rounded-full bg-[#febc2e]" />
          <div className="w-3 h-3 rounded-full bg-[#28c840]" />
          <span className="ml-3 text-xs text-slate-500 font-mono">
            atlas terminal
          </span>
        </div>
        {/* Terminal content */}
        <div className="bg-slate-950/90 p-6 font-mono text-sm min-h-[280px]">
          <pre className="text-slate-300 whitespace-pre-wrap break-words">
            {displayedLines}
            <span className="inline-block w-2 h-4 bg-sky-400/80 animate-pulse ml-0.5 align-middle" />
          </pre>
        </div>
      </motion.div>
    </motion.div>
  );
}

function HowItWorksCard({
  icon: Icon,
  badge,
  title,
  description,
  features,
}: {
  icon: React.ElementType;
  badge: string;
  title: string;
  description: string;
  features: string[];
}) {
  return (
    <motion.div
      className="group relative rounded-2xl border border-slate-800 bg-slate-900/50 p-8 transition-all duration-300 hover:border-sky-500/40 hover:shadow-lg hover:shadow-sky-500/5 hover:-translate-y-1"
      initial={{ opacity: 0, y: 40 }}
      whileInView={{ opacity: 1, y: 0 }}
      viewport={{ once: true, margin: "-100px" }}
      transition={{ duration: 0.6, ease: [0.25, 0.46, 0.45, 0.94] }}
    >
      <div className="flex items-start justify-between mb-6">
        <div className="flex items-center gap-3">
          <div className="flex items-center justify-center w-12 h-12 rounded-xl bg-sky-500/10 border border-sky-500/20 text-sky-400">
            <Icon className="w-6 h-6" />
          </div>
          <span className="text-xs font-mono px-2 py-1 rounded-full bg-sky-500/10 text-sky-400 border border-sky-500/20">
            {badge}
          </span>
        </div>
        <ArrowRight className="w-5 h-5 text-slate-600 group-hover:text-sky-400 transition-colors" />
      </div>

      <h3 className="text-xl font-semibold text-white mb-3">{title}</h3>
      <p className="text-slate-400 text-sm leading-relaxed mb-6">
        {description}
      </p>

      <ul className="space-y-2">
        {features.map((feature, i) => (
          <li key={i} className="flex items-center gap-2 text-sm text-slate-500">
            <Sparkles className="w-3.5 h-3.5 text-sky-500/60 shrink-0" />
            {feature}
          </li>
        ))}
      </ul>
    </motion.div>
  );
}

export default function LandingPage() {
  return (
    <motion.div className="relative min-h-screen overflow-hidden">
      {/* Background glow effects */}
      <motion.div
        className="absolute top-0 left-1/2 -translate-x-1/2 w-[800px] h-[600px] bg-sky-500/8 rounded-full blur-[120px] pointer-events-none"
        animate={{ opacity: [0.5, 0.8, 0.5] }}
        transition={{ duration: 6, ease: "easeInOut", repeat: Infinity, repeatType: "mirror" }}
      />
      <motion.div
        className="absolute top-1/3 right-0 w-[400px] h-[400px] bg-emerald-500/5 rounded-full blur-[100px] pointer-events-none"
        animate={{ opacity: [0.3, 0.6, 0.3] }}
        transition={{ duration: 8, ease: "easeInOut", repeat: Infinity, repeatType: "mirror" }}
      />

      {/* Hero Section */}
      <section className="relative z-10 pt-32 pb-20 px-6">
        <div className="max-w-4xl mx-auto text-center">
          <motion.h1
            className="text-5xl sm:text-6xl md:text-7xl font-bold tracking-tight text-white mb-6"
            initial={{ opacity: 0, y: 30 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.8, ease: [0.25, 0.46, 0.45, 0.94] }}
          >
            The 2-in-1 Suite:{" "}
            <span className="text-sky-400 drop-shadow-[0_0_30px_rgba(56,189,248,0.3)]">
              Atlas.cli
            </span>
          </motion.h1>

          <motion.p
            className="text-lg sm:text-xl text-slate-400 max-w-2xl mx-auto mb-10 leading-relaxed"
            initial={{ opacity: 0, y: 30 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.8, ease: [0.25, 0.46, 0.45, 0.94], delay: 0.2 }}
          >
            Streamline developer onboarding and automate your release
            pipeline&nbsp;&mdash;&nbsp;all from a single CLI. Ship faster,
            onboard instantly.
          </motion.p>

          <motion.div
            initial={{ opacity: 0, y: 30 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.8, ease: [0.25, 0.46, 0.45, 0.94], delay: 0.4 }}
          >
            <Link
              href="/setup"
              className="inline-flex items-center gap-2 px-8 py-4 rounded-xl bg-sky-500 text-slate-950 font-semibold text-base transition-all duration-200 hover:bg-sky-400 hover:shadow-[0_0_30px_rgba(56,189,248,0.4)] hover:-translate-y-0.5"
            >
              Start Enterprise Setup
              <ArrowRight className="w-5 h-5" />
            </Link>
          </motion.div>
        </div>

        {/* Terminal mockup */}
        <TypingTerminal />

        {/* Teammate video */}
        <motion.div
          className="mt-16 max-w-[1000px] mx-auto rounded-2xl overflow-hidden border border-slate-800 shadow-lg shadow-sky-500/10"
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.8, ease: [0.25, 0.46, 0.45, 0.94], delay: 0.6 }}
        >
          <video
            src="/hero-video.mp4"
            loop
            autoPlay
            muted
            playsInline
            className="w-full h-auto"
          />
        </motion.div>
      </section>

      {/* How it works section */}
      <section className="relative z-10 py-24 px-6">
        <div className="max-w-5xl mx-auto">
          <div className="text-center mb-16">
            <div className="inline-flex items-center gap-2 px-4 py-2 rounded-full bg-slate-800/50 border border-slate-700 text-slate-400 text-sm mb-4">
              <Terminal className="w-4 h-4" />
              Two commands. Zero friction.
            </div>
            <h2 className="text-3xl sm:text-4xl font-bold text-white">
              How it works
            </h2>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
            <HowItWorksCard
              icon={Map}
              badge="atlas map"
              title="The Onboarding Engine"
              description="Instantly generate architectural Mermaid.js charts from your codebase. New developers understand your entire system in minutes, not weeks."
              features={[
                "Auto-generated service topology diagrams",
                "Dependency graph visualization via Mermaid.js",
                "Real-time architecture documentation sync",
              ]}
            />
            <HowItWorksCard
              icon={Rocket}
              badge="atlas ship"
              title="The Release Orchestrator"
              description="Generate comprehensive tech docs, QA summaries, and automated WhatsApp client updates with a single command."
              features={[
                "Auto-generated release notes & tech docs",
                "AI-powered QA test summaries",
                "WhatsApp & Slack client notifications",
              ]}
            />
          </div>
        </div>
      </section>

      {/* Footer spacer */}
      <section className="h-32" />
    </motion.div>
  );
}
