#!/bin/bash
# Quick demo script for the Crafting & Combat System CLI

echo "Running demo script..."
echo ""

cargo run -- --human-readable <<EOF
# Demo: Exploring the system
list recipes
list items

# Demo: Creating items
new copper_ore
new tin_ore
inventory

# Demo: Combat system
combat 10 5 8 3
combat-round 20 3 15 2
combat 5 5 5 5

# Exit
exit
EOF
