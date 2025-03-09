use petgraph::stable_graph::StableGraph;
use petgraph::dot::{Dot, Config};
use reqwest::Error;
use serde::Deserialize;
use std::fs::File;
use std::io::Write;
use std::io;
use petgraph::visit::IntoEdgeReferences;
use petgraph::visit::EdgeRef;
use std::collections::HashMap;
#[derive(Debug, Deserialize)]
struct WordEntry {
    definitions: Vec<Definition>,
}

#[derive(Debug, Deserialize)]
struct Definition {
    partOfSpeech: String,
    definition: Vec<String>,
}

#[tokio::main]
async fn main() ->std::io::Result<()> {
    println!("Hello World");

    // Create a graph with `String` as node type
    let mut graph = StableGraph::<String, u32>::new();
    let origin = "a".to_string(); // Start word is "a"
    graph.add_node(origin.clone());

    // Initialize definitions for depth-first expansion
    let mut current_definitions: Vec<Vec<String>> = Vec::new();
    current_definitions.push(vec![origin.clone()]); // Start depth with "a"

    let mut depth_index = 0;
    let mut def_length = 1;
    let mut extra_data = HashMap::new();
    
    while depth_index < def_length {
        if !current_definitions.is_empty() {
            let current_word_list = current_definitions.get(depth_index).unwrap().clone();

            for word in current_word_list {
                println!("{}", word);

                let (definitions, parts_of_speech) = get_definitions(word.as_str()).await;
                extra_data.insert(word.clone(), parts_of_speech);
                if !definitions.is_empty() {
                    // Add connections and update graph
                    
                    let definitions_list = remove_duplicates(&definitions, &mut graph);
                    add_connections(word.clone(), definitions.clone(), &mut graph);
                    
                    if !definitions_list.is_empty() {
                        
                        current_definitions.push(definitions_list);
                        def_length +=1;
                    }
                } else {
                    println!("Incomplete dictionary result: expected at least 1 element.");
                }
            }

            depth_index += 1;
        } else {
            break;
        }
    }

    println!("PROGRAM STOPPED");
    export_dot_file(&graph, "output.dot")?;
    export_type_json(extra_data);
    println!("Graph exported to 'output.dot'");
    

    Ok(())
}

async fn search_dictionary(word: &str) -> Result<Vec<WordEntry>, bool> {
    let url = format!("http://localhost:8000/{}", word);
    let response = reqwest::get(&url).await;

    match response {
        Ok(resp) if resp.status().is_success() => {
            let word_data: Result<Vec<WordEntry>, Error> = resp.json().await;
            match word_data {
                Ok(data) => Ok(data),
                Err(_) => Err(false),
            }
        }
        _ => Err(false),
    }
}

async fn get_definitions(word: &str) -> (Vec<String>, Vec<String>) {
    let word_data: Option<Vec<WordEntry>> = match search_dictionary(word).await {
        Ok(data) => Some(data),
        Err(_) => None,
    };

    let mut definitions_list: Vec<String> = Vec::new();
    let mut parts_of_speech_list: Vec<String> = Vec::new();

    if let Some(word_data) = word_data {
        if let Some(word_entry) = word_data.get(0) {
            for def in &word_entry.definitions {
                definitions_list.extend(def.definition.clone());
                parts_of_speech_list.push(def.partOfSpeech.clone());
            }
        } else {
            println!("No word entry found.");
        }
    } else {
        println!("No word data available.");
    }

    (definitions_list, parts_of_speech_list)
}

fn add_connection(word1: String, word2: String, graph: &mut StableGraph<String, u32>) {
    let word1_node = graph
        .node_indices()
        .find(|&n| graph[n] == word1)
        .unwrap_or_else(|| graph.add_node(word1.clone()));

    let word2_node = graph
        .node_indices()
        .find(|&n| graph[n] == word2)
        .unwrap_or_else(|| graph.add_node(word2.clone()));

    if let Some(edge_index) = graph.find_edge(word1_node, word2_node) {
        graph[edge_index] += 1;
        
    } else {
        graph.add_edge(word1_node, word2_node, 1);
        
    }
}

fn add_connections(origin: String, words: Vec<String>, graph: &mut StableGraph<String, u32>) {
    for word in words {
        add_connection(origin.clone(), word, graph);
    }
}

fn remove_duplicates(words: &[String], graph: &StableGraph<String, u32>) -> Vec<String> {
    let mut new_words: Vec<String> = Vec::new();

    for word in words {
        let exists = graph.node_indices().any(|n| {
            let node_value = &graph[n];
            
            node_value == word
        });

        if !exists {
            
            new_words.push(word.clone());
        } else {
            
            continue;
        }
    }

    new_words
}

fn export_dot_file(graph: &StableGraph<String, u32>, file_path: &str) -> io::Result<()> {
    // Open the file for writing
    let mut file = File::create(file_path)?;

    // Write the DOT graph structure with custom labels
    writeln!(file, "digraph {{")?;
    for node_index in graph.node_indices() {
        let label = &graph[node_index]; // Node label (the word)
        writeln!(file, "    {} [label=\"{}\"];", label,label)?;
    }

    for edge in graph.edge_references() {
        let source = edge.source();
        let target = edge.target();
        let weight = edge.weight();
        let source_label = graph.node_weight(source).unwrap();
        let target_label = graph.node_weight(target).unwrap();
        writeln!(file, "    {} -> {} [label=\"{}\"];", source_label, target_label, weight)?;
    }

    writeln!(file, "}}")?;
    Ok(())
}
fn export_type_json(data:HashMap<String, Vec<String>){
    let serialized_data = serde_json::to_string(&data).unwrap();
}
