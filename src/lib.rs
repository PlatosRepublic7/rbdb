use std::io;
use std::io::Write;
use std::error::Error;
use std::collections::HashMap;

pub fn rbdb_run(store: &mut HashMap<String, String>) -> Result<(), Box<dyn Error>> {
    let mut input = String::new();

    loop {
        input.clear();
        
        // Print the prompt to let the user know they're in "query" mode
        print!("RBDB -> ");

        // Flush the output to ensure the prompt is displayed immediately
        io::stdout().flush()?;

        // Read a line from the standard input
        io::stdin().read_line(&mut input)?;

        if input.trim() == "quit" || input.trim() == "exit" {
            break
        }
        
        // Tokenize the input
        let tokens: Vec<&str> = input.split_ascii_whitespace().collect();

        // build_query returns a Result<_, Box<dyn Error>>
        // using ? means: on Err, immediately return that Err from 'run'
        let query = match Query::build_query(tokens) {
            Ok(q) => q,
            Err(e) => {
                eprintln!("Query is malformed: {e}");
                continue;
            }
        };

        // We now need to process the query, and update the store
        match process_query(&query, store) {
            Ok(r) => println!("{r}"),
            Err(e) => {
                eprintln!("Query processing failed: {e}");
                continue;
            }
        };

        // Print out the Query tokens
        // let value_display = query.value.as_deref().unwrap_or("");
        // println!("Query Contains: {:?} {} {}", query.q_type, query.key, value_display);
    }
    Ok(())
}

fn process_query(query: &Query, store: &mut HashMap<String, String>) -> Result<String, Box<dyn Error>> {
    let mut query_result = String::new();
    match query.q_type {
        QueryType::Insert => {
            if store.contains_key(&query.key) {
                eprintln!("Key {} already exists. Use UPDATE query instead", query.key);
            }

            if let Some(ref value) = query.value {
                store.insert(query.key.clone(), value.clone());
                query_result = format!("SUCCESS: Inserted {}:{} into database", query.key, value);
            } else {
                eprintln!("INSERT requires a value, but none was provided");
            }
        }
        QueryType::Select => {
            if let Some(value) = store.get(&query.key) {
                query_result = format!{"{}", value};
            } else {
                eprintln!{"No entry found for key = {}", query.key};
            }
        }
        QueryType::Update => {
            if store.contains_key(&query.key) {
                if let Some(ref value) = query.value {
                    store.insert(query.key.clone(), value.clone());
                    query_result = format!{"SUCCESS: Updated {} with {}", query.key, value};
                } else {
                    eprintln!("UPDATE requres a value, but none was provided");
                }
            } else {
                eprintln!("No entry found for key = {}", query.key);
            }
        }
        QueryType::Delete => {
            if store.remove(&query.key).is_some() {
                query_result = format!("SUCCESS: Deleted {}", query.key);
            } else {
                eprintln!("No entry found for key = {}", query.key);
            }
        }
    }

    Ok(query_result)
}

#[derive(Debug, PartialEq)]
enum QueryType {
    Insert,
    Select,
    Update,
    Delete,
}

#[derive(Debug)]
struct Query {
    q_type: QueryType,
    key: String,
    value: Option<String>,
}

impl Query {
    pub fn build_query(tokens: Vec<&str>) -> Result<Self, Box<dyn Error>> {
        // Require at least two tokens
        if tokens.len() < 2 {
            return Err("Not enough arguments".into());
        }

        // Convert the first token into a QueryType
        let q_type = match tokens[0].to_uppercase().as_str() {
            "INSERT" => QueryType::Insert,
            "SELECT" => QueryType::Select,
            "UPDATE" => QueryType::Update,
            "DELETE" => QueryType::Delete,
            _ => return Err("Invalid query type".into())
        };

        let key = tokens[1].to_string();
        let value = if tokens.len() > 2 {
            Some(tokens[2].to_string())
        } else {
            None
        };
    
        Ok(Query { q_type, key, value })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn good_query() {
        let tokens = vec!["insert", "key", "value"];
        let query = Query::build_query(tokens.clone()).unwrap();
        assert_eq!(query.q_type, QueryType::Insert);
        assert_eq!(query.key, tokens[1]);
        let value = query.value.as_deref().unwrap();
        assert_eq![value, tokens[2]];
    }

    #[test]
    fn bad_query() {
        let tokens = vec!["delete"];
        let query = Query::build_query(tokens);
        assert!(query.is_err());
        let query_err = query.unwrap_err();
        assert_eq!(query_err.to_string(), "Not enough arguments");
    }

    #[test]
    fn insert_query() {
        let query = Query{ q_type: QueryType::Insert, key: "some_key".to_string(), value: Some("some_value".to_string()) };
        let mut store: HashMap<String, String> = HashMap::new();
        let query_result = process_query(&query, &mut store).unwrap();
        let result_string = "SUCCESS: Inserted some_key:some_value into database".to_string();
        assert_eq!(query_result, result_string);
    }

    #[test]
    fn select_query() {
        let mut store: HashMap<String, String> = HashMap::new();
        store.insert("some_key".to_string(), "some_value".to_string());
        let query = Query{ q_type: QueryType::Select, key: "some_key".to_string() , value: None};
        let query_result = process_query(&query, &mut store).unwrap();
        let result_string = "some_value".to_string();
        assert_eq!(query_result, result_string);
    }

    #[test]
    fn update_query() {
        let mut store: HashMap<String, String> = HashMap::new();
        store.insert("some_key".to_string(), "some_value".to_string());
        let query = Query{ q_type: QueryType::Update, key: "some_key".to_string(), value: Some("new_value".to_string()) };
        let query_result = process_query(&query, &mut store).unwrap();
        let result_string = "SUCCESS: Updated some_key with new_value".to_string();
        assert_eq!(query_result, result_string);
    }

    #[test]
    fn delete_query() {
        let mut store: HashMap<String, String> = HashMap::new();
        store.insert("some_key".to_string(), "some_value".to_string());
        let query = Query{ q_type: QueryType::Delete, key: "some_key".to_string(), value: None };
        let query_result = process_query(&query, &mut store).unwrap();
        let result_string = "SUCCESS: Deleted some_key".to_string();
        assert_eq!(query_result, result_string);
    }
}