"use client";

import { useState } from "react";
import Link from "next/link";
import { Loader2, Copy, Check, ArrowLeft, KeyRound } from "lucide-react";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";

function generateMockKey(): string {
  const chars = "0123456789abcdef";
  const random = Array.from({ length: 28 }, () =>
    chars[Math.floor(Math.random() * chars.length)]
  ).join("");
  return `ATLAS_live_${random}`;
}

export default function SetupPage() {
  const [email, setEmail] = useState("");
  const [phone, setPhone] = useState("");
  const [whatsapp, setWhatsapp] = useState("");
  const [loading, setLoading] = useState(false);
  const [generated, setGenerated] = useState(false);
  const [apiKey, setApiKey] = useState("");
  const [copied, setCopied] = useState(false);

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);

    setTimeout(() => {
      setApiKey(generateMockKey());
      setLoading(false);
      setGenerated(true);
    }, 1500);
  };

  const handleCopy = async () => {
    await navigator.clipboard.writeText(apiKey);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  if (generated) {
    return (
      <div className="relative min-h-screen flex items-center justify-center px-6">
        {/* Background glow */}
        <div className="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[600px] h-[400px] bg-emerald-500/5 rounded-full blur-[120px] pointer-events-none" />

        <div className="relative z-10 w-full max-w-lg text-center">
          {/* Success icon */}
          <div className="mx-auto w-16 h-16 rounded-full bg-emerald-500/10 border border-emerald-500/20 flex items-center justify-center mb-6">
            <KeyRound className="w-8 h-8 text-emerald-500" />
          </div>

          <h1 className="text-2xl font-bold text-white mb-2">
            API Key Generated
          </h1>
          <p className="text-slate-400 text-sm mb-8">
            Your enterprise API key has been created. Save it securely — you
            won&apos;t be able to view it again.
          </p>

          {/* API Key display */}
          <div className="flex items-center gap-2 p-3 rounded-xl bg-slate-900 border border-slate-800 font-mono text-sm">
            <code className="flex-1 text-left text-sky-400 truncate">
              {apiKey}
            </code>
            <button
              onClick={handleCopy}
              className="shrink-0 flex items-center gap-1.5 px-3 py-2 rounded-lg bg-slate-800 text-slate-300 text-xs hover:bg-slate-700 transition-colors"
            >
              {copied ? (
                <>
                  <Check className="w-3.5 h-3.5 text-emerald-500" />
                  Copied
                </>
              ) : (
                <>
                  <Copy className="w-3.5 h-3.5" />
                  Copy
                </>
              )}
            </button>
          </div>

          {/* Actions */}
          <div className="mt-8">
            <Link
              href="/dashboard"
              className="inline-flex items-center gap-2 px-8 py-3.5 rounded-xl bg-sky-500 text-slate-950 font-semibold text-base transition-all duration-200 hover:bg-sky-400 hover:shadow-[0_0_30px_rgba(56,189,248,0.4)] hover:-translate-y-0.5"
            >
              Go to Dashboard
              <ArrowLeft className="w-4 h-4 rotate-180" />
            </Link>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="relative min-h-screen flex items-center justify-center px-6">
      {/* Background glow */}
      <div className="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[600px] h-[500px] bg-sky-500/5 rounded-full blur-[120px] pointer-events-none" />

      <div className="relative z-10 w-full max-w-lg">
        {/* Header */}
        <div className="text-center mb-10">
          <div className="mx-auto w-14 h-14 rounded-2xl bg-sky-500/10 border border-sky-500/20 flex items-center justify-center mb-4">
            <KeyRound className="w-7 h-7 text-sky-400" />
          </div>
          <h1 className="text-2xl font-bold text-white mb-2">
            Enterprise Setup
          </h1>
          <p className="text-slate-400 text-sm">
            Connect your integrations to generate your API key.
          </p>
        </div>

        {/* Form */}
        <form onSubmit={handleSubmit} className="space-y-6">
          <div className="space-y-2">
            <Label htmlFor="email" className="text-slate-300">
              Lead Engineer Email
            </Label>
            <Input
              id="email"
              type="email"
              placeholder="eng-lead@company.com"
              value={email}
              onChange={(e) => setEmail(e.target.value)}
              required
              className="bg-slate-900 border-slate-800 text-white placeholder:text-slate-600 focus-visible:border-sky-500/50 focus-visible:ring-sky-500/20"
            />
          </div>

          <div className="space-y-2">
            <Label htmlFor="phone" className="text-slate-300">
              CEO Phone Number (Twilio)
            </Label>
            <Input
              id="phone"
              type="tel"
              placeholder="+1 (555) 000-0000"
              value={phone}
              onChange={(e) => setPhone(e.target.value)}
              required
              className="bg-slate-900 border-slate-800 text-white placeholder:text-slate-600 focus-visible:border-sky-500/50 focus-visible:ring-sky-500/20"
            />
          </div>

          <div className="space-y-2">
            <Label htmlFor="whatsapp" className="text-slate-300">
              Client WhatsApp Number
            </Label>
            <Input
              id="whatsapp"
              type="tel"
              placeholder="+1 (555) 000-0000"
              value={whatsapp}
              onChange={(e) => setWhatsapp(e.target.value)}
              required
              className="bg-slate-900 border-slate-800 text-white placeholder:text-slate-600 focus-visible:border-sky-500/50 focus-visible:ring-sky-500/20"
            />
          </div>

          <button
            type="submit"
            disabled={loading}
            className="w-full flex items-center justify-center gap-2 px-6 py-3.5 rounded-xl bg-sky-500 text-slate-950 font-semibold text-base transition-all duration-200 hover:bg-sky-400 hover:shadow-[0_0_30px_rgba(56,189,248,0.4)] hover:-translate-y-0.5 disabled:opacity-60 disabled:cursor-not-allowed disabled:hover:translate-y-0 disabled:hover:shadow-none"
          >
            {loading ? (
              <>
                <Loader2 className="w-5 h-5 animate-spin" />
                Generating Key...
              </>
            ) : (
              <>
                <KeyRound className="w-5 h-5" />
                Generate API Key
              </>
            )}
          </button>
        </form>
      </div>
    </div>
  );
}
