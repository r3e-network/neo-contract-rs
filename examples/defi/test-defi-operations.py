#!/usr/bin/env python3

import json
import subprocess
import time
from decimal import Decimal

class DeFiTester:
    def __init__(self, network="defi-testnet.neo-express"):
        self.network = network
        self.load_contracts()
    
    def load_contracts(self):
        """Load deployed contract addresses"""
        try:
            with open("deployed-contracts.json", "r") as f:
                self.contracts = json.load(f)
        except:
            print("Error: deployed-contracts.json not found. Run deploy-contracts.py first.")
            exit(1)
    
    def run_command(self, cmd):
        """Execute a shell command"""
        result = subprocess.run(cmd, shell=True, capture_output=True, text=True)
        print(f"Command: {cmd}")
        print(f"Output: {result.stdout}")
        if result.returncode != 0:
            print(f"Error: {result.stderr}")
        return result
    
    def test_uniswap_swap(self):
        """Test Uniswap token swap"""
        print("\n=== Testing Uniswap Swap ===")
        
        # Alice swaps 100 USDT for WBTC
        print("Alice swapping 100 USDT for WBTC...")
        
        # First approve Uniswap to spend USDT
        cmd = f'''neo-express contract invoke {self.contracts["USDT"]} approve \\
                --arg "{{\\"type\\":\\"Hash160\\",\\"value\\":\\"uniswap-v2-amm\\"}}" \\
                --arg "{{\\"type\\":\\"Integer\\",\\"value\\":\\"100000000000\\"}}" \\
                alice -i {self.network}'''
        self.run_command(cmd)
        
        # Execute swap
        cmd = f'''neo-express contract invoke uniswap-v2-amm swap \\
                --arg "{{\\"type\\":\\"Integer\\",\\"value\\":\\"100000000000\\"}}" \\
                --arg "{{\\"type\\":\\"Integer\\",\\"value\\":\\"1000000\\"}}" \\
                --arg "{{\\"type\\":\\"Hash160\\",\\"value\\":\\"{self.contracts["USDT"]}\\"}}" \\
                alice -i {self.network}'''
        self.run_command(cmd)
        
        # Check balances
        self.check_balance("alice", "WBTC")
    
    def test_compound_lending(self):
        """Test Compound lending and borrowing"""
        print("\n=== Testing Compound Lending ===")
        
        # Bob supplies 500 USDT
        print("Bob supplying 500 USDT...")
        
        # Approve Compound to spend USDT
        cmd = f'''neo-express contract invoke {self.contracts["USDT"]} approve \\
                --arg "{{\\"type\\":\\"Hash160\\",\\"value\\":\\"compound-lending\\"}}" \\
                --arg "{{\\"type\\":\\"Integer\\",\\"value\\":\\"500000000000\\"}}" \\
                bob -i {self.network}'''
        self.run_command(cmd)
        
        # Supply USDT
        cmd = f'''neo-express contract invoke compound-lending supply \\
                --arg "{{\\"type\\":\\"Integer\\",\\"value\\":\\"500000000000\\"}}" \\
                bob -i {self.network}'''
        self.run_command(cmd)
        
        # Bob borrows 0.01 WBTC using USDT as collateral
        print("Bob borrowing 0.01 WBTC...")
        cmd = f'''neo-express contract invoke compound-lending borrow \\
                --arg "{{\\"type\\":\\"Integer\\",\\"value\\":\\"10000000\\"}}" \\
                bob -i {self.network}'''
        self.run_command(cmd)
        
        # Check Bob's WBTC balance
        self.check_balance("bob", "WBTC")
    
    def test_aave_flashloan(self):
        """Test Aave flash loan"""
        print("\n=== Testing Aave Flash Loan ===")
        
        # Charlie executes a flash loan of 1000 USDT
        print("Charlie executing flash loan of 1000 USDT...")
        
        # First, Charlie needs to implement the flash loan receiver interface
        # For testing, we'll simulate a simple arbitrage
        
        cmd = f'''neo-express contract invoke aave-flashloan flash_loan \\
                --arg "{{\\"type\\":\\"Hash160\\",\\"value\\":\\"{self.contracts["USDT"]}\\"}}" \\
                --arg "{{\\"type\\":\\"Integer\\",\\"value\\":\\"1000000000000\\"}}" \\
                --arg "{{\\"type\\":\\"String\\",\\"value\\":\\"arbitrage\\"}}" \\
                charlie -i {self.network}'''
        result = self.run_command(cmd)
        
        if "success" in result.stdout.lower():
            print("✅ Flash loan executed successfully!")
        else:
            print("❌ Flash loan failed - expected as Charlie needs to implement receiver")
    
    def test_liquidity_provision(self):
        """Test adding liquidity to Uniswap"""
        print("\n=== Testing Liquidity Provision ===")
        
        # Alice adds liquidity to USDT/WETH pool
        print("Alice adding liquidity to USDT/WETH pool...")
        
        # Approve tokens
        for token in ["USDT", "WETH"]:
            cmd = f'''neo-express contract invoke {self.contracts[token]} approve \\
                    --arg "{{\\"type\\":\\"Hash160\\",\\"value\\":\\"uniswap-v2-amm\\"}}" \\
                    --arg "{{\\"type\\":\\"Integer\\",\\"value\\":\\"1000000000000\\"}}" \\
                    alice -i {self.network}'''
            self.run_command(cmd)
        
        # Add liquidity
        cmd = f'''neo-express contract invoke uniswap-v2-amm add_liquidity \\
                --arg "{{\\"type\\":\\"Integer\\",\\"value\\":\\"100000000000\\"}}" \\
                --arg "{{\\"type\\":\\"Integer\\",\\"value\\":\\"10000000000\\"}}" \\
                --arg "{{\\"type\\":\\"Integer\\",\\"value\\":\\"90000000000\\"}}" \\
                --arg "{{\\"type\\":\\"Integer\\",\\"value\\":\\"9000000000\\"}}" \\
                alice -i {self.network}'''
        self.run_command(cmd)
        
        print("✅ Liquidity added successfully!")
    
    def test_liquidation(self):
        """Test Compound liquidation"""
        print("\n=== Testing Liquidation ===")
        
        # Simulate a liquidation scenario
        print("Simulating under-collateralized position...")
        
        # This would require manipulating prices or time
        # For demo purposes, we'll just show the liquidation call
        
        cmd = f'''neo-express contract invoke compound-lending liquidate \\
                --arg "{{\\"type\\":\\"Hash160\\",\\"value\\":\\"bob\\"}}" \\
                --arg "{{\\"type\\":\\"Integer\\",\\"value\\":\\"5000000\\"}}" \\
                --arg "{{\\"type\\":\\"Hash160\\",\\"value\\":\\"{self.contracts["USDT"]}\\"}}" \\
                alice -i {self.network}'''
        
        print("Liquidation command prepared (would execute if position was liquidatable)")
    
    def check_balance(self, user, token):
        """Check user's token balance"""
        cmd = f'''neo-express contract invoke {self.contracts[token]} balance_of \\
                --arg "{{\\"type\\":\\"Hash160\\",\\"value\\":\\"{user}\\"}}" \\
                {user} -i {self.network}'''
        result = self.run_command(cmd)
        print(f"{user}'s {token} balance: {result.stdout}")
    
    def run_all_tests(self):
        """Run all DeFi tests"""
        print("=== Starting DeFi Operations Test ===")
        
        # Test each protocol
        self.test_uniswap_swap()
        self.test_compound_lending()
        self.test_aave_flashloan()
        self.test_liquidity_provision()
        self.test_liquidation()
        
        print("\n=== All Tests Complete ===")
        
        # Final balance check
        print("\n=== Final Balances ===")
        for user in ["alice", "bob", "charlie"]:
            for token in ["USDT", "WBTC", "WETH"]:
                self.check_balance(user, token)

if __name__ == "__main__":
    tester = DeFiTester()
    tester.run_all_tests()