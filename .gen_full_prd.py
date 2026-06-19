import os
os.chdir("C:/Users/ssdsk/projects/SOCOM")
print("Writing PRD...")
with open("docs/Specs/requirements.md", "w", encoding="utf-8") as r:
    r.write("# SOCOM PRD\n\nTest write successful\n")
print("Done")
