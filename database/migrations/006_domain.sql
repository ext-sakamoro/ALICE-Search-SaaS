-- ALICE Search: Domain-specific tables
CREATE TABLE IF NOT EXISTS search_indexes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES auth.users(id),
    name TEXT NOT NULL,
    doc_count BIGINT NOT NULL DEFAULT 0,
    size_bytes BIGINT NOT NULL DEFAULT 0,
    fm_index_size_bytes BIGINT NOT NULL DEFAULT 0,
    compression_ratio DOUBLE PRECISION NOT NULL DEFAULT 1.0,
    language TEXT NOT NULL DEFAULT 'en',
    status TEXT NOT NULL DEFAULT 'active' CHECK (status IN ('building', 'active', 'suspended')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS search_documents (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    index_id UUID NOT NULL REFERENCES search_indexes(id) ON DELETE CASCADE,
    doc_id TEXT NOT NULL,
    title TEXT NOT NULL,
    body TEXT NOT NULL,
    metadata JSONB,
    indexed_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS search_query_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    index_id UUID NOT NULL REFERENCES search_indexes(id) ON DELETE CASCADE,
    query TEXT NOT NULL,
    total_hits BIGINT NOT NULL DEFAULT 0,
    elapsed_us BIGINT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_search_indexes_user ON search_indexes(user_id);
CREATE INDEX idx_search_documents_index ON search_documents(index_id, doc_id);
CREATE INDEX idx_search_query_logs_index ON search_query_logs(index_id, created_at);
