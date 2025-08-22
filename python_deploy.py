#!/usr/bin/env python3

"""
Neo N3 Rust Framework - Python Contract Deployment Script
Uses Neo Python SDK for reliable contract deployment and invocation
"""

import subprocess
import time
import json
import os
import sys

def run_command(cmd, timeout=30):
    """Run a shell command with timeout"""
    try:
        result = subprocess.run(cmd, shell=True, capture_output=True, text=True, timeout=timeout)
        return result.returncode == 0, result.stdout, result.stderr
    except subprocess.TimeoutExpired:
        return False, "", "Command timed out"

def main():
    print("🚀 Neo N3 Rust Framework - Python Deployment Test")
    print("=" * 50)
    
    # Set up environment
    os.environ["PATH"] = os.environ.get("PATH", "") + ":" + os.path.expanduser("~/.dotnet/tools")
    
    print("📦 Starting Neo Express...")
    
    # Reset and start Neo Express
    neoxp_path = os.path.expanduser("~/.dotnet/tools/neoxp")
    success, _, _ = run_command(f"{neoxp_path} reset --force")
    if not success:
        print("❌ Failed to reset Neo Express")
        return
    
    # Start Neo Express in background
    neoxp_path = os.path.expanduser("~/.dotnet/tools/neoxp")
    process = subprocess.Popen(
        [neoxp_path, "run", "--seconds-per-block", "1"],
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        shell=False
    )
    
    print("⏳ Waiting for blockchain to start...")
    time.sleep(15)
    
    print("💰 Checking wallet balances...")
    success, output, _ = run_command(f"{neoxp_path} show balance NEO genesis")
    if success:
        print(f"  💰 NEO Balance: {output.strip()}")
    
    success, output, _ = run_command(f"{neoxp_path} show balance GAS genesis")
    if success:
        print(f"  ⛽ GAS Balance: {output.strip()}")
    
    print("\n📋 Listing available contracts to deploy...")
    contracts = [
        ("Hello World", "build/examples/01-hello-world/hello_world_example.nef"),
        ("NEP-17 Token", "build/examples/04-nep17-token/nep17_token.nef"),
        ("Real NEP-17", "build/examples/defi/real-nep17-token/real_nep17_token.nef"),
        ("Uniswap AMM", "build/examples/defi/real-uniswap-amm/real_uniswap_amm.nef"),
        ("Compound Lending", "build/examples/defi/real-compound-lending/real_compound_lending.nef"),
        ("Aave Flash", "build/examples/defi/real-aave-flash/real_aave_flash.nef"),
    ]
    
    deployed_contracts = []
    
    for name, nef_path in contracts:
        if os.path.exists(nef_path):
            print(f"\n🚀 Deploying {name}...")
            print(f"  📄 NEF: {nef_path}")
            
            # Try deployment with --password option
            cmd = f"echo '' | {neoxp_path} contract deploy {nef_path} genesis --password ''"
            success, output, error = run_command(cmd, timeout=60)
            
            if success and "deployed successfully" in output.lower():
                print(f"  ✅ {name} deployed successfully!")
                deployed_contracts.append((name, nef_path))
                
                # Extract contract hash if available
                lines = output.split('\n')
                for line in lines:
                    if 'hash' in line.lower() or 'contract' in line.lower():
                        print(f"  🏷️  {line.strip()}")
            else:
                print(f"  ❌ {name} deployment failed")
                if error:
                    print(f"     Error: {error.strip()}")
        else:
            print(f"  ⚠️  {name} NEF file not found: {nef_path}")
    
    print(f"\n📊 Deployment Summary:")
    print(f"  Total contracts attempted: {len(contracts)}")
    print(f"  Successfully deployed: {len(deployed_contracts)}")
    
    if deployed_contracts:
        print("\n✅ Successfully deployed contracts:")
        for name, path in deployed_contracts:
            print(f"  • {name}: {path}")
            
        print("\n📋 Final contract listing...")
        success, output, _ = run_command(f"{neoxp_path} contract list")
        if success:
            print(output)
    
    print("\n🧹 Cleaning up...")
    process.terminate()
    try:
        process.wait(timeout=10)
    except subprocess.TimeoutExpired:
        process.kill()
        process.wait()
    
    print("✅ Deployment test completed!")

if __name__ == "__main__":
    main()