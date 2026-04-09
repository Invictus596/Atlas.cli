"use client";

import { useState } from "react";
import {
  CheckCircle2,
  Copy,
  Check,
  GitBranch,
  Terminal,
} from "lucide-react";

const repos = [
  { name: "core-banking-api", status: "Active" },
  { name: "frontend-monorepo", status: "Active" },
  { name: "auth-service", status: "Active" },
];

function CodeBlock({ code }: { code: string }) {
  const [copied, setCopied] = useState(false);

  const handleCopy = async () => {
    await navigator.clipboard.writeText(code);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="rounded-xl overflow-hidden border border-slate-800 bg-slate-950">
      <div className="flex items-center justify-between px-4 py-2.5 border-b border-slate-800/60">
        <div className="flex items-center gap-2 text-xs text-slate-500 font-mono">
          <Terminal className="w-3.5 h-3.5" />
          Quick Start
        </div>
        <button
          onClick={handleCopy}
          className="flex items-center gap-1.5 px-2.5 py-1.5 rounded-md bg-slate-800 text-slate-400 text-xs hover:bg-slate-700 hover:text-slate-300 transition-colors"
        >
          {copied ? (
            <>
              <Check className="w-3 h-3 text-emerald-500" />
              Copied
            </>
          ) : (
            <>
              <Copy className="w-3 h-3" />
              Copy
            </>
          )}
        </button>
      </div>
      <div className="p-5">
        <code className="text-sm font-mono text-sky-400">{code}</code>
      </div>
    </div>
  );
}

export default function DashboardPage() {
  return (
    <div className="relative min-h-screen px-6 py-12">
      {/* Background glow */}
      <div className="absolute top-0 left-1/2 -translate-x-1/2 w-[600px] h-[400px] bg-sky-500/5 rounded-full blur-[120px] pointer-events-none" />

      <div className="relative z-10 max-w-5xl mx-auto space-y-10">
        {/* Header */}
        <div className="flex items-center gap-3">
          <div className="flex items-center gap-2 px-4 py-2.5 rounded-xl bg-emerald-500/10 border border-emerald-500/20">
            <CheckCircle2 className="w-5 h-5 text-emerald-500" />
            <span className="text-sm font-medium text-emerald-400">
              Atlas Enterprise Status: Active
            </span>
          </div>
        </div>

        {/* Dashboard grid */}
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          {/* Linked Repositories */}
          <div className="rounded-2xl border border-slate-800 bg-slate-900/50 p-6">
            <div className="flex items-center gap-3 mb-6">
              <div className="flex items-center justify-center w-10 h-10 rounded-lg bg-sky-500/10 border border-sky-500/20 text-sky-400">
                <GitBranch className="w-5 h-5" />
              </div>
              <div>
                <h2 className="text-lg font-semibold text-white">
                  Linked Repositories
                </h2>
                <p className="text-xs text-slate-500">
                  {repos.length} repos connected
                </p>
              </div>
            </div>

            <ul className="space-y-3">
              {repos.map((repo) => (
                <li
                  key={repo.name}
                  className="flex items-center justify-between px-4 py-3 rounded-xl bg-slate-950/60 border border-slate-800/60"
                >
                  <span className="font-mono text-sm text-slate-300">
                    {repo.name}
                  </span>
                  <span className="flex items-center gap-1.5 px-2.5 py-1 rounded-md bg-emerald-500/10 text-emerald-400 text-xs font-medium border border-emerald-500/20">
                    <CheckCircle2 className="w-3 h-3" />
                    {repo.status}
                  </span>
                </li>
              ))}
            </ul>
          </div>

          {/* Quick Start */}
          <div className="relative rounded-2xl border border-sky-500/20 bg-slate-900/50 p-6 shadow-lg shadow-sky-500/5">
            <div className="absolute inset-0 rounded-2xl bg-gradient-to-br from-sky-500/[0.03] to-transparent pointer-events-none" />
            <div className="relative">
              <div className="flex items-center gap-3 mb-6">
                <div className="flex items-center justify-center w-10 h-10 rounded-lg bg-sky-500/10 border border-sky-500/20 text-sky-400">
                  <Terminal className="w-5 h-5" />
                </div>
                <div>
                  <h2 className="text-lg font-semibold text-white">
                    Quick Start
                  </h2>
                  <p className="text-xs text-slate-500">
                    Your next step as a developer
                  </p>
                </div>
              </div>

              <CodeBlock code="atlas login [YOUR_KEY]" />
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
