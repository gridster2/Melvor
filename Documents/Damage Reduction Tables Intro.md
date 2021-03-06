# Damage Reduction Tables for Dungeons

Melvor Idle v0.19.2, /u/gridster2

This is intended as an appendix to my [Compendium of Damage Reduction](https://www.reddit.com/r/MelvorIdle/comments/mk7zt3/compendium_of_damage_reduction/?).

%%insertminiintro%%

### Introduction

The following charts give the damage reduction required to safely idle a dungeon with a given number of hitpoints using a given combat style, in both normal and [Hardcore](https://wiki.melvoridle.com/index.php?title=Hardcore) mode. For example, a player with 550 hitpoints, a magic combat style, and Auto Eat Tier III can safely idle Dragons Den with a damage reduction of 43. The [Wasteful Ring](https://wiki.melvoridle.com/index.php?title=Wasteful_Ring) (which increases the auto-eat threshold) adds a bonus of 5% (additive) to the Auto Eat threshold (for [Auto Eat III](https://wiki.melvoridle.com/index.php?title=Auto_Eat_-_Tier_III), this means a threshold of 45%), making it an important item to include in calculations.

The column headings use emoji to display combat style, difficulty, and equipment (excuse the pictographs; it's much more readable than long text). The ❤️ column gives the player's hitpoints (including bonuses). ⚔️ indicates that the player is using Melee, 🏹 indicates Ranged, 🧙 indicates Magic. ☠️ indicates Hardcore mode. 💍 indicates that the [Wasteful Ring](https://wiki.melvoridle.com/index.php?title=Wasteful_Ring) is equipped, adding 5% to the Auto Eat threshold.

Values in these charts crossed out and in italic mean that the given damage reduction is beyond the practical limit of 81%, and likely can't be reached. If the hitpoints "skips" a row (for example, the row for 890 hitpoints is followed by 910), then it is because the middle value has the same values as its predecessors (e.g. the damage reductions for 900 hitpoints is the same as for 890). The rows start at either 100 hitpoints or the point at which damage reduction values are reachable, and end at 1160 hitpoints, or when damage reduction values for all combat styles are 0.

### Note on "Into the Mist"

[Into the Mist](https://wiki.melvoridle.com/index.php?title=Into_the_Mist) is a unique dungeon, in that it needs only ever to be completed five times, it can spawn any enemy whose combat level is between 165 and 677 (inclusive), and the final three monsters each can only be damaged by a single type of damage. In addition, the player's health can potentially be capped at 50% max hitpoints. It included in this set of tables for completeness's sake (omitted on Reddit, due to length limits), and split into the warmup (the 20 randomized monsters), and the boss (the final three enemies); right now, none of the information provided is very useful, but further changes to the game may affect this. The table for the boss part of Into the Mist gives the damage reduction for each combat style; that is, this fight requires switching between three equipment sets, each with the given damage reduction. The max hit for each category _does_ take into account all of the possible randomized enemy spawns. Because of the potential hitpoint cap, the damage reduction penalty, and the variety of high-level multi-combat-style enemies, [Into the Mist](https://wiki.melvoridle.com/index.php?title=Into_the_Mist) cannot safely be idled.

### Max Hits of Dungeon Monsters

As of v0.18.0, certain enemies can affect the player's maximum hitpoints and their damage reduction. This makes it important to consider not just the max hit of the dungeon (as had been done for previous versions), but the specific monster dealing the hit. In [Into the Mist](https://wiki.melvoridle.com/index.php?title=Into_the_Mist), the player can potentially be "Afflicted", reducing maximum HP to 50%. The boss's first phase, [Mysterious Figure](https://wiki.melvoridle.com/index.php?title=Mysterious_Figure_-_Phase_1), can apply Mark of Death to the player, immediately halving their damage reduction. The below table lists the max hit of each monster in each dungeon, taking into account special attacks, and the potential for special attacks to deal extra damage (i.e. when the player is stunned).
