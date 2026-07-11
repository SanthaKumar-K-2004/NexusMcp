#!/usr/bin/env python3
import os
import sys
import json
import platform
import subprocess
from pathlib import Path

def print_status(msg, status="INFO"):
    colors = {
        "INFO": "\033[94m[i]\033[0m",
        "SUCCESS": "\033[92m[✓]\033[0m",
        "WARNING": "\033[93m[!]\033[0m",
        "ERROR": "\033[91m[✗]\033[0m"
    }
    print(f"{colors.get(status, '[ ]')} {msg}")

def ensure_binary():
    root_dir = Path(__file__).parent.resolve()
    binary_name = "nexusmcp.exe" if platform.system() == "Windows" else "nexusmcp"
    binary_path = root_dir / "target" / "release" / binary_name
    
    if not binary_path.exists():
        print_status("NexusMCP release binary not found. Initiating build...", "INFO")
        try:
            subprocess.run(["cargo", "build", "--release"], check=True, cwd=root_dir)
            print_status("NexusMCP compiled successfully.", "SUCCESS")
        except subprocess.CalledProcessError as e:
            print_status(f"Compilation failed: {e}", "ERROR")
            sys.exit(1)
    else:
        print_status("NexusMCP release binary found.", "SUCCESS")
        
    return binary_path

def merge_mcp_config(config_path, binary_path):
    config_path = Path(config_path).expanduser()
    
    # Ensure parent directory exists
    config_path.parent.mkdir(parents=True, exist_ok=True)
    
    data = {}
    if config_path.exists():
        try:
            with open(config_path, 'r') as f:
                data = json.load(f)
            print_status(f"Found existing configuration at: {config_path}", "INFO")
        except Exception as e:
            print_status(f"Failed to read existing config at {config_path}: {e}. Creating a new one.", "WARNING")
            data = {}
            
    if "mcpServers" not in data or not isinstance(data["mcpServers"], dict):
        data["mcpServers"] = {}
        
    data["mcpServers"]["nexusmcp"] = {
        "command": str(binary_path),
        "args": ["mcp", "--stealth"]
    }
    
    try:
        with open(config_path, 'w') as f:
            json.dump(data, f, indent=2)
        print_status(f"Successfully integrated NexusMCP into: {config_path}", "SUCCESS")
    except Exception as e:
        print_status(f"Failed to write configuration to {config_path}: {e}", "ERROR")

def main():
    print("=" * 60)
    print("         NexusMCP All-in-One Installer & Auto-Configurator")
    print("=" * 60)
    
    # 1. Compile/Get Binary
    binary_path = ensure_binary()
    
    # 2. Get standard paths based on platform
    system = platform.system()
    home = Path.home()
    
    claude_paths = []
    cline_paths = []
    
    if system == "Darwin": # macOS
        claude_paths = [
            home / "Library" / "Application Support" / "Claude" / "claude_desktop_config.json"
        ]
        cline_paths = [
            home / "Library" / "Application Support" / "Code" / "User" / "globalStorage" / "saoudrizwan.claude-dev" / "settings" / "cline_mcp_settings.json",
            home / "Library" / "Application Support" / "Code" / "User" / "globalStorage" / "roovet.roo-cline" / "settings" / "cline_mcp_settings.json"
        ]
    elif system == "Windows":
        appdata = Path(os.environ.get("APPDATA", str(home / "AppData" / "Roaming")))
        claude_paths = [
            appdata / "Claude" / "claude_desktop_config.json"
        ]
        cline_paths = [
            appdata / "Code" / "User" / "globalStorage" / "saoudrizwan.claude-dev" / "settings" / "cline_mcp_settings.json",
            appdata / "Code" / "User" / "globalStorage" / "roovet.roo-cline" / "settings" / "cline_mcp_settings.json"
        ]
    else: # Linux and POSIX fallbacks
        claude_paths = [
            home / ".config" / "Claude" / "claude_desktop_config.json"
        ]
        cline_paths = [
            home / ".config" / "Code" / "User" / "globalStorage" / "saoudrizwan.claude-dev" / "settings" / "cline_mcp_settings.json",
            home / ".config" / "Code" / "User" / "globalStorage" / "roovet.roo-cline" / "settings" / "cline_mcp_settings.json"
        ]
    
    # 3. Configure Claude Desktop
    claude_configured = False
    for path in claude_paths:
        if path.parent.exists() or system != "Windows":
            merge_mcp_config(path, binary_path)
            claude_configured = True
            break
            
    # 4. Configure VS Code extensions (Cline and Roo Code)
    for path in cline_paths:
        if path.parent.parent.parent.parent.exists():
            merge_mcp_config(path, binary_path)
            
    # 5. Print GUI client instructions
    print("\n" + "-" * 50)
    print("🎨 Integration Guidelines for GUI Clients")
    print("-" * 50)
    print("1. Cursor Desktop:")
    print("   • Go to Settings -> Cursor Settings -> Features -> MCP.")
    print("   • Click '+ Add New MCP Server'.")
    print("   • Fill in the details:")
    print("     - Name: nexusmcp")
    print("     - Type: command")
    print(f"     - Command: {binary_path}")
    print("     - Arguments: mcp --stealth")
    print("   • Click 'Save' and reload the window.")
    
    print("\n2. Google Antigravity Agent Platform:")
    print("   • Add the following block to your active MCP setup configuration:")
    print(json.dumps({
        "mcpServers": {
            "nexusmcp": {
                "command": str(binary_path),
                "args": ["mcp", "--stealth"]
            }
        }
    }, indent=4))
    
    print("=" * 60)
    print("   ✓ Configuration Completed! Restart your IDE/client to load tools.")
    print("=" * 60)

if __name__ == "__main__":
    main()
