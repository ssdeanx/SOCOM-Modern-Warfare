
import os
os.chdir("C:/Users/ssdsk/projects/SOCOM")
with open("docs/Specs/requirements.md", "w", encoding="utf-8") as f:
    f.write("""---
title: "SOCOM Tactical Shooter - Product Requirements Document"
version: "3.0.0"
date: "2026-06-19"
author: "Sam / Quicksilver"
status: "Active - Phase 2 Systems Complete, Pre-Asset"
license: "MIT"
---

# SOCOM Tactical Shooter â€” Product Requirements Document

## 1. Executive Summary

**SOCOM Tactical Shooter** is a modern reimagining of the classic PS2 tactical shooter (SOCOM: U.S. Navy SEALs) rebuilt for PC with AAA production values using Rust and the Bevy 0.18.1 engine. It targets authentic squad-based tactical combat inspired by Arma 3, Squad, and Ghost Recon - featuring combined 1st/3rd person perspectives, an 8-component modular weapon system, deep gear progression, drone warfare, AI teammates, and competitive multiplayer modes.

**Current Status:** All core gameplay systems implemented and compiling (91 source files, 0 errors). Pre-asset - awaiting audio, 3D models, and VFX before first playable build.

**Development Philosophy:** Enterprise-grade architecture with zero Bevy dependency in core crate, message-driven inter-system communication, single-responsibility files, and complete systems built before asset integration.
""")
print("Header written")

import os
os.chdir("C:/Users/ssdsk/projects/SOCOM")
with open("docs/Specs/requirements.md", "w", encoding="utf-8") as f:
    pass
print("Cleared")
