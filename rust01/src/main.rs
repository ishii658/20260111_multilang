use postgres::{Client, NoTls};
use std::env;

// - documents: ドキュメントメタ情報格納用
//   - id: SERIAL PRIMARY KEY
//   - file_name: TEXT
//   - url: TEXT
//   - shecksum: TEXT
//   - created_at: TIMESTAMP DEFAULT CURRENT_TIMESTAMP
// - markdown: Markdownテキスト格納用
//     - id: SERIAL PRIMARY KEY
//     - document_id: INTEGER REFERENCES documents(id) ON DELETE CASCADE
//     - page_number: INTEGER
//     - text: TEXT
// - vectors: 埋め込みベクトル格納用
//   - id: SERIAL PRIMARY KEY
//   - document_id: INTEGER REFERENCES documents(id) ON DELETE CASCADE
//   - page_number: INTEGER
//   - vector: REAL[]
// - img_vectors: 画像埋め込みベクトル格納用
//   - id: SERIAL PRIMARY KEY
//   - document_id: INTEGER REFERENCES documents(id) ON DELETE CASCADE
//   - page_number: INTEGER
//   - vector: REAL[]

fn main() {
    let db_name = env::var("DB_NAME").expect("DB_NAME environment variable not set");
    let db_host = env::var("DB_HOST").expect("DB_HOST environment variable not set");
    let db_port = "5432"; // Default PostgreSQL port
    let password = env::var("DB_PASS").expect("DB_PASS environment variable not set");
    let user = env::var("DB_USER").expect("DB_USER environment variable not set");
    let mut client = Client::connect(
        &format!(
            "host={} user={} password={} dbname={} port={}",
            db_host, user, password, db_name, db_port
        ),
        NoTls,
    )
    .expect("Failed to connect to database");

    // Parse CLI arguments for schema/table (supports -s/--schema, -t/--table, or positional). Defaults: schema=public, table=documents
    let args: Vec<String> = env::args().collect();
    let mut schema = env::var("DB_SCHEMA").unwrap_or_else(|_| "public".to_string());
    let mut table = "documents".to_string();
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-s" | "--schema" => {
                if i + 1 < args.len() {
                    schema = args[i + 1].clone();
                    i += 2;
                    continue;
                } else {
                    eprintln!("Expected schema name after {}", args[i]);
                    std::process::exit(1);
                }
            }
            "-t" | "--table" => {
                if i + 1 < args.len() {
                    table = args[i + 1].clone();
                    i += 2;
                    continue;
                } else {
                    eprintln!("Expected table name after {}", args[i]);
                    std::process::exit(1);
                }
            }
            _ => {
                // Positional args: first -> schema (if not provided), second -> table (if not provided)
                if schema == "public" {
                    schema = args[i].clone();
                } else if table == "documents" {
                    table = args[i].clone();
                }
                i += 1;
            }
        }
    }

    // Validate schema/table name (simple identifier check)
    fn is_valid_ident(s: &str) -> bool {
        let mut chars = s.chars();
        match chars.next() {
            Some(c) if c.is_ascii_alphabetic() || c == '_' => (),
            _ => return false,
        }
        for c in chars {
            if !(c.is_ascii_alphanumeric() || c == '_') {
                return false;
            }
        }
        true
    }

    if !is_valid_ident(&schema) {
        eprintln!("Invalid schema name: {}", schema);
        std::process::exit(1);
    }
    if !is_valid_ident(&table) {
        eprintln!("Invalid table name: {}", table);
        std::process::exit(1);
    }

    println!("Attempting to query table '{}.{}'...", schema, table);

    // Verify schema exists
    match client.query_one(
        "SELECT EXISTS (SELECT 1 FROM information_schema.schemata WHERE schema_name = $1)",
        &[&schema],
    ) {
        Ok(row) => {
            let exists: bool = row.get(0);
            if !exists {
                eprintln!("Schema '{}' does not exist", schema);
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Error checking schema existence: {:?}", e);
            std::process::exit(1);
        }
    }

    // Verify table exists
    match client.query_one(
        "SELECT EXISTS (SELECT 1 FROM information_schema.tables WHERE table_schema = $1 AND table_name = $2)",
        &[&schema, &table],
    ) {
        Ok(row) => {
            let exists: bool = row.get(0);
            if !exists {
                eprintln!("Table '{}' does not exist in schema '{}'", table, schema);
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Error checking table existence: {:?}", e);
            std::process::exit(1);
        }
    }

    // Check which columns are available and choose query accordingly
    let mut has_required_cols = true;
    for col in ["relevance_score", "possibility_score", "file_name"].iter() {
        match client.query_one(
            "SELECT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_schema = $1 AND table_name = $2 AND column_name = $3)",
            &[&schema, &table, col],
        ) {
            Ok(r) => {
                let exists: bool = r.get(0);
                if !exists {
                    has_required_cols = false;
                    break;
                }
            }
            Err(e) => {
                eprintln!("Error checking for column {}: {:?}", col, e);
                std::process::exit(1);
            }
        }
    }

    if has_required_cols {
        let query = format!(
            "SELECT relevance_score, possibility_score, file_name FROM {}.{} LIMIT 5",
            schema, table
        );
        match client.query(&query, &[],) {
            Ok(rows) => {
                println!("Query successful! Found {} rows.", rows.len());

                if rows.is_empty() {
                    println!("No data found in {}.{}.", schema, table);
                    return;
                }

                println!("Query results:");
                println!("relevance_score, possibility_score, document_name");
                println!("-----------------------------");

                // Iterate through rows and extract the actual values
                for row in &rows {
                    let relevance_score: Option<i32> = row.get(0);
                    let possibility_score: Option<i32> = row.get(1);
                    let document_name: Option<String> = row.get(2);

                    match (relevance_score, possibility_score, document_name) {
                        (Some(rel), Some(poss), Some(dn)) => {
                            println!("{}, {}, {}", rel, poss, dn);
                        }
                        _ => {
                            println!("(NULL), (NULL)");
                        }
                    }
                }
            },
            Err(e) => {
                eprintln!("Error executing query: {:?}", e);
                eprintln!("This might be because the table doesn't exist, the columns are missing, or there are permission issues.");
                eprintln!("Make sure the table '{}' exists in the '{}' schema, has the expected columns, and you have proper permissions.", table, schema);
            }
        }
    } else {
        // Fallback: check for id and file_name
        let mut has_id_name = true;
        for col in ["id", "file_name"].iter() {
            match client.query_one(
                "SELECT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_schema = $1 AND table_name = $2 AND column_name = $3)",
                &[&schema, &table, col],
            ) {
                Ok(r) => {
                    let exists: bool = r.get(0);
                    if !exists {
                        has_id_name = false;
                        break;
                    }
                }
                Err(e) => {
                    eprintln!("Error checking for column {}: {:?}", col, e);
                    std::process::exit(1);
                }
            }
        }

        if has_id_name {
            let query = format!("SELECT id, file_name FROM {}.{} LIMIT 5", schema, table);
            match client.query(&query, &[],) {
                Ok(rows) => {
                    println!("Query successful! Found {} rows.", rows.len());

                    if rows.is_empty() {
                        println!("No data found in {}.{}.", schema, table);
                        return;
                    }

                    println!("Query results:");
                    println!("id, file_name");
                    println!("-----------------------------");

                    for row in &rows {
                        let id: Option<i32> = row.get(0);
                        let file_name: Option<String> = row.get(1);

                        match (id, file_name) {
                            (Some(idv), Some(fnv)) => {
                                println!("{}, {}", idv, fnv);
                            }
                            _ => {
                                println!("(NULL), (NULL)");
                            }
                        }
                    }
                },
                Err(e) => {
                    eprintln!("Error executing fallback query: {:?}", e);
                    std::process::exit(1);
                }
            }
        } else {
            eprintln!("Table '{}' in schema '{}' does not contain expected columns. Required: (relevance_score, possibility_score, file_name) or fallback (id, file_name).", table, schema);
            std::process::exit(1);
        }
    }
}

