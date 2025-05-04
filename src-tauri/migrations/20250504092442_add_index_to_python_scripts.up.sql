-- Add up migration script here
CREATE UNIQUE INDEX idx_unique_python_script_file_path ON python_scripts(script_file_path);