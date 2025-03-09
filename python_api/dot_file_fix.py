fixed = ""
with open("../src/output.dot","r") as f:
    f.seek(0)
    stuff = f.read()
    flagged_words = [" edge "," graph ", " digraph ", " node "," strict "]
    fixed = stuff
    print(stuff[:100])
    for flagged_word in flagged_words:
        fixed = fixed.replace(flagged_word, f'"{flagged_word.strip()}"')
with open("../src/output.dot", "w") as f:
    f.write(fixed)
