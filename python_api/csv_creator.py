import json

import os
import re

word_range = (1,20446) # inclusive
connection_range = (20447, 210814) # inclusive

parts_of_speech = {}

with open("parts_of_speech.json","r") as f:
    parts_of_speech = eval(f.read())

words = []
connections = []
def fix_words(stupid_lines):
    real_words = []
    for stupid_line in stupid_lines:
        real_word = stupid_line.split("[")[0].strip()
        if len(real_word) > 0:
            real_words.append(real_word)
    return real_words
def get_connections(bad_lines):
    connections = []
    for connection in bad_lines:
        pattern = r'(\w+)\s*->\s*(\w+)\s*\[label="(\d+)"\]'
        regex_match = re.search(pattern, connection)
        if regex_match:
            # Extract the groups from the match
            word1, word2, weight = regex_match.groups()
            weight = int(weight)  # Convert the weight to an integer
            connections.append((word1,word2,weight))

        else:
            print("No match found.")
    return connections
def get_word_class(word):
    word_class = parts_of_speech[word]
    if len(word_class) > 0:
        word_class = ";".join(list(set(word_class)))
    else:
        word_class = "Default"
    return word_class
with open("output.dot","r") as dot:
    lines = dot.readlines()
    
    words = fix_words(lines[word_range[0]:word_range[1]])
    connections = get_connections(lines[connection_range[0]:connection_range[1]])

with open("nodes.csv",'w') as f:
    lines = ["Label, Parts Of Speech"]
    for word in words:
        word_class = get_word_class(word) 
        lines.append("\n"+word+","+word_class)
    f.writelines(lines)
with open("connections.csv", "w") as f:
    connection_type = "Default"
    lines = ["Source, Target, Type, Weight"]
    for connection in connections:
        word1, word2, weight = connection
        lines.append("\n"+",".join((word1,word2,"undirected",str(weight))))
    f.writelines(lines)


