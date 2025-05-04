-- Add up migration script here
CREATE UNIQUE INDEX idx_unique_file_path ON project_files(file_path);