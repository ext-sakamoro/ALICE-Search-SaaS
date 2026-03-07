# ALICE-Search-SaaS

Full-text search service backed by an FM-Index (Burrows-Wheeler Transform) with 2.5x compression ratio, faceted search, multi-language tokenization, and sub-millisecond autocomplete.

## Architecture

```
Client
  │
  ▼
┌─────────────────────────────────────────┐
│          ALICE-Search-SaaS API          │
│         (Rust / Axum, port 8081)        │
└────────────┬──────────────┬────────────┘
             │              │
    ┌─────────────┐  ┌──────────────────┐
    │  Query      │  │  Indexer         │
    │  Processor  │  │  (BWT + FM-Index)│
    └──────┬──────┘  └────────┬─────────┘
           │                  │
  ┌────────▼──────────────────▼─────────┐
  │           FM-Index Core             │
  │  (2.5x compression, in-memory)      │
  └────────────────────────────────────┘
           │
  ┌────────▼────────┐  ┌───────────────┐
  │  Facet Engine   │  │  Autocomplete │
  │  (Field filters)│  │  (Trie index) │
  └────────┬────────┘  └───────┬───────┘
           │                   │
  ┌────────▼───────────────────▼────────┐
  │   Multi-Language Tokenizer          │
  │   (EN, JA, FR, DE, ES, ...)        │
  └─────────────────────────────────────┘
```

## Features

| Feature | Details |
|---------|---------|
| FM-Index Full-Text Search | Burrows-Wheeler Transform; 2.5x compression ratio |
| Faceted Search | Filter and aggregate by any indexed field |
| Autocomplete | Compressed trie prefix search, under 1 ms |
| Multi-Language | Built-in tokenizers for EN, JA, FR, DE, ES and more |
| In-Memory Index | Entire index fits in RAM for microsecond query latency |

## API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/api/v1/search/query` | Full-text search with optional facet filters |
| POST | `/api/v1/search/index` | Add or update a document in an index |
| POST | `/api/v1/search/autocomplete` | Prefix-based autocomplete suggestions |
| GET | `/api/v1/search/indexes` | List all indexes with document count and size |
| GET | `/api/v1/search/stats` | Query throughput, latency, and compression ratio |

## Quick Start

```bash
# Clone and start the backend
git clone https://github.com/your-org/ALICE-Search-SaaS.git
cd ALICE-Search-SaaS
cargo run --release

# In a second terminal, start the frontend
cd frontend
npm install
npm run dev
# Open http://localhost:3000
```

### Example: Index a Document

```bash
curl -X POST http://localhost:8081/api/v1/search/index \
  -H "Content-Type: application/json" \
  -d '{
    "index": "articles",
    "document": {
      "id": "doc-1",
      "title": "ALICE Search Engine",
      "body": "Full-text search powered by FM-Index.",
      "category": "tech"
    }
  }'
```

### Example: Full-Text Search with Facets

```bash
curl -X POST http://localhost:8081/api/v1/search/query \
  -H "Content-Type: application/json" \
  -d '{
    "index": "articles",
    "query": "ALICE search engine",
    "language": "en",
    "facets": ["category", "author"]
  }'
```

### Example: Autocomplete

```bash
curl -X POST http://localhost:8081/api/v1/search/autocomplete \
  -H "Content-Type: application/json" \
  -d '{"index":"articles","prefix":"alice s"}'
```

## License

AGPL-3.0-or-later — see [LICENSE](LICENSE) for details.
