from fastapi import FastAPI, HTTPException
import re
import requests
api_to_contact = "https://api.dictionaryapi.dev/api/v2/entries/en/"
app = FastAPI()
@app.get("/{term}")
async def get_definition(term: str):
    url = api_to_contact + term
    result = requests.get(url)
    if result.status_code == 200:
        return process_json(result.json())
    else:
        return False
def process_json(data:list):
    result = []
    
    for entry in data:
        word_info = {
            
            "definitions": []
        }
        
        for meaning in entry.get("meanings", []):
            part_of_speech = meaning.get("partOfSpeech", "")
            for definition_entry in meaning.get("definitions", []):
                definition = {
                    "partOfSpeech": part_of_speech.lower(),
                    "definition": process_def(definition_entry.get("definition")),
                }
                word_info["definitions"].append(definition)
        
        result.append(word_info)
    
    return result
def process_def(definition:str):
     # Replace all non-alphabetic characters with spaces
    cleaned_text = re.sub(r'[^a-zA-Z\s]', ' ', definition).lower();
    # Remove extra whitespace and return the result
    return list(set(re.sub(r'\s+', ' ', cleaned_text).strip().split(" ")))

