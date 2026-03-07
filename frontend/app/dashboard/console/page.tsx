"use client";

import { useState } from "react";

const API_BASE = process.env.NEXT_PUBLIC_API_URL ?? "http://localhost:8081";

type Tab = "search" | "index" | "autocomplete" | "stats";

export default function ConsolePage() {
  const [tab, setTab] = useState<Tab>("search");
  const [result, setResult] = useState<string>("");
  const [loading, setLoading] = useState(false);

  // search
  const [searchIndex, setSearchIndex] = useState("articles");
  const [searchQuery, setSearchQuery] = useState("ALICE search engine");
  const [searchLang, setSearchLang] = useState("en");
  const [searchFacets, setSearchFacets] = useState("category,author");

  // index
  const [indexName, setIndexName] = useState("articles");
  const [indexDoc, setIndexDoc] = useState(
    '{"id":"doc-1","title":"ALICE Search","body":"Full-text search powered by FM-Index.","category":"tech"}'
  );

  // autocomplete
  const [acIndex, setAcIndex] = useState("articles");
  const [acPrefix, setAcPrefix] = useState("alice s");

  const run = async () => {
    setLoading(true);
    setResult("");
    try {
      let res: Response;
      if (tab === "search") {
        res = await fetch(`${API_BASE}/api/v1/search/query`, {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({
            index: searchIndex,
            query: searchQuery,
            language: searchLang,
            facets: searchFacets.split(",").map((f) => f.trim()).filter(Boolean),
          }),
        });
      } else if (tab === "index") {
        res = await fetch(`${API_BASE}/api/v1/search/index`, {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({
            index: indexName,
            document: JSON.parse(indexDoc),
          }),
        });
      } else if (tab === "autocomplete") {
        res = await fetch(`${API_BASE}/api/v1/search/autocomplete`, {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({ index: acIndex, prefix: acPrefix }),
        });
      } else {
        res = await fetch(`${API_BASE}/api/v1/search/stats`);
      }
      const json = await res.json();
      setResult(JSON.stringify(json, null, 2));
    } catch (e) {
      setResult(`Error: ${e instanceof Error ? e.message : String(e)}`);
    } finally {
      setLoading(false);
    }
  };

  const tabs: Tab[] = ["search", "index", "autocomplete", "stats"];

  return (
    <div className="min-h-screen bg-gray-900 text-green-400 p-6 font-mono">
      <h1 className="text-2xl font-bold mb-6 text-green-300">
        ALICE-Search-SaaS Console
      </h1>

      {/* Tab bar */}
      <div className="flex gap-2 mb-6">
        {tabs.map((t) => (
          <button
            key={t}
            onClick={() => { setTab(t); setResult(""); }}
            className={`px-4 py-2 rounded text-sm font-semibold transition-colors ${
              tab === t
                ? "bg-green-600 text-gray-900"
                : "bg-gray-800 text-green-400 hover:bg-gray-700"
            }`}
          >
            {t.toUpperCase()}
          </button>
        ))}
      </div>

      {/* Tab content */}
      <div className="bg-gray-800 rounded-lg p-6 mb-6 space-y-4">
        {tab === "search" && (
          <>
            <div>
              <label className="block text-xs text-green-500 mb-1">Index</label>
              <input
                value={searchIndex}
                onChange={(e) => setSearchIndex(e.target.value)}
                className="w-full bg-gray-900 border border-gray-700 rounded px-3 py-2 text-green-400 text-sm"
              />
            </div>
            <div>
              <label className="block text-xs text-green-500 mb-1">Query</label>
              <input
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                className="w-full bg-gray-900 border border-gray-700 rounded px-3 py-2 text-green-400 text-sm"
              />
            </div>
            <div className="grid grid-cols-2 gap-4">
              <div>
                <label className="block text-xs text-green-500 mb-1">Language</label>
                <select
                  value={searchLang}
                  onChange={(e) => setSearchLang(e.target.value)}
                  className="w-full bg-gray-900 border border-gray-700 rounded px-3 py-2 text-green-400 text-sm"
                >
                  <option value="en">English</option>
                  <option value="ja">Japanese</option>
                  <option value="fr">French</option>
                  <option value="de">German</option>
                  <option value="es">Spanish</option>
                </select>
              </div>
              <div>
                <label className="block text-xs text-green-500 mb-1">
                  Facets (comma-separated)
                </label>
                <input
                  value={searchFacets}
                  onChange={(e) => setSearchFacets(e.target.value)}
                  className="w-full bg-gray-900 border border-gray-700 rounded px-3 py-2 text-green-400 text-sm"
                />
              </div>
            </div>
          </>
        )}

        {tab === "index" && (
          <>
            <div>
              <label className="block text-xs text-green-500 mb-1">Index Name</label>
              <input
                value={indexName}
                onChange={(e) => setIndexName(e.target.value)}
                className="w-full bg-gray-900 border border-gray-700 rounded px-3 py-2 text-green-400 text-sm"
              />
            </div>
            <div>
              <label className="block text-xs text-green-500 mb-1">Document (JSON)</label>
              <textarea
                value={indexDoc}
                onChange={(e) => setIndexDoc(e.target.value)}
                rows={5}
                className="w-full bg-gray-900 border border-gray-700 rounded px-3 py-2 text-green-400 text-sm resize-none"
              />
            </div>
          </>
        )}

        {tab === "autocomplete" && (
          <>
            <div>
              <label className="block text-xs text-green-500 mb-1">Index</label>
              <input
                value={acIndex}
                onChange={(e) => setAcIndex(e.target.value)}
                className="w-full bg-gray-900 border border-gray-700 rounded px-3 py-2 text-green-400 text-sm"
              />
            </div>
            <div>
              <label className="block text-xs text-green-500 mb-1">Prefix</label>
              <input
                value={acPrefix}
                onChange={(e) => setAcPrefix(e.target.value)}
                className="w-full bg-gray-900 border border-gray-700 rounded px-3 py-2 text-green-400 text-sm"
              />
            </div>
          </>
        )}

        {tab === "stats" && (
          <p className="text-green-500 text-sm">
            Fetches GET /api/v1/search/stats — click Run to retrieve index
            sizes, query throughput, and compression ratio metrics.
          </p>
        )}
      </div>

      <button
        onClick={run}
        disabled={loading}
        className="px-6 py-2 bg-green-600 hover:bg-green-500 disabled:bg-gray-700 text-gray-900 font-bold rounded transition-colors"
      >
        {loading ? "Running..." : "Run"}
      </button>

      {result && (
        <pre className="mt-6 bg-gray-800 rounded-lg p-4 text-green-300 text-sm overflow-x-auto whitespace-pre-wrap">
          {result}
        </pre>
      )}
    </div>
  );
}
