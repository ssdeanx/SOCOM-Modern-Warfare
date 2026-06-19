import os
os.chdir("C:/Users/ssdsk/projects/SOCOM")
content = open("docs/Specs/requirements.md", "r").read()
size = len(content)
print(f"Current file size: {size} bytes")
if size < 100:
    print("File needs rewriting")

