# NEO RPG Game Smart Contract

This is an on-chain role-playing game (RPG) smart contract built on the Neo N3 blockchain using the neo-contract-rs framework. It demonstrates how complex game logic can be implemented in a blockchain environment.

## Features

- **Character Management**: Create and customize characters with different classes and attributes
- **Item System**: Weapons, armor, accessories, consumables, and quest items with stats
- **Inventory Management**: Purchase, equip, and use items
- **Combat System**: Battle monsters with critical hit mechanics and rewards
- **Quest System**: Accept, complete, and track quests with multiple objectives
- **Experience and Leveling**: Gain experience and level up to improve character stats

## Game Mechanics

### Character Classes

- **Warrior**: Strong melee fighter with high strength and vitality
- **Rogue**: Agile fighter with high dexterity and luck
- **Mage**: Spell caster with high intelligence
- **Ranger**: Ranged attacker with high dexterity
- **Cleric**: Support class with balanced stats and healing abilities

### Stats System

Characters have five primary attributes that affect gameplay:
- **Strength**: Affects physical damage and carrying capacity
- **Dexterity**: Affects accuracy, evasion, and critical hit chance
- **Intelligence**: Affects mana pool and magical abilities
- **Vitality**: Affects health points and physical resistance
- **Luck**: Affects critical hit chance and item drop rates

### Combat

The combat system uses a turn-based approach with:
- Damage calculation based on character stats and equipped items
- Critical hit chances based on the character's luck stat
- Monster counter-attacks
- Experience and gold rewards upon victory
- Potential item drops from defeated monsters

### Quests

The quest system supports:
- Multiple quest objectives (kill monsters, collect items, complete other quests)
- Prerequisites for quests
- Experience, gold, and item rewards

## Contract Methods

### Admin Methods

- `add_item`: Create a new item in the game
- `add_quest`: Create a new quest
- `add_monster`: Create a new monster

### Player Methods

- `create_character`: Create a new character
- `equip_item`: Equip an item from inventory
- `use_item`: Use a consumable item
- `start_quest`: Begin a quest
- `update_quest_progress`: Update progress on an active quest
- `start_combat`: Start a battle with a monster
- `attack`: Perform an attack in combat
- `rest`: Restore some health and mana

### View Methods

- `get_character`: View character information
- `get_inventory`: View a character's inventory
- `get_item`: View item details
- `get_quest`: View quest details
- `get_quest_objectives`: View objectives for a quest
- `get_quest_progress`: View progress on a quest
- `get_combat_session`: View current combat status

## Security Considerations

- All state-changing methods verify the caller's signature
- Combat actions have cooldown periods to prevent spam
- Item and quest prerequisites prevent progression exploits
- Critical game logic is protected from reentrancy attacks

## Usage Example

```python
# Python example using neo-python client
from neo3.api import SmartContract

# Contract hash of the deployed RPG game
contract_hash = '0x1234567890abcdef1234567890abcdef12345678'
rpg_contract = SmartContract(contract_hash)

# Create a new character
wallet.sign_transaction(
    rpg_contract.create_character(
        name="Adventurer",
        class_id=0  # 0=Warrior, 1=Rogue, 2=Mage, 3=Ranger, 4=Cleric
    )
)

# Start combat with a monster
wallet.sign_transaction(
    rpg_contract.start_combat(monster_id=1)
)

# Attack the monster
wallet.sign_transaction(
    rpg_contract.attack()
)
```

## License

This code is provided as an example and is licensed under MIT License.