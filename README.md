cargo install sqlx-cli --no-default-features --features sqlite

sqlx database create --database-url sqlite://C:/Users/J/AppData/Roaming/com.bakalaurs.blendio-tauri/test.db

sqlx migrate add create_users_table