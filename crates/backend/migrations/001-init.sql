CREATE TABLE IF NOT EXISTS photos
(
    id INT PRIMARY KEY AUTO_INCREMENT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    uploaded_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    url TEXT NOT NULL UNIQUE,
    title TEXT NOT NULL,
    description TEXT
);

CREATE INDEX IF NOT EXISTS idx_photos_on_created_at ON photos( created_at DESC);
CREATE INDEX IF NOT EXISTS idx_photos_on_uploaded_at ON photos( uploaded_at DESC);

-- Re-cluster photos based on the uploaded_at column.
CLUSTER photos USING idx_photos_on_uploaded_at;
