# Compendium of Damage Reduction

Melvor Idle v0.19.2, /u/gridster2

* Introduction
* Damage Reduction and Idle Combat
* Combat Triangle
* Idling Dungeons
* Sources of Damage Reduction
    * Passive Items
    * Agility
    * Pets
    * Prayer
    * Potions
    * Capes
    * Rings
    * Amulets
    * Gloves
    * Hats
    * Shields
    * Equipment Sets
* Maximum Damage Reduction
* Loadouts
* Closing

## Introduction

This guide best used alongside [Damage Reduction Tables for Dungeons](https://www.reddit.com/r/MelvorIdle/comments/kdzha0/damage_reduction_tables_for_dungeons/?).

[Damage Reduction](https://wiki.melvoridle.com/index.php?title=Combat#Combat_Info) is the most important combat modifier in the game, and the most difficult to optimize for. The [Melvor Idle Wiki](https://wiki.melvoridle.com/index.php?title=Main_Page) does not have much information about damage reduction, and determining how to maximize reduction requires hopping back and forth through several pages. Determining hitpoint / damage reduction breakpoints for dungeons is also difficult. This compendium will hopefully improve this by listing all the relevant information on a single page, and explain how conclusions were drawn.

For information about the damage reduction required to safely idle specific dungeons, refer to the [Damage Reduction Tables for Dungeons](https://www.reddit.com/r/MelvorIdle/comments/kdzha0/damage_reduction_tables_for_dungeons/?). Scroll to the bottom of this post to see calculations on maximum damage reduction.

## Damage Reduction and Idle Combat

Whether an enemy or dungeon can be idled is decided by four factors: the enemy's max hit, the purchased level of [Auto Eat](https://wiki.melvoridle.com/index.php?title=Shop#Auto_Eat), the player's maximum [Hitpoints](https://wiki.melvoridle.com/index.php?title=Hitpoints), and of course the player's [damage reduction](https://wiki.melvoridle.com/index.php?title=Combat#Combat_Info). As long as the player has enough food to heal, the only attacks that can kill are ones that hit hard enough to bypass auto eat entirely. Thus, an enemy can safely be idled as long as the following is true:

*(Player HP)* × *(Auto Eat Threshold)* > *(Max Hit)* × (1 - *(Damage Reduction)*)

For example, a player with 500 HP, 10% damage reduction, and [Auto Eat II](https://wiki.melvoridle.com/index.php?title=Auto_Eat_-_Tier_II) (threshold 30%) could safely idle an enemy with a max hit of 166, but not 167. To idle the [Air God Dungeon](https://wiki.melvoridle.com/index.php?title=Air_God_Dungeon), the player must be able to survive [Voltaire's](https://wiki.melvoridle.com/index.php?title=Voltaire) max hit of 713; this could be done with 900 HP, [Auto Eat III](https://wiki.melvoridle.com/index.php?title=Auto_Eat_-_Tier_III), and 50% damage reduction. This is what makes damage reduction so important. [Evasion](https://wiki.melvoridle.com/index.php?title=Combat#Combat_Info) or [Accuracy](https://wiki.melvoridle.com/index.php?title=Combat#Combat_Info) are helpful for minimizing the amount of food needed, but the deciding factor is how big a hit the player can survive.

Obtaining [Finn the Cat](https://wiki.melvoridle.com/index.php?title=Finn,_the_Cat) as a pet gives a bonus 10 hitpoints, which should be taken into account. The [Dragonfire Shield](https://wiki.melvoridle.com/index.php?title=Dragonfire_Shield) gives an additional 30 hitpoints. A potential 130 hitpoints can be gained from [Agility](https://wiki.melvoridle.com/w/Agility) obstacles (see the section on Agility for details). This gives a maximum of 1160 hitpoints. The [Wasteful Ring](https://wiki.melvoridle.com/index.php?title=Wasteful_Ring) adds 5% to the Auto Eat threshold (for [Auto Eat III](https://wiki.melvoridle.com/index.php?title=Auto_Eat_-_Tier_III), this means a threshold of 45%); at high HP levels, this weighs so heavily on the idling calculation as to make it the most useful ring-slot item, being worth potentially several DR points.

## Combat Triangle

The [Combat Triangle](https://wiki.melvoridle.com/index.php?title=Combat_Triangle#Bonuses) modifies damage reduction according to the player's combat style's strength against the enemy's style. This is important to pay attention to, as using the wrong style can potentially halve the effective damage reduction, and using the superior style can give a significant bonus. [Ranged](https://wiki.melvoridle.com/index.php?title=Ranged) has an advantage, as it has the least penalty to damage reduction when used against the wrong style. If the player is using the [Hardcore](https://wiki.melvoridle.com/index.php?title=Hardcore) game mode, the penalties for using a weak style increase. The multipliers to damage reduction are given below:

|Player Style|DR vs Melee|DR vs Ranged|DR vs Magic|
|---|---|---|---|
|Melee|1.0×|1.25×|**0.5×**|
|Ranged|**0.95×**|1.0×|1.25×|
|Magic|1.25×|**0.85×**|1.0×|
|Melee (HC)|1.0×|1.25×|**0.25×**|
|Ranged (HC)|**0.75×**|1.0×|1.25×|
|Magic (HC)|1.25×|**0.75×**|1.0×|

## Idling Dungeons

[Dungeons](https://wiki.melvoridle.com/index.php?title=Dungeons) present a new issue for idling combat. The player encounters multiple combat styles, and thus must consider the  [Combat Triangle](https://wiki.melvoridle.com/index.php?title=Combat_Triangle#Bonuses) when determining the required damage reduction. A dungeon can be safely idled so long as the following is true for each type combat style in the dungeon:

*(Player HP)* × *(Auto Eat Threshold)* > *(Max Hit)* × (1 - (*(Damage Reduction)* × *(Damage Reduction Modifier)*))

As of v0.18.0, certain enemies can affect the player's maximum hitpoints and their damage reduction. This makes it important to consider not just the max hit of the dungeon (as had been done for previous versions), but the specific monster dealing the hit. It is also important to consider the damage dealt by special attacks, as some special attacks deal extra damage when the player is stunned.

The [Damage Reduction Tables for Dungeons](https://www.reddit.com/r/MelvorIdle/comments/kdzha0/damage_reduction_tables_for_dungeons/?), giving the damage reduction required for a given combat style and number of hitpoints, can be referenced for details. It is not included in this compendium due to Reddit's character limit.

## Sources of Damage Reduction

Damage reduction can come from [pets](https://wiki.melvoridle.com/index.php?title=Pets), [prayer](https://wiki.melvoridle.com/index.php?title=Prayer#Prayers), [potions](https://wiki.melvoridle.com/index.php?title=Herblore#Combat_Potions), and [equipment](https://wiki.melvoridle.com/index.php?title=Equipment).

### Passive Items

After completing [Into the Mist](https://wiki.melvoridle.com/index.php?title=Into_the_Mist), the player can make use of the Passive equipment slot. This is one space that only certain items can be equipped to. Only the [Guardian Amulet](https://wiki.melvoridle.com/index.php?title=Guardian_Amulet) provides damage reduction while in the passive slot (both it base 5%, and the extra 5% when under 50% max hitpoints). The [Dragonfire Shield](https://wiki.melvoridle.com/index.php?title=Dragonfire_Shield) and [Wasteful Ring](https://wiki.melvoridle.com/index.php?title=Wasteful_Ring) can provide their special effects from the passive slot.

|Item|Reduction|Required Level|Combat Style|
|---|---|---|---|
|[Guardian Amulet](https://wiki.melvoridle.com/index.php?title=Guardian_Amulet)|5%|*None*|*None*|

### Agility

Only one [Agility](https://wiki.melvoridle.com/w/Agility) obstacle increases damage reduction:

|Slot|Name|Reduction|
|---|---|---|
|6|Lake Swim|1%|

However, this means not using Rocky Waters (+50 HP) in Slot 6, which at all levels increases the survivable max hit more than the 1% damage reduction bonus. When trying to maximize the survivable hit, Rocky Waters is always better than Lake Swim.

The maximum HP bonus from obstacles is 130, however, this comes at the cost of -4% DR. Mud Dive, the Slot 4 obstacle, trades damage reduction for HP (-4% DR, +20 HP). If the player has less than 520 hitpoints (including other bonuses), Mud Dive actually increases the max survivable hit. If Mud Dive is mastered (halving the negative effect), it increases the maximum survivable hit for players with fewer than 1020 hitpoints. After passing the threshold (520 or 1020), Mud Dive only decreases the max survivable hit, and should be avoided.

Thus, the maximum useful HP bonus from obstacles is 110, considering that most players min-maxing their max survivable hit will be above the Mud Dive threshold.

Agility bonuses/nerfs to auto-eat efficiency affect only the amount healed, not the threshold. This affects only the player's consumption of food, and not the max survivable hit.

### Pets

Only two [pets](https://wiki.melvoridle.com/index.php?title=Pets) grant damage reduction.

|Name|Reduction|Source|
|---|---|---|
|[Leonardo](https://wiki.melvoridle.com/index.php?title=Leonardo)|1%|Defence|
|[Erran](https://wiki.melvoridle.com/index.php?title=Erran)|1%|Defence|

### Prayer

At level 88 [Prayer](https://wiki.melvoridle.com/index.php?title=Prayer), [Stone Skin](https://wiki.melvoridle.com/index.php?title=Stone_Skin) can be activated. It costs 3 prayer points per enemy attack. This is the only prayer that impacts damage reduction.

|Prayer|Reduction|Cost|
|---|---|---|
|[Stone Skin](https://wiki.melvoridle.com/index.php?title=Stone_Skin)|3%|2|

### Potions

[Damage reduction potions](https://wiki.melvoridle.com/index.php?title=Damage_Reduction_Potion_I) can be created at [Herblore](https://wiki.melvoridle.com/index.php?title=Herblore) level 90. They require two Barrentoe Herbs, two Eyeballs, and one Large Horn. One charge of the potion is used per enemy attack.

|Potion|Reduction|Charges|Mastery Level|
|---|---|---|---|
|[Damage Reduction Potion I](https://wiki.melvoridle.com/index.php?title=Damage_Reduction_Potion_I)|2%|10|1|
|[Damage Reduction Potion II](https://wiki.melvoridle.com/index.php?title=Damage_Reduction_Potion_II)|4%|15|20|
|[Damage Reduction Potion III](https://wiki.melvoridle.com/index.php?title=Damage_Reduction_Potion_III)|6%|20|50|
|[Damage Reduction Potion IV](https://wiki.melvoridle.com/index.php?title=Damage_Reduction_Potion_IV)|10%|30|90|

### Capes

The [Cape of Completion](https://wiki.melvoridle.com/index.php?title=Cape_of_Completion) has the best damage reduction of any cape, but requires completing the entire game before it can be purchased.

|Item|Reduction|Required Level|Combat Style|
|---|---|---|---|
|[Fire Cape](https://wiki.melvoridle.com/index.php?title=Fire_Cape)|2%|*None*|*None*|
|[Skull Cape](https://wiki.melvoridle.com/index.php?title=Skull_Cape)|3%|*None*|Magic|
|[Infernal Cape](https://wiki.melvoridle.com/index.php?title=Infernal_Cape)|4%|*None*|*None*|
|[Cape of Completion](https://wiki.melvoridle.com/index.php?title=Cape_of_Completion)|5%|*None*|*None*|

### Rings

|Item|Reduction|Required Level|Combat Style|
|---|---|---|---|
|[Silver Diamond Ring](https://wiki.melvoridle.com/index.php?title=Silver_Diamond_Ring)|1%|*None*|*None*|
|[Guardian Ring](https://wiki.melvoridle.com/index.php?title=Guardian_Ring)|2%|*None*|*None*|

### Amulets

The [Guardian Amulet](https://wiki.melvoridle.com/index.php?title=Guardian_Amulet) is the best damage reduction amulet, with a base 5% damage reduction. At under 50% HP, it provides 10% reduction; in special circumstances, this means can allow completion of a dungeon at lower damage reduction levels than normal, although such a situation would be so specific as to not be worth calculating. The same item cannot be equipped both in the passive and an active slot, and so stacking multiple bonuses from one item is not possible.

|Item|Reduction|Required Level|Combat Style|
|---|---|---|---|
|[Elite Amulet of Ranged](https://wiki.melvoridle.com/index.php?title=Elite_Amulet_of_Ranged)|1%|*None*|Ranged|
|[Elite Amulet of Defence](https://wiki.melvoridle.com/index.php?title=Elite_Amulet_of_Defence)|2%|*None*|Melee|
|[Elite Amulet of Magic](https://wiki.melvoridle.com/index.php?title=Elite_Amulet_of_Magic)|2%|*None*|Magic|
|[Silver Diamond Necklace](https://wiki.melvoridle.com/index.php?title=Silver_Diamond_Necklace)|2%|*None*|*None*|
|[Fury of the Elemental Zodiacs](https://wiki.melvoridle.com/index.php?title=Fury_of_the_Elemental_Zodiacs)|2%|*None*|*None*|
|[Guardian Amulet](https://wiki.melvoridle.com/index.php?title=Guardian_Amulet)|5%|*None*|*None*|

### Gloves

Other gloves are listed as part of Equipment Sets (see below). There are only two pairs of gloves that don't belong to a larger equipment set, and also provide damage reduction. [(U) Red D-hide Vambraces](https://wiki.melvoridle.com/index.php?title=%28U%29_Red_D-hide_Vambraces) are superior to both of these with 5% reduction, but require 70 ranged and do not provide the same attack bonuses.

|Item|Reduction|Required Level|Combat Style|
|---|---|---|---|
|[Desert Wrappings](https://wiki.melvoridle.com/index.php?title=Desert_Wrappings)|2%|*None*|*None*|
|[Paladin Gloves](https://wiki.melvoridle.com/index.php?title=Paladin_Gloves)|4%|*None*|Melee|

### Hats

The three party hats each provide a 1% damage reduction, allowing the player to stack Bragging Rights onto a damage-reduction build.

|Item|Reduction|Required Level|Combat Style|
|---|---|---|---|
|[Red Party Hat](https://wiki.melvoridle.com/index.php?title=Red_Party_Hat)|1%|*None*|*None*|
|[Green Party Hat](https://wiki.melvoridle.com/index.php?title=Green_Party_Hat)|1%|*None*|*None*|
|[Purple Party Hat](https://wiki.melvoridle.com/index.php?title=Purple_Party_Hat)|1%|*None*|*None*|

### Shields

Other shields are listed as part of Equipment Sets (see below).

|Item|Reduction|Required Level|Combat Style|
|---|---|---|---|
|[Recoil Shield](https://wiki.melvoridle.com/index.php?title=Recoil_Shield)|6%|*None*|Melee|
|[Dragonfire Shield](https://wiki.melvoridle.com/index.php?title=Dragonfire_Shield)|8%|*None*|Melee|
|[Earth Layered Shield](https://wiki.melvoridle.com/index.php?title=Earth_Layered_Shield)|12%|*None*|Melee|

### Equipment Sets

Most damage-reducing equipment is part of a set. For most sets, the included items have identical level requirements and damage reduction. It often makes sense to mix-and-match parts of sets, especially when one set doesn't cover all equipment slots (for example, wearing [dragon boots](https://wiki.melvoridle.com/index.php?title=%28G%29_Dragon_Boots) to fill out the ancient melee set). This matters particularly for gloves; aside from god dungeon loot, the only damage-reducing gloves are (U) Vambraces, which have no penalties to non-ranged attacks. This makes them the best in slot item for damage reduction pre-God Dungeons. See the entries for the individual items for details on obtaining them.

|Set|Number of Items|Reduction per Item|Total Reduction|Required Level|Combat Style|Items|
|---|---|---|---|---|---|---|
|Slayer (Elite) (Melee)|2|4%|8%|60 [Defence](https://wiki.melvoridle.com/index.php?title=Defence)|Melee|[Helmet](https://wiki.melvoridle.com/index.php?title=Slayer_Helmet_%28Elite%29), [Platebody](https://wiki.melvoridle.com/index.php?title=Slayer_Platebody_%28Elite%29)|
|Slayer (Master) (Melee)|2|6%|12%|80 [Defence](https://wiki.melvoridle.com/index.php?title=Defence)|Melee|[Helmet](https://wiki.melvoridle.com/index.php?title=Slayer_Helmet_%28Master%29), [Platebody](https://wiki.melvoridle.com/index.php?title=Slayer_Platebody_%28Master%29)|
|Slayer (Master) (Ranged)|2|6%|12%|80 [Ranged](https://wiki.melvoridle.com/index.php?title=Ranged)|Ranged|[Cowl](https://wiki.melvoridle.com/index.php?title=Slayer_Cowl_%28Master%29), [Leather Body](https://wiki.melvoridle.com/index.php?title=Slayer_Leather_Body_%28Master%29)|
|Slayer (Master) (Magic)|2|6%|12%|80 [Magic](https://wiki.melvoridle.com/index.php?title=Magic)|Magic|[Wizard Hat](https://wiki.melvoridle.com/index.php?title=Slayer_Wizard_Hat_%28Master%29), [Wizard Robes](https://wiki.melvoridle.com/index.php?title=Slayer_Wizard_Robes_%28Master%29)|
|(G) Steel|5|1%|5%|5 [Defence](https://wiki.melvoridle.com/index.php?title=Defence)|Melee|[Helmet](https://wiki.melvoridle.com/index.php?title=%28G%29_Steel_Helmet), [Platebody](https://wiki.melvoridle.com/index.php?title=%28G%29_Steel_Platebody), [Platelegs](https://wiki.melvoridle.com/index.php?title=%28G%29_Steel_Platelegs), [Boots](https://wiki.melvoridle.com/index.php?title=%28G%29_Steel_Boots), [Shield](https://wiki.melvoridle.com/index.php?title=%28G%29_Steel_Shield)|
|(G) Black|5|2%|10%|10 [Defence](https://wiki.melvoridle.com/index.php?title=Defence)|Melee|[Helmet](https://wiki.melvoridle.com/index.php?title=%28G%29_Black_Helmet), [Platebody](https://wiki.melvoridle.com/index.php?title=%28G%29_Black_Platebody), [Platelegs](https://wiki.melvoridle.com/index.php?title=%28G%29_Black_Platelegs), [Boots](https://wiki.melvoridle.com/index.php?title=%28G%29_Black_Boots), [Shield](https://wiki.melvoridle.com/index.php?title=%28G%29_Black_Shield)|
|(G) Mithril|5|3%|15%|20 [Defence](https://wiki.melvoridle.com/index.php?title=Defence)|Melee|[Helmet](https://wiki.melvoridle.com/index.php?title=%28G%29_Mithril_Helmet), [Platebody](https://wiki.melvoridle.com/index.php?title=%28G%29_Mithril_Platebody), [Platelegs](https://wiki.melvoridle.com/index.php?title=%28G%29_Mithril_Platelegs), [Boots](https://wiki.melvoridle.com/index.php?title=%28G%29_Mithril_Boots), [Shield](https://wiki.melvoridle.com/index.php?title=%28G%29_Mithril_Shield)|
|(G) Adamant|5|4%|20%|30 [Defence](https://wiki.melvoridle.com/index.php?title=Defence)|Melee|[Helmet](https://wiki.melvoridle.com/index.php?title=%28G%29_Adamant_Helmet), [Platebody](https://wiki.melvoridle.com/index.php?title=%28G%29_Adamant_Platebody), [Platelegs](https://wiki.melvoridle.com/index.php?title=%28G%29_Adamant_Platelegs), [Boots](https://wiki.melvoridle.com/index.php?title=%28G%29_Adamant_Boots), [Shield](https://wiki.melvoridle.com/index.php?title=%28G%29_Adamant_Shield)|
|(G) Rune|5|5%|25%|40 [Defence](https://wiki.melvoridle.com/index.php?title=Defence)|Melee|[Helmet](https://wiki.melvoridle.com/index.php?title=%28G%29_Rune_Helmet), [Platebody](https://wiki.melvoridle.com/index.php?title=%28G%29_Rune_Platebody), [Platelegs](https://wiki.melvoridle.com/index.php?title=%28G%29_Rune_Platelegs), [Boots](https://wiki.melvoridle.com/index.php?title=%28G%29_Rune_Boots), [Shield](https://wiki.melvoridle.com/index.php?title=%28G%29_Rune_Shield)|
|(G) Dragon|5|6%|30%|60 [Defence](https://wiki.melvoridle.com/index.php?title=Defence)|Melee|[Helmet](https://wiki.melvoridle.com/index.php?title=%28G%29_Dragon_Helmet), [Platebody](https://wiki.melvoridle.com/index.php?title=%28G%29_Dragon_Platebody), [Platelegs](https://wiki.melvoridle.com/index.php?title=%28G%29_Dragon_Platelegs), [Boots](https://wiki.melvoridle.com/index.php?title=%28G%29_Dragon_Boots), [Shield](https://wiki.melvoridle.com/index.php?title=%28G%29_Dragon_Shield)|
|(G) Ancient|4|7%|28%|70 [Defence](https://wiki.melvoridle.com/index.php?title=Defence)|Melee|[Helmet](https://wiki.melvoridle.com/index.php?title=%28G%29_Ancient_Helmet), [Platebody](https://wiki.melvoridle.com/index.php?title=%28G%29_Ancient_Platebody), [Platelegs](https://wiki.melvoridle.com/index.php?title=%28G%29_Ancient_Platelegs), [Shield](https://wiki.melvoridle.com/index.php?title=%28G%29_Ancient_Shield)|
|Ragnar God|5|7%|35%|85 [Defence](https://wiki.melvoridle.com/index.php?title=Defence)|Melee|[Helmet](https://wiki.melvoridle.com/index.php?title=Ragnar_God_Helmet), [Platebody](https://wiki.melvoridle.com/index.php?title=Ragnar_God_Platebody), [Platelegs](https://wiki.melvoridle.com/index.php?title=Ragnar_God_Platelegs), [Boots](https://wiki.melvoridle.com/index.php?title=Ragnar_God_Boots), [Gloves](https://wiki.melvoridle.com/index.php?title=Ragnar_God_Gloves)|
|Terran God|5|8%|40%|85 [Defence](https://wiki.melvoridle.com/index.php?title=Defence)|Melee|[Helmet](https://wiki.melvoridle.com/index.php?title=Terran_God_Helmet), [Platebody](https://wiki.melvoridle.com/index.php?title=Terran_God_Platebody), [Platelegs](https://wiki.melvoridle.com/index.php?title=Terran_God_Platelegs), [Boots](https://wiki.melvoridle.com/index.php?title=Terran_God_Boots), [Gloves](https://wiki.melvoridle.com/index.php?title=Terran_God_Gloves)|
|Ice|5|2%|10%|40 [Defence](https://wiki.melvoridle.com/index.php?title=Defence), 40 [Ranged](https://wiki.melvoridle.com/index.php?title=Ranged)|Ranged|[Helmet](https://wiki.melvoridle.com/index.php?title=Ice_Helmet), [Platebody](https://wiki.melvoridle.com/index.php?title=Ice_Platebody), [Platelegs](https://wiki.melvoridle.com/index.php?title=Ice_Platelegs), [Boots](https://wiki.melvoridle.com/index.php?title=Ice_Boots), [Shield](https://wiki.melvoridle.com/index.php?title=Ice_Shield)|
|Miolite|5|10%|7%|40 [Defence](https://wiki.melvoridle.com/index.php?title=Defence), 40 [Ranged](https://wiki.melvoridle.com/index.php?title=Ranged)|Ranged|[Helmet](https://wiki.melvoridle.com/index.php?title=Miolite_Helmet), [Platebody](https://wiki.melvoridle.com/index.php?title=Miolite_Platebody), [Platelegs](https://wiki.melvoridle.com/index.php?title=Miolite_Platelegs), [Boots](https://wiki.melvoridle.com/index.php?title=Miolite_Boots), [Shield](https://wiki.melvoridle.com/index.php?title=Miolite_Shield)|
|(U) Green D-hide|4|2%|8%|40 [Ranged](https://wiki.melvoridle.com/index.php?title=Ranged)|Ranged|[Body](https://wiki.melvoridle.com/index.php?title=%28U%29_Green_D-hide_Body), [Chaps](https://wiki.melvoridle.com/index.php?title=%28U%29_Green_D-hide_Chaps), [Vambraces](https://wiki.melvoridle.com/index.php?title=%28U%29_Green_D-hide_Vambraces), [Shield](https://wiki.melvoridle.com/index.php?title=%28U%29_Green_D-hide_Shield)|
|(U) Blue D-hide|4|3%|12%|50 [Ranged](https://wiki.melvoridle.com/index.php?title=Ranged)|Ranged|[Body](https://wiki.melvoridle.com/index.php?title=%28U%29_Blue_D-hide_Body), [Chaps](https://wiki.melvoridle.com/index.php?title=%28U%29_Blue_D-hide_Chaps), [Vambraces](https://wiki.melvoridle.com/index.php?title=%28U%29_Blue_D-hide_Vambraces), [Shield](https://wiki.melvoridle.com/index.php?title=%28U%29_Blue_D-hide_Shield)|
|(U) Red D-hide|4|4%|16%|60 [Ranged](https://wiki.melvoridle.com/index.php?title=Ranged)|Ranged|[Body](https://wiki.melvoridle.com/index.php?title=%28U%29_Red_D-hide_Body), [Chaps](https://wiki.melvoridle.com/index.php?title=%28U%29_Red_D-hide_Chaps), [Vambraces](https://wiki.melvoridle.com/index.php?title=%28U%29_Red_D-hide_Vambraces), [Shield](https://wiki.melvoridle.com/index.php?title=%28U%29_Red_D-hide_Shield)|
|(U) Black D-hide|4|5%|20%|70 [Ranged](https://wiki.melvoridle.com/index.php?title=Ranged)|Ranged|[Body](https://wiki.melvoridle.com/index.php?title=%28U%29_Black_D-hide_Body), [Chaps](https://wiki.melvoridle.com/index.php?title=%28U%29_Black_D-hide_Chaps), [Vambraces](https://wiki.melvoridle.com/index.php?title=%28U%29_Black_D-hide_Vambraces), [Shield](https://wiki.melvoridle.com/index.php?title=%28U%29_Black_D-hide_Shield)|
|(U) Ancient D-hide|4|6%|24%|80 [Ranged](https://wiki.melvoridle.com/index.php?title=Ranged)|Ranged|[Body](https://wiki.melvoridle.com/index.php?title=%28U%29_Ancient_D-hide_Body), [Chaps](https://wiki.melvoridle.com/index.php?title=%28U%29_Ancient_D-hide_Chaps), [Vambraces](https://wiki.melvoridle.com/index.php?title=%28U%29_Ancient_D-hide_Vambraces), [Shield](https://wiki.melvoridle.com/index.php?title=%28U%29_Ancient_D-hide_Shield)|
|Aeris God|5|8%|40%|85 [Ranged](https://wiki.melvoridle.com/index.php?title=Ranged)|Ranged|[Helmet](https://wiki.melvoridle.com/index.php?title=Aeris_God_Helmet), [Platebody](https://wiki.melvoridle.com/index.php?title=Aeris_God_Platebody), [Platelegs](https://wiki.melvoridle.com/index.php?title=Aeris_God_Platelegs), [Boots](https://wiki.melvoridle.com/index.php?title=Aeris_God_Boots), [Gloves](https://wiki.melvoridle.com/index.php?title=Aeris_God_Gloves)|
|Air Adept|4|4%|16%|35 [Magic](https://wiki.melvoridle.com/index.php?title=Magic)|Magic|[Hat](https://wiki.melvoridle.com/index.php?title=Air_Adept_Wizard_Hat), [Robes](https://wiki.melvoridle.com/index.php?title=Air_Adept_Wizard_Robes), [Bottoms](https://wiki.melvoridle.com/index.php?title=Air_Adept_Wizard_Bottoms), [Boots](https://wiki.melvoridle.com/index.php?title=Air_Adept_Wizard_Boots)|
|Water Adept|4|4%|16%|39 [Magic](https://wiki.melvoridle.com/index.php?title=Magic)|Magic|[Hat](https://wiki.melvoridle.com/index.php?title=Water_Adept_Wizard_Hat), [Robes](https://wiki.melvoridle.com/index.php?title=Water_Adept_Wizard_Robes), [Bottoms](https://wiki.melvoridle.com/index.php?title=Water_Adept_Wizard_Bottoms), [Boots](https://wiki.melvoridle.com/index.php?title=Water_Adept_Wizard_Boots)|
|Earth Adept|4|4%|16%|43 [Magic](https://wiki.melvoridle.com/index.php?title=Magic)|Magic|[Hat](https://wiki.melvoridle.com/index.php?title=Earth_Adept_Wizard_Hat), [Robes](https://wiki.melvoridle.com/index.php?title=Earth_Adept_Wizard_Robes), [Bottoms](https://wiki.melvoridle.com/index.php?title=Earth_Adept_Wizard_Bottoms), [Boots](https://wiki.melvoridle.com/index.php?title=Earth_Adept_Wizard_Boots)|
|Fire Adept|4|4%|16%|48 [Magic](https://wiki.melvoridle.com/index.php?title=Magic)|Magic|[Hat](https://wiki.melvoridle.com/index.php?title=Fire_Adept_Wizard_Hat), [Robes](https://wiki.melvoridle.com/index.php?title=Fire_Adept_Wizard_Robes), [Bottoms](https://wiki.melvoridle.com/index.php?title=Fire_Adept_Wizard_Bottoms), [Boots](https://wiki.melvoridle.com/index.php?title=Fire_Adept_Wizard_Boots)|
|Air Expert|4|5%|20%|65 [Magic](https://wiki.melvoridle.com/index.php?title=Magic)|Magic|[Hat](https://wiki.melvoridle.com/index.php?title=Air_Expert_Wizard_Hat), [Robes](https://wiki.melvoridle.com/index.php?title=Air_Expert_Wizard_Robes), [Bottoms](https://wiki.melvoridle.com/index.php?title=Air_Expert_Wizard_Bottoms), [Boots](https://wiki.melvoridle.com/index.php?title=Air_Expert_Wizard_Boots)|
|Water Expert|4|5%|20%|69 [Magic](https://wiki.melvoridle.com/index.php?title=Magic)|Magic|[Hat](https://wiki.melvoridle.com/index.php?title=Water_Expert_Wizard_Hat), [Robes](https://wiki.melvoridle.com/index.php?title=Water_Expert_Wizard_Robes), [Bottoms](https://wiki.melvoridle.com/index.php?title=Water_Expert_Wizard_Bottoms), [Boots](https://wiki.melvoridle.com/index.php?title=Water_Expert_Wizard_Boots)|
|Earth Expert|4|5%|20%|73 [Magic](https://wiki.melvoridle.com/index.php?title=Magic)|Magic|[Hat](https://wiki.melvoridle.com/index.php?title=Earth_Expert_Wizard_Hat), [Robes](https://wiki.melvoridle.com/index.php?title=Earth_Expert_Wizard_Robes), [Bottoms](https://wiki.melvoridle.com/index.php?title=Earth_Expert_Wizard_Bottoms), [Boots](https://wiki.melvoridle.com/index.php?title=Earth_Expert_Wizard_Boots)|
|Fire Expert|4|5%|20%|78 [Magic](https://wiki.melvoridle.com/index.php?title=Magic)|Magic|[Hat](https://wiki.melvoridle.com/index.php?title=Fire_Expert_Wizard_Hat), [Robes](https://wiki.melvoridle.com/index.php?title=Fire_Expert_Wizard_Robes), [Bottoms](https://wiki.melvoridle.com/index.php?title=Fire_Expert_Wizard_Bottoms), [Boots](https://wiki.melvoridle.com/index.php?title=Fire_Expert_Wizard_Boots)|
|Ancient Wizard|4|5%|20%|70 [Magic](https://wiki.melvoridle.com/index.php?title=Magic)|Magic|[Hat](https://wiki.melvoridle.com/index.php?title=Ancient_Wizard_Hat), [Robes](https://wiki.melvoridle.com/index.php?title=Ancient_Wizard_Robes), [Bottoms](https://wiki.melvoridle.com/index.php?title=Ancient_Wizard_Bottoms), [Boots](https://wiki.melvoridle.com/index.php?title=Ancient_Wizard_Boots)|
|Glacia God|5|8%|40%|85 [Magic](https://wiki.melvoridle.com/index.php?title=Magic)|Magic|[Helmet](https://wiki.melvoridle.com/index.php?title=Glacia_God_Helmet), [Platebody](https://wiki.melvoridle.com/index.php?title=Glacia_God_Platebody), [Platelegs](https://wiki.melvoridle.com/index.php?title=Glacia_God_Platelegs), [Boots](https://wiki.melvoridle.com/index.php?title=Glacia_God_Boots), [Gloves](https://wiki.melvoridle.com/index.php?title=Glacia_God_Gloves)|

## Maximum Damage Reduction

The following equipment sets describe the maximum damage reduction for each combat style. The [Cape of Completion](https://wiki.melvoridle.com/index.php?title=Cape_of_Completion) is excluded, since it probably won't be obtained while its damage reduction is still useful. The [Guardian Amulet](https://wiki.melvoridle.com/index.php?title=Guardian_Amulet)'s bonus 5% is also excluded, since its damage reduction only kicks in when already low on health, and thus isn't useful when calculating idle combat.

This is the theoretical maximum damage reduction that a player can reach, using the [Cape of Completion](https://wiki.melvoridle.com/index.php?title=Cape_of_Completion) with a [Guardian Amulet](https://wiki.melvoridle.com/index.php?title=Guardian_Amulet). This totals to a whopping **82%** reduction, or with a boosted [Guardian Amulet](https://wiki.melvoridle.com/index.php?title=Guardian_Amulet) and the proper combat triangle bonus, **108%**.

It is unrealistic to do calculations with this percentage reduction, as it relies on nearly impossible-to-get items and specific circumstances. It's worth noting that damage reduction caps at 100%; anything beyond that has no additional effect.

|**Source**|**Item**|**Reduction**|
|---|---|---|
|Passive|[Guardian Amulet](https://wiki.melvoridle.com/index.php?title=Guardian_Amulet)|5%|
|Pet|[Leonardo](https://wiki.melvoridle.com/index.php?title=Leonardo)|1%|
|Pet|[Erran](https://wiki.melvoridle.com/index.php?title=Erran)|1%|
|Agility|[Lake Swim](https://wiki.melvoridle.com/index.php?title=Agility)|1%|
|Prayer|[Stone Skin](https://wiki.melvoridle.com/index.php?title=Stone_Skin)|3%|
|Potion|[Damage Reduction Potion IV](https://wiki.melvoridle.com/index.php?title=Damage_Reduction_Potion_IV)|10%|
|Cape|[Cape of Completion](https://wiki.melvoridle.com/index.php?title=Cape_of_Completion)|5%|
|Ring|[Guardian Ring](https://wiki.melvoridle.com/index.php?title=Guardian_Ring)|2%|
|Amulet|[Fury of the Elemental Zodiacs](https://wiki.melvoridle.com/index.php?title=Fury_of_the_Elemental_Zodiacs)|2%|
|Shield|[Earth Layered Shield](https://wiki.melvoridle.com/index.php?title=Earth_Layered_Shield)|12%|
|Head|[Terran God Helmet](https://wiki.melvoridle.com/index.php?title=Terran_God_Helmet)|8%|
|Body|[Terran God Platebody](https://wiki.melvoridle.com/index.php?title=Terran_God_Platebody)|8%|
|Legs|[Terran God Platelegs](https://wiki.melvoridle.com/index.php?title=Terran_God_Platelegs)|8%|
|Hands|[Terran God Gloves](https://wiki.melvoridle.com/index.php?title=Terran_God_Gloves)|8%|
|Feet|[Terran God Boots](https://wiki.melvoridle.com/index.php?title=Terran_God_Boots)|8%|
|||
|**Description**|**Total Reduction**|**Notes**|
|Total|**82%**||
|Plus: Guardian Amulet bonus|87%|5% bonus for having <50% HP|
|Plus: Combat Triangle bonus|102%|+25% for fighting the weaker combat style|
|Plus: Guardian Amulet, Combat Triangle|108%|+25% for fighting the weaker combat style|

A more realistic maximum would use the [Infernal Cape](https://wiki.melvoridle.com/index.php?title=Infernal_Cape), and not rely on the [Guardian Amulet](https://wiki.melvoridle.com/index.php?title=Guardian_Amulet)'s bonus. This gives a sustainable maximum of **81%** damage reduction.

|**Source**|**Item**|**Reduction**|
|---|---|---|
|Passive|[Guardian Amulet](https://wiki.melvoridle.com/index.php?title=Guardian_Amulet)|5%|
|Pet|[Leonardo](https://wiki.melvoridle.com/index.php?title=Leonardo)|1%|
|Pet|[Erran](https://wiki.melvoridle.com/index.php?title=Erran)|1%|
|Agility|[Lake Swim](https://wiki.melvoridle.com/index.php?title=Agility)|1%|
|Prayer|[Stone Skin](https://wiki.melvoridle.com/index.php?title=Stone_Skin)|3%|
|Potion|[Damage Reduction Potion IV](https://wiki.melvoridle.com/index.php?title=Damage_Reduction_Potion_IV)|10%|
|Cape|[Infernal Cape](https://wiki.melvoridle.com/index.php?title=Infernal_Cape)|4%|
|Ring|[Guardian Ring](https://wiki.melvoridle.com/index.php?title=Guardian_Ring)|2%|
|Amulet|[Fury of the Elemental Zodiacs](https://wiki.melvoridle.com/index.php?title=Fury_of_the_Elemental_Zodiacs)|2%|
|Shield|[Earth Layered Shield](https://wiki.melvoridle.com/index.php?title=Earth_Layered_Shield)|12%|
|Head|[Terran God Helmet](https://wiki.melvoridle.com/index.php?title=Terran_God_Helmet)|8%|
|Body|[Terran God Platebody](https://wiki.melvoridle.com/index.php?title=Terran_God_Platebody)|8%|
|Legs|[Terran God Platelegs](https://wiki.melvoridle.com/index.php?title=Terran_God_Platelegs)|8%|
|Hands|[Terran God Gloves](https://wiki.melvoridle.com/index.php?title=Terran_God_Gloves)|8%|
|Feet|[Terran God Boots](https://wiki.melvoridle.com/index.php?title=Terran_God_Boots)|8%|
|||
|**Description**|**Total Reduction**|**Notes**|
|Total|**81%**||

## Loadouts

*Coming soon: example loadouts for dungeon-grinding - add a comment, using the above format, if you have any suggestions*

## Closing

Check out this project on [GitHub](https://github.com/gridster2/Melvor). Please add, correct, or suggest anything that comes to mind. See you next update!