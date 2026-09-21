CREATE TABLE IF NOT EXISTS settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL
);

INSERT OR IGNORE INTO settings (key, value) VALUES ('show_recent_notes', 'true');
INSERT OR IGNORE INTO settings (key, value) VALUES ('recent_notes_count', '5');
INSERT OR IGNORE INTO settings (key, value) VALUES ('auto_save', 'true');
INSERT OR IGNORE INTO settings (key, value) VALUES ('auto_save_delay_ms', '1500');
INSERT OR IGNORE INTO settings (key, value) VALUES ('enable_draft_recovery', 'true');

