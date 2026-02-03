# Brainstorm for Minimum Playable Product for the Q Game

## Systems at Play

Systems at play are going to be:

- Movement/terrain
- Crafting
- Combat

Those are already done, but those two together don't really provide much gameplay. So combat is like the basic combat is done, so it'll basically be movement, crafting, and combat.

## Core Concept

I think a good way to do this would be to have a small fixed area, maybe a 3x3 or 5x5 area, and then have the movement-combat interaction system. This isn't implemented yet, and I'd like to get to an MVP type of thing before that's implemented.

So, what we'll do is have combat be a gateway and an obstacle to entering a new land tile. You can't enter a land tile until you've defeated the combat for it. Let's say we have a 5x5 area, and some of the lands have an icon over them that show enemies in those lands. You have to beat the enemies before you can take those lands. You start in the top-left, and the bottom-right is the final boss. Your goal is to get strong enough to beat the final boss.

The primary activity you'll do to progress towards the final boss is going to be crafting. So you'll basically have to get top-tier gear in order to be strong enough to beat the boss. The task for designing the MVP becomes coming up with the content and the progression. So coming up with the enemies and the crafting. The other order of business is actually integrating the crafting system into the overall game with the movement system and integrating combat. It should be easy because it'll just be a layer on top of it. We can keep combat fairly separate.

## Key Features

### Workstation Placement

Oh, and then another thing that's gonna be necessary is placement of workstations. The way we'll do it is that if you're within a land that has that workstation, then it's available to you for crafting. You don't have to actually go and walk over it.

### Crafting GUI

Another thing that's probably going to take a bit of work is the GUI for crafting. To keep things simple for now, we'll probably just have a list of all crafts that are possible given the current inventory.

### Inventory System

Oh, that's another thing we need to add. We do need to have an inventory system. The inventory system will remain simple for now and just say no stacking.

### Equipment System

And we need to have stats on items and make them equippable. There has to be a UI for that as well, for equipping items. We'll definitely do all this as a CLI first.

## Implementation Plan

We'll make another directory and then in that directory we'll do a new Cargo project. We'll kind of build from scratch but pulling in the pieces from the subsystems, and we'll have a 5x5 area. In each of those tiles, a few will be accessible right away, and then each tile is going to have something unique about it that allows you to progress in your crafting career.

## Steps

### Step One: Copy Movement/Terrain System

Copy a bunch of stuff from the main game minus generation to get a 5x5 land area.

- We'll know we're done with this when we can through the CLI move to different lands, and then go into the lands, and then move within the lands.
- Should have a working and tested REPL here. The REPL should be as easy to use as possible for a human. So that means short versions of commands, ideally one letter when possible. The letters U, D, L, and R will be used for going up, down, left, right, both in terrain view and in land view.
- We want the REPL to work such that the screen clears and reprints so that you know you might have a history log of your commands on one side of the screen. But then you want to actually display each time you take a step, you want to show that you're in the next land. So actually, maybe we just do a web view so that we actually have some graphics, but then keep it REPL style in the way that the way you interact with the game is strictly through typing commands into a little command box.

### Step Two: Add Combat Obstacle

Next, we want to add the combat obstacle on top of it.

- This will mean that when we drill down into a land, we first enter a combat sequence. In the CLI, at each turn, you'll have to take an action. You can either attack or flee, so that'll be like 'a' or 'f'.
- For now, when you flee combat, all parties are restored to full health.
- We will have to decide at some point whether enemies blocking lands stay dead, or if each new time you enter the land they come back. Or we could do a time-based system/turn-based system.
- Requires adding stats into the game. So a personal character with stats, and then, of course, the enemies and their stats.

### Step Three: Add Crafting

Then we want to add crafting. This will probably be a multi-step process, so the first step in adding crafting will be to add gathering and an inventory system. The way gathering will work is items that are on the ground whose pickup requirement is none. When you're on the same tile as that item, and you use the P command, you'll pick it up, and it will enter your inventory. The inventory is just going to be an infinite list of single items, or not. I mean, it will be not infinite but it will have infinite capacity.

Right after doing Gathering, it will be fairly simple to implement mob drops from After Combat.

- **Stretch goal**: instead of having combat just deposit mob drop items directly into your inventory, we will put a carcass of that mob into the land, and then you go and interact with that using say a skinning knife to get leather off it or other implements to process the carcass.

Once that's done, then we'll implement actually crafting. The way that'll work is when you open your inventory on your left, you'll have a top-to-bottom list of all the items you have. On the right, you'll have a top-to-bottom list of all the items that are currently craftable. Currently craftable means you have the materials in your inventory and the necessary workstations, if any, are in the current land.

- You will not be able to craft unless you're in a land, in the land view.

Once that basic crafting is done, we will then implement object placement so that we can place down workstations. The way that will work is we'll have some command to place an object, and it'll take the argument of the thing you want to set down. When you set down a workstation, that now enables you to craft with it.

### Step Four: Add Equipment System

Next after that, we'll add in the ability to equip gear. and the gear affects your stats.

- **Stretch goal**: gear can grant more combat options besides attack and flee.

### Step Five: Add Content and Progression

And then the last step will be to add the content that lays out the progression system. The progression system being all of the raw materials, intermediates, workstations, creatures and drops that ultimately lead to the final gear that allows you to defeat the Boss.

## Notes

- For now, if you die in the game, you just basically get kicked out of combat. That's basically the same as fleeing.
