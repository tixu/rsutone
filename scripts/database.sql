CREATE TABLE reports (
    id INTEGER PRIMARY KEY,
    app_name TEXT NOT NULL,
    app_version TEXT NOT NULL,
    general_info TEXT NOT NULL,
    error TEXT NOT NULL,
    error_status TEXT NOT NULL
);
