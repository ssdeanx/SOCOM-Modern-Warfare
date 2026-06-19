import os
os.chdir("C:/Users/ssdsk/projects/SOCOM")
lines = []
lines.append("# PRD Header")
lines.append("")
lines.append("## Test content")
with open("docs/Specs/requirements.md","w",encoding="utf-8") as out:
    out.write("
".join(lines))
print("Test PRD written")
