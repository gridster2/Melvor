# Damage Reduction Tables for Dungeons

Melvor Idle v0.19.2, /u/gridster2

This is intended as an appendix to my [Compendium of Damage Reduction](https://www.reddit.com/r/MelvorIdle/comments/mk7zt3/compendium_of_damage_reduction/?).

### Introduction

The following charts give the damage reduction required to safely idle a dungeon with a given number of hitpoints using a given combat style, in both normal and [Hardcore](https://wiki.melvoridle.com/index.php?title=Hardcore) mode. For example, a player with 550 hitpoints, a magic combat style, and Auto Eat Tier III can safely idle Dragons Den with a damage reduction of 43. The [Wasteful Ring](https://wiki.melvoridle.com/index.php?title=Wasteful_Ring) (which increases the auto-eat threshold) adds a bonus of 5% (additive) to the Auto Eat threshold (for [Auto Eat III](https://wiki.melvoridle.com/index.php?title=Auto_Eat_-_Tier_III), this means a threshold of 45%), making it an important item to include in calculations.

The column headings use emoji to display combat style, difficulty, and equipment (excuse the pictographs; it's much more readable than long text). The ❤️ column gives the player's hitpoints (including bonuses). ⚔️ indicates that the player is using Melee, 🏹 indicates Ranged, 🧙 indicates Magic. ☠️ indicates Hardcore mode. 💍 indicates that the [Wasteful Ring](https://wiki.melvoridle.com/index.php?title=Wasteful_Ring) is equipped, adding 5% to the Auto Eat threshold.

Values in these charts crossed out and in italic mean that the given damage reduction is beyond the practical limit of 81%, and likely can't be reached. If the hitpoints "skips" a row (for example, the row for 890 hitpoints is followed by 910), then it is because the middle value has the same values as its predecessors (e.g. the damage reductions for 900 hitpoints is the same as for 890). The rows start at either 100 hitpoints or the point at which damage reduction values are reachable, and end at 1160 hitpoints, or when damage reduction values for all combat styles are 0.

### Note on "Into the Mist"

[Into the Mist](https://wiki.melvoridle.com/index.php?title=Into_the_Mist) is a unique dungeon, in that it needs only ever to be completed five times, it can spawn any enemy whose combat level is between 165 and 677 (inclusive), and the final three monsters each can only be damaged by a single type of damage. In addition, the player's health can potentially be capped at 50% max hitpoints. It included in this set of tables for completeness's sake (omitted on Reddit, due to length limits), and split into the warmup (the 20 randomized monsters), and the boss (the final three enemies); right now, none of the information provided is very useful, but further changes to the game may affect this. The table for the boss part of Into the Mist gives the damage reduction for each combat style; that is, this fight requires switching between three equipment sets, each with the given damage reduction. The max hit for each category _does_ take into account all of the possible randomized enemy spawns. Because of the potential hitpoint cap, the damage reduction penalty, and the variety of high-level multi-combat-style enemies, [Into the Mist](https://wiki.melvoridle.com/index.php?title=Into_the_Mist) cannot safely be idled.

### Max Hits of Dungeon Monsters

As of v0.18.0, certain enemies can affect the player's maximum hitpoints and their damage reduction. This makes it important to consider not just the max hit of the dungeon (as had been done for previous versions), but the specific monster dealing the hit. In [Into the Mist](https://wiki.melvoridle.com/index.php?title=Into_the_Mist), the player can potentially be "Afflicted", reducing maximum HP to 50%. The boss's first phase, [Mysterious Figure](https://wiki.melvoridle.com/index.php?title=Mysterious_Figure_-_Phase_1), can apply Mark of Death to the player, immediately halving their damage reduction. The below table lists the max hit of each monster in each dungeon, taking into account special attacks, and the potential for special attacks to deal extra damage (i.e. when the player is stunned).

|Dungeon|Enemy|Max Hit ⚔️|Max Hit 🏹|Max Hit 🧙
|---|---|---|---|---
|**[Chicken Coop](https://wiki.melvoridle.com/index.php?title=Chicken_Coop)**||||
||[Chicken](https://wiki.melvoridle.com/index.php?title=Chicken)|11|0|0
||[Chick](https://wiki.melvoridle.com/index.php?title=Chick)|10|0|0
||[Mumma Chicken](https://wiki.melvoridle.com/index.php?title=Mumma_Chicken)|52|0|0
||**Maximum**|52|0|0
|**[Undead Graveyard](https://wiki.melvoridle.com/index.php?title=Undead_Graveyard)**||||
||[Zombie Hand](https://wiki.melvoridle.com/index.php?title=Zombie_Hand)|42|0|0
||[Zombie](https://wiki.melvoridle.com/index.php?title=Zombie)|52|0|0
||[Zombie Leader](https://wiki.melvoridle.com/index.php?title=Zombie_Leader)|106|0|0
||[Ghost](https://wiki.melvoridle.com/index.php?title=Ghost)|62|0|0
||**Maximum**|106|0|0
|**[Bandit Base](https://wiki.melvoridle.com/index.php?title=Bandit_Base)**||||
||[Bandit Trainee](https://wiki.melvoridle.com/index.php?title=Bandit_Trainee)|0|42|0
||[Bandit](https://wiki.melvoridle.com/index.php?title=Bandit)|0|82|0
||[Bandit Leader](https://wiki.melvoridle.com/index.php?title=Bandit_Leader)|0|174|0
||**Maximum**|0|174|0
|**[Hall of Wizards](https://wiki.melvoridle.com/index.php?title=Hall_of_Wizards)**||||
||[Wizard](https://wiki.melvoridle.com/index.php?title=Wizard)|0|0|120
||[Dark Wizard](https://wiki.melvoridle.com/index.php?title=Dark_Wizard)|0|0|210
||[Master Wizard](https://wiki.melvoridle.com/index.php?title=Master_Wizard)|0|0|170
||[Elder Wizard](https://wiki.melvoridle.com/index.php?title=Elder_Wizard)|0|0|210
||**Maximum**|0|0|210
|**[Spider Forest](https://wiki.melvoridle.com/index.php?title=Spider_Forest)**||||
||[Spider](https://wiki.melvoridle.com/index.php?title=Spider)|72|0|0
||[Spider](https://wiki.melvoridle.com/index.php?title=Spider)|72|0|0
||[Evil Spider](https://wiki.melvoridle.com/index.php?title=Evil_Spider)|102|0|0
||[Spider King](https://wiki.melvoridle.com/index.php?title=Spider_King)|142|0|0
||**Maximum**|142|0|0
|**[Miolite Caves](https://wiki.melvoridle.com/index.php?title=Miolite_Caves)**||||
||[Miolite Sprig](https://wiki.melvoridle.com/index.php?title=Miolite_Sprig)|150|0|0
||[Miolite Trio](https://wiki.melvoridle.com/index.php?title=Miolite_Trio)|0|0|150
||[Miolite Warden](https://wiki.melvoridle.com/index.php?title=Miolite_Warden)|152|0|0
||[Miolite Monarch](https://wiki.melvoridle.com/index.php?title=Miolite_Monarch)|0|0|200
||**Maximum**|152|0|200
|**[Deep Sea Ship](https://wiki.melvoridle.com/index.php?title=Deep_Sea_Ship)**||||
||[Pirate](https://wiki.melvoridle.com/index.php?title=Pirate)|84|0|0
||[Pirate Captain](https://wiki.melvoridle.com/index.php?title=Pirate_Captain)|177|0|0
||[The Kraken](https://wiki.melvoridle.com/index.php?title=The_Kraken)|204|0|0
||[First Mate](https://wiki.melvoridle.com/index.php?title=First_Mate)|131|0|0
||**Maximum**|204|0|0
|**[Frozen Cove](https://wiki.melvoridle.com/index.php?title=Frozen_Cove)**||||
||[Ice Monster](https://wiki.melvoridle.com/index.php?title=Ice_Monster)|92|0|0
||[Ice Troll](https://wiki.melvoridle.com/index.php?title=Ice_Troll)|102|0|0
||[Ice](https://wiki.melvoridle.com/index.php?title=Ice)|102|0|0
||[Protector of Ice](https://wiki.melvoridle.com/index.php?title=Protector_of_Ice)|172|0|0
||**Maximum**|172|0|0
|**[Dragons Den](https://wiki.melvoridle.com/index.php?title=Dragons_Den)**||||
||[Green Dragon](https://wiki.melvoridle.com/index.php?title=Green_Dragon)|143|0|0
||[Blue Dragon](https://wiki.melvoridle.com/index.php?title=Blue_Dragon)|168|0|0
||[Red Dragon](https://wiki.melvoridle.com/index.php?title=Red_Dragon)|212|0|0
||[Black Dragon](https://wiki.melvoridle.com/index.php?title=Black_Dragon)|268|0|0
||[Elder Dragon](https://wiki.melvoridle.com/index.php?title=Elder_Dragon)|470|0|0
||**Maximum**|470|0|0
|**[Volcanic Cave](https://wiki.melvoridle.com/index.php?title=Volcanic_Cave)**||||
||[Bat](https://wiki.melvoridle.com/index.php?title=Bat)|52|0|0
||[Big Bat](https://wiki.melvoridle.com/index.php?title=Big_Bat)|82|0|0
||[The Eye](https://wiki.melvoridle.com/index.php?title=The_Eye)|0|0|140
||[Resurrected Eye](https://wiki.melvoridle.com/index.php?title=Resurrected_Eye)|0|0|240
||[Prat, the Protector of Secrets](https://wiki.melvoridle.com/index.php?title=Prat,_the_Protector_of_Secrets)|0|501|0
||[Malcs, the Guardian of Melvor](https://wiki.melvoridle.com/index.php?title=Malcs,_the_Guardian_of_Melvor)|520|0|0
||[Vicious Serpent](https://wiki.melvoridle.com/index.php?title=Vicious_Serpent)|0|282|0
||[Fire Spirit](https://wiki.melvoridle.com/index.php?title=Fire_Spirit)|0|0|340
||**Maximum**|520|501|340
|**[Infernal Stronghold](https://wiki.melvoridle.com/index.php?title=Infernal_Stronghold)**||||
||[Cerberus](https://wiki.melvoridle.com/index.php?title=Cerberus)|162|0|0
||[Fearful Eye](https://wiki.melvoridle.com/index.php?title=Fearful_Eye)|0|0|236
||[Red Devil](https://wiki.melvoridle.com/index.php?title=Red_Devil)|314|0|0
||[Phoenix](https://wiki.melvoridle.com/index.php?title=Phoenix)|0|0|371
||[Incendius](https://wiki.melvoridle.com/index.php?title=Incendius)|433|0|0
||[Prat, the Guardian of Secrets](https://wiki.melvoridle.com/index.php?title=Prat,_the_Guardian_of_Secrets)|0|522|0
||[Malcs, the Leader of Dragons](https://wiki.melvoridle.com/index.php?title=Malcs,_the_Leader_of_Dragons)|750|0|0
||**Maximum**|750|522|371
|**[Air God Dungeon](https://wiki.melvoridle.com/index.php?title=Air_God_Dungeon)**||||
||[Air Guard](https://wiki.melvoridle.com/index.php?title=Air_Guard)|0|340|0
||[Air Monster](https://wiki.melvoridle.com/index.php?title=Air_Monster)|441|0|0
||[Air Golem](https://wiki.melvoridle.com/index.php?title=Air_Golem)|0|650|0
||[Aleron](https://wiki.melvoridle.com/index.php?title=Aleron)|699|0|0
||[Voltaire](https://wiki.melvoridle.com/index.php?title=Voltaire)|0|713|0
||[Aeris](https://wiki.melvoridle.com/index.php?title=Aeris)|0|650|0
||**Maximum**|699|713|0
|**[Water God Dungeon](https://wiki.melvoridle.com/index.php?title=Water_God_Dungeon)**||||
||[Water Guard](https://wiki.melvoridle.com/index.php?title=Water_Guard)|0|0|370
||[Water Monster](https://wiki.melvoridle.com/index.php?title=Water_Monster)|650|0|0
||[Water Golem](https://wiki.melvoridle.com/index.php?title=Water_Golem)|0|0|845
||[Lissia](https://wiki.melvoridle.com/index.php?title=Lissia)|0|0|720
||[Murtia](https://wiki.melvoridle.com/index.php?title=Murtia)|733|0|0
||[Glacia](https://wiki.melvoridle.com/index.php?title=Glacia)|0|0|1080
||**Maximum**|733|0|1080
|**[Earth God Dungeon](https://wiki.melvoridle.com/index.php?title=Earth_God_Dungeon)**||||
||[Earth Guard](https://wiki.melvoridle.com/index.php?title=Earth_Guard)|0|320|0
||[Earth Monster](https://wiki.melvoridle.com/index.php?title=Earth_Monster)|650|0|0
||[Earth Golem](https://wiki.melvoridle.com/index.php?title=Earth_Golem)|850|0|0
||[Mistral](https://wiki.melvoridle.com/index.php?title=Mistral)|0|0|680
||[Ophidia](https://wiki.melvoridle.com/index.php?title=Ophidia)|699|0|0
||[Terran](https://wiki.melvoridle.com/index.php?title=Terran)|950|0|0
||**Maximum**|950|320|680
|**[Fire God Dungeon](https://wiki.melvoridle.com/index.php?title=Fire_God_Dungeon)**||||
||[Fire Guard](https://wiki.melvoridle.com/index.php?title=Fire_Guard)|650|0|0
||[Fire Monster](https://wiki.melvoridle.com/index.php?title=Fire_Monster)|487|0|0
||[Fire Golem](https://wiki.melvoridle.com/index.php?title=Fire_Golem)|0|0|580
||[Pyra](https://wiki.melvoridle.com/index.php?title=Pyra)|0|0|900
||[Ignis](https://wiki.melvoridle.com/index.php?title=Ignis)|900|0|0
||[Ragnar](https://wiki.melvoridle.com/index.php?title=Ragnar)|0|0|1300
||**Maximum**|900|0|1300
|**[Into the Mist (Warmup)](https://wiki.melvoridle.com/index.php?title=Into_the_Mist_(Warmup))**||||
||[Pegasus](https://wiki.melvoridle.com/index.php?title=Pegasus)|264|0|0
||[Dark Horned Elite](https://wiki.melvoridle.com/index.php?title=Dark_Horned_Elite)|443|0|0
||[The Kraken](https://wiki.melvoridle.com/index.php?title=The_Kraken)|204|0|0
||[Resurrected Eye](https://wiki.melvoridle.com/index.php?title=Resurrected_Eye)|0|0|240
||[Prat, the Protector of Secrets](https://wiki.melvoridle.com/index.php?title=Prat,_the_Protector_of_Secrets)|0|501|0
||[Malcs, the Guardian of Melvor](https://wiki.melvoridle.com/index.php?title=Malcs,_the_Guardian_of_Melvor)|520|0|0
||[Protector of Ice](https://wiki.melvoridle.com/index.php?title=Protector_of_Ice)|172|0|0
||[Vicious Serpent](https://wiki.melvoridle.com/index.php?title=Vicious_Serpent)|0|282|0
||[Fire Spirit](https://wiki.melvoridle.com/index.php?title=Fire_Spirit)|0|0|340
||[Air Guard](https://wiki.melvoridle.com/index.php?title=Air_Guard)|0|340|0
||[Air Monster](https://wiki.melvoridle.com/index.php?title=Air_Monster)|441|0|0
||[Air Golem](https://wiki.melvoridle.com/index.php?title=Air_Golem)|0|650|0
||[Aleron](https://wiki.melvoridle.com/index.php?title=Aleron)|699|0|0
||[Voltaire](https://wiki.melvoridle.com/index.php?title=Voltaire)|0|713|0
||[Water Guard](https://wiki.melvoridle.com/index.php?title=Water_Guard)|0|0|370
||[Water Monster](https://wiki.melvoridle.com/index.php?title=Water_Monster)|650|0|0
||[Water Golem](https://wiki.melvoridle.com/index.php?title=Water_Golem)|0|0|845
||[Lissia](https://wiki.melvoridle.com/index.php?title=Lissia)|0|0|720
||[Murtia](https://wiki.melvoridle.com/index.php?title=Murtia)|733|0|0
||[Earth Guard](https://wiki.melvoridle.com/index.php?title=Earth_Guard)|0|320|0
||[Earth Monster](https://wiki.melvoridle.com/index.php?title=Earth_Monster)|650|0|0
||[Earth Golem](https://wiki.melvoridle.com/index.php?title=Earth_Golem)|850|0|0
||[Mistral](https://wiki.melvoridle.com/index.php?title=Mistral)|0|0|680
||[Ophidia](https://wiki.melvoridle.com/index.php?title=Ophidia)|699|0|0
||[Fire Guard](https://wiki.melvoridle.com/index.php?title=Fire_Guard)|650|0|0
||[Fire Monster](https://wiki.melvoridle.com/index.php?title=Fire_Monster)|487|0|0
||[Fire Golem](https://wiki.melvoridle.com/index.php?title=Fire_Golem)|0|0|580
||[Pyra](https://wiki.melvoridle.com/index.php?title=Pyra)|0|0|900
||[Ignis](https://wiki.melvoridle.com/index.php?title=Ignis)|900|0|0
||[Elder Dragon](https://wiki.melvoridle.com/index.php?title=Elder_Dragon)|470|0|0
||[Furious Horned Elite](https://wiki.melvoridle.com/index.php?title=Furious_Horned_Elite)|0|397|0
||[Miolite Monarch](https://wiki.melvoridle.com/index.php?title=Miolite_Monarch)|0|0|200
||[Elementalist](https://wiki.melvoridle.com/index.php?title=Elementalist)|0|0|240
||[Sand Beast](https://wiki.melvoridle.com/index.php?title=Sand_Beast)|0|260|0
||[Rancora Spider](https://wiki.melvoridle.com/index.php?title=Rancora_Spider)|0|450|0
||[Elder Vampire](https://wiki.melvoridle.com/index.php?title=Elder_Vampire)|0|326|0
||[Cursed Maiden](https://wiki.melvoridle.com/index.php?title=Cursed_Maiden)|0|0|450
||[Bounty Hunter](https://wiki.melvoridle.com/index.php?title=Bounty_Hunter)|329|0|0
||[Chaotic Greater Dragon](https://wiki.melvoridle.com/index.php?title=Chaotic_Greater_Dragon)|600|0|0
||[Hunting Greater Dragon](https://wiki.melvoridle.com/index.php?title=Hunting_Greater_Dragon)|0|600|0
||[Wicked Greater Dragon](https://wiki.melvoridle.com/index.php?title=Wicked_Greater_Dragon)|0|0|120
||[Fearful Eye](https://wiki.melvoridle.com/index.php?title=Fearful_Eye)|0|0|236
||[Red Devil](https://wiki.melvoridle.com/index.php?title=Red_Devil)|314|0|0
||[Phoenix](https://wiki.melvoridle.com/index.php?title=Phoenix)|0|0|371
||[Incendius](https://wiki.melvoridle.com/index.php?title=Incendius)|433|0|0
||[Prat, the Guardian of Secrets](https://wiki.melvoridle.com/index.php?title=Prat,_the_Guardian_of_Secrets)|0|522|0
||**Maximum**|900|713|900
|**[Into the Mist (Boss)](https://wiki.melvoridle.com/index.php?title=Into_the_Mist_(Boss))**||||
||[Mysterious Figure - Phase 1](https://wiki.melvoridle.com/index.php?title=Mysterious_Figure_-_Phase_1)|600|0|0
||[Mysterious Figure - Phase 2](https://wiki.melvoridle.com/index.php?title=Mysterious_Figure_-_Phase_2)|0|450|0
||[Ahrenia](https://wiki.melvoridle.com/index.php?title=Ahrenia)|0|0|800
||**Maximum**|600|450|800

***

***

### [Chicken Coop](https://wiki.melvoridle.com/index.php?title=Chicken_Coop), Auto Eat I

|❤️|⚔️|🏹|🧙||⚔️ ☠️|🏹 ☠️|🧙 ☠️||⚔️ 💍|🏹 💍|🧙 💍||⚔️ ☠️ 💍|🏹 ☠️ 💍|🧙 ☠️ 💍|
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|100|60|64|48||60|80|48||51|54|41||51|68|41|
|110|56|59|45||56|75|45||47|50|38||47|63|38|
|120|52|55|42||52|70|42||41|44|33||41|55|33|
|130|49|52|40||49|66|40||37|39|30||37|50|30|
|140|45|48|36||45|60|36||31|33|25||31|42|25|
|150|41|44|33||41|55|33||27|29|22||27|36|22|
|160|37|39|30||37|50|30||22|24|18||22|30|18|
|170|33|35|27||33|44|27||18|19|15||18|24|15|
|180|29|31|24||29|39|24||12|13|10||12|16|10|
|190|26|28|21||26|35|21||8|9|7||8|11|7|
|200|22|24|18||22|30|18||2|3|2||2|3|2|
|210|18|19|15||18|24|15||0|0|0||0|0|0|
|220|14|15|12||14|19|12||0|0|0||0|0|0|
|230|10|11|8||10|14|8||0|0|0||0|0|0|
|240|6|7|5||6|8|5||0|0|0||0|0|0|
|250|2|3|2||2|3|2||0|0|0||0|0|0|
|260|0|0|0||0|0|0||0|0|0||0|0|0|

### [Chicken Coop](https://wiki.melvoridle.com/index.php?title=Chicken_Coop), Auto Eat II

|❤️|⚔️|🏹|🧙||⚔️ ☠️|🏹 ☠️|🧙 ☠️||⚔️ 💍|🏹 💍|🧙 💍||⚔️ ☠️ 💍|🏹 ☠️ 💍|🧙 ☠️ 💍|
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|100|41|44|33||41|55|33||31|33|25||31|42|25|
|110|35|37|28||35|47|28||26|28|21||26|35|21|
|120|29|31|24||29|39|24||18|19|15||18|24|15|
|130|24|26|20||24|32|20||12|13|10||12|16|10|
|140|18|19|15||18|24|15||4|5|4||4|6|4|
|150|12|13|10||12|16|10||0|0|0||0|0|0|
|160|6|7|5||6|8|5||0|0|0||0|0|0|
|170|1|2|1||1|2|1||0|0|0||0|0|0|
|180|0|0|0||0|0|0||0|0|0||0|0|0|

### [Chicken Coop](https://wiki.melvoridle.com/index.php?title=Chicken_Coop), Auto Eat III

|❤️|⚔️|🏹|🧙||⚔️ ☠️|🏹 ☠️|🧙 ☠️||⚔️ 💍|🏹 💍|🧙 💍||⚔️ ☠️ 💍|🏹 ☠️ 💍|🧙 ☠️ 💍|
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|100|22|24|18||22|30|18||12|13|10||12|16|10|
|110|14|15|12||14|19|12||4|5|4||4|6|4|
|120|6|7|5||6|8|5||0|0|0||0|0|0|
|130|0|0|0||0|0|0||0|0|0||0|0|0|

### [Undead Graveyard](https://wiki.melvoridle.com/index.php?title=Undead_Graveyard), Auto Eat I

|❤️|⚔️|🏹|🧙||⚔️ ☠️|🏹 ☠️|🧙 ☠️||⚔️ 💍|🏹 💍|🧙 💍||⚔️ ☠️ 💍|🏹 ☠️ 💍|🧙 ☠️ 💍|
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|100|81|~~*86*~~|65||81|~~*100*~~|65||76|80|61||76|~~*100*~~|61|
|110|79|~~*84*~~|64||79|~~*100*~~|64||74|78|60||74|~~*99*~~|60|
|120|77|~~*82*~~|62||77|~~*100*~~|62||71|75|57||71|~~*95*~~|57|
|130|75|79|60||75|~~*100*~~|60||69|73|56||69|~~*92*~~|56|
|140|73|77|59||73|~~*98*~~|59||67|71|54||67|~~*90*~~|54|
|150|71|75|57||71|~~*95*~~|57||65|69|52||65|~~*87*~~|52|
|160|69|73|56||69|~~*92*~~|56||62|66|50||62|~~*83*~~|50|
|170|67|71|54||67|~~*90*~~|54||60|64|48||60|80|48|
|180|66|70|53||66|~~*88*~~|53||57|60|46||57|76|46|
|190|64|68|52||64|~~*86*~~|52||55|58|44||55|74|44|
|200|62|66|50||62|~~*83*~~|50||52|55|42||52|70|42|
|210|60|64|48||60|80|48||51|54|41||51|68|41|
|220|58|62|47||58|78|47||48|51|39||48|64|39|
|230|56|59|45||56|75|45||46|49|37||46|62|37|
|240|54|57|44||54|72|44||43|46|35||43|58|35|
|250|52|55|42||52|70|42||41|44|33||41|55|33|
|260|51|54|41||51|68|41||38|40|31||38|51|31|
|270|49|52|40||49|66|40||36|38|29||36|48|29|
|280|47|50|38||47|63|38||34|36|28||34|46|28|
|290|45|48|36||45|60|36||32|34|26||32|43|26|
|300|43|46|35||43|58|35||29|31|24||29|39|24|
|310|41|44|33||41|55|33||27|29|22||27|36|22|
|320|39|42|32||39|52|32||24|26|20||24|32|20|
|330|37|39|30||37|50|30||22|24|18||22|30|18|
|340|35|37|28||35|47|28||19|20|16||19|26|16|
|350|34|36|28||34|46|28||17|18|14||17|23|14|
|360|32|34|26||32|43|26||15|16|12||15|20|12|
|370|30|32|24||30|40|24||13|14|11||13|18|11|
|380|28|30|23||28|38|23||10|11|8||10|14|8|
|390|26|28|21||26|35|21||8|9|7||8|11|7|
|400|24|26|20||24|32|20||5|6|4||5|7|4|
|410|22|24|18||22|30|18||3|4|3||3|4|3|
|420|20|22|16||20|27|16||1|2|1||1|2|1|
|430|18|19|15||18|24|15||0|0|0||0|0|0|
|440|17|18|14||17|23|14||0|0|0||0|0|0|
|450|15|16|12||15|20|12||0|0|0||0|0|0|
|460|13|14|11||13|18|11||0|0|0||0|0|0|
|470|11|12|9||11|15|9||0|0|0||0|0|0|
|480|9|10|8||9|12|8||0|0|0||0|0|0|
|490|7|8|6||7|10|6||0|0|0||0|0|0|
|500|5|6|4||5|7|4||0|0|0||0|0|0|
|510|3|4|3||3|4|3||0|0|0||0|0|0|
|520|1|2|1||1|2|1||0|0|0||0|0|0|
|530|0|0|0||0|0|0||0|0|0||0|0|0|

### [Undead Graveyard](https://wiki.melvoridle.com/index.php?title=Undead_Graveyard), Auto Eat II

|❤️|⚔️|🏹|🧙||⚔️ ☠️|🏹 ☠️|🧙 ☠️||⚔️ 💍|🏹 💍|🧙 💍||⚔️ ☠️ 💍|🏹 ☠️ 💍|🧙 ☠️ 💍|
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|100|71|75|57||71|~~*95*~~|57||67|71|54||67|~~*90*~~|54|
|110|68|72|55||68|~~*91*~~|55||64|68|52||64|~~*86*~~|52|
|120|66|70|53||66|~~*88*~~|53||60|64|48||60|80|48|
|130|63|67|51||63|~~*84*~~|51||57|60|46||57|76|46|
|140|60|64|48||60|80|48||53|56|43||53|71|43|
|150|57|60|46||57|76|46||51|54|41||51|68|41|
|160|54|57|44||54|72|44||47|50|38||47|63|38|
|170|51|54|41||51|68|41||44|47|36||44|59|36|
|180|49|52|40||49|66|40||41|44|33||41|55|33|
|190|46|49|37||46|62|37||37|39|30||37|50|30|
|200|43|46|35||43|58|35||34|36|28||34|46|28|
|210|40|43|32||40|54|32||31|33|25||31|42|25|
|220|37|39|30||37|50|30||27|29|22||27|36|22|
|230|34|36|28||34|46|28||24|26|20||24|32|20|
|240|32|34|26||32|43|26||20|22|16||20|27|16|
|250|29|31|24||29|39|24||17|18|14||17|23|14|
|260|26|28|21||26|35|21||14|15|12||14|19|12|
|270|23|25|19||23|31|19||11|12|9||11|15|9|
|280|20|22|16||20|27|16||7|8|6||7|10|6|
|290|17|18|14||17|23|14||4|5|4||4|6|4|
|300|15|16|12||15|20|12||1|2|1||1|2|1|
|310|12|13|10||12|16|10||0|0|0||0|0|0|
|320|9|10|8||9|12|8||0|0|0||0|0|0|
|330|6|7|5||6|8|5||0|0|0||0|0|0|
|340|3|4|3||3|4|3||0|0|0||0|0|0|
|350|1|2|1||1|2|1||0|0|0||0|0|0|
|360|0|0|0||0|0|0||0|0|0||0|0|0|

### [Undead Graveyard](https://wiki.melvoridle.com/index.php?title=Undead_Graveyard), Auto Eat III

|❤️|⚔️|🏹|🧙||⚔️ ☠️|🏹 ☠️|🧙 ☠️||⚔️ 💍|🏹 💍|🧙 💍||⚔️ ☠️ 💍|🏹 ☠️ 💍|🧙 ☠️ 💍|
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|100|62|66|50||62|~~*83*~~|50||57|60|46||57|76|46|
|110|58|62|47||58|78|47||53|56|43||53|71|43|
|120|54|57|44||54|72|44||49|52|40||49|66|40|
|130|51|54|41||51|68|41||45|48|36||45|60|36|
|140|47|50|38||47|63|38||40|43|32||40|54|32|
|150|43|46|35||43|58|35||36|38|29||36|48|29|
|160|39|42|32||39|52|32||32|34|26||32|43|26|
|170|35|37|28||35|47|28||28|30|23||28|38|23|
|180|32|34|26||32|43|26||23|25|19||23|31|19|
|190|28|30|23||28|38|23||19|20|16||19|26|16|
|200|24|26|20||24|32|20||15|16|12||15|20|12|
|210|20|22|16||20|27|16||11|12|9||11|15|9|
|220|17|18|14||17|23|14||6|7|5||6|8|5|
|230|13|14|11||13|18|11||2|3|2||2|3|2|
|240|9|10|8||9|12|8||0|0|0||0|0|0|
|250|5|6|4||5|7|4||0|0|0||0|0|0|
|260|1|2|1||1|2|1||0|0|0||0|0|0|
|270|0|0|0||0|0|0||0|0|0||0|0|0|

### [Bandit Base](https://wiki.melvoridle.com/index.php?title=Bandit_Base), Auto Eat I

|❤️|⚔️|🏹|🧙||⚔️ ☠️|🏹 ☠️|🧙 ☠️||⚔️ 💍|🏹 💍|🧙 💍||⚔️ ☠️ 💍|🏹 ☠️ 💍|🧙 ☠️ 💍|
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|100|71|~~*88*~~|~~*100*~~||71|~~*88*~~|~~*100*~~||69|~~*86*~~|~~*100*~~||69|~~*86*~~|~~*100*~~|
|110|70|~~*87*~~|~~*100*~~||70|~~*87*~~|~~*100*~~||68|~~*84*~~|~~*99*~~||68|~~*84*~~|~~*100*~~|
|120|69|~~*86*~~|~~*100*~~||69|~~*86*~~|~~*100*~~||67|~~*83*~~|~~*98*~~||67|~~*83*~~|~~*100*~~|
|130|68|~~*85*~~|~~*100*~~||68|~~*85*~~|~~*100*~~||66|~~*82*~~|~~*97*~~||66|~~*82*~~|~~*100*~~|
|140|68|~~*84*~~|~~*99*~~||68|~~*84*~~|~~*100*~~||64|80|~~*95*~~||64|80|~~*100*~~|
|150|67|~~*83*~~|~~*98*~~||67|~~*83*~~|~~*100*~~||64|79|~~*93*~~||64|79|~~*100*~~|
|160|66|~~*82*~~|~~*97*~~||66|~~*82*~~|~~*100*~~||62|77|~~*91*~~||62|77|~~*100*~~|
|170|64|80|~~*95*~~||64|80|~~*100*~~||61|76|~~*90*~~||61|76|~~*100*~~|
|180|64|79|~~*93*~~||64|79|~~*100*~~||60|74|~~*88*~~||60|74|~~*99*~~|
|190|63|78|~~*92*~~||63|78|~~*100*~~||59|73|~~*86*~~||59|73|~~*98*~~|
|200|62|77|~~*91*~~||62|77|~~*100*~~||57|71|~~*84*~~||57|71|~~*95*~~|
|210|61|76|~~*90*~~||61|76|~~*100*~~||56|70|~~*83*~~||56|70|~~*94*~~|
|220|60|75|~~*89*~~||60|75|~~*100*~~||55|68|80||55|68|~~*91*~~|
|230|59|73|~~*86*~~||59|73|~~*98*~~||54|67|79||54|67|~~*90*~~|
|240|58|72|~~*85*~~||58|72|~~*96*~~||52|65|77||52|65|~~*87*~~|
|250|57|71|~~*84*~~||57|71|~~*95*~~||52|64|76||52|64|~~*86*~~|
|260|56|70|~~*83*~~||56|70|~~*94*~~||51|63|75||51|63|~~*84*~~|
|270|56|69|~~*82*~~||56|69|~~*92*~~||49|61|72||49|61|~~*82*~~|
|280|55|68|80||55|68|~~*91*~~||48|60|71||48|60|80|
|290|54|67|79||54|67|~~*90*~~||48|59|70||48|59|79|
|300|52|65|77||52|65|~~*87*~~||46|57|68||46|57|76|
|310|52|64|76||52|64|~~*86*~~||45|56|66||45|56|75|
|320|51|63|75||51|63|~~*84*~~||44|54|64||44|54|72|
|330|50|62|73||50|62|~~*83*~~||43|53|63||43|53|71|
|340|49|61|72||49|61|~~*82*~~||41|51|60||41|51|68|
|350|48|60|71||48|60|80||40|50|59||40|50|67|
|360|48|59|70||48|59|79||39|48|57||39|48|64|
|370|46|57|68||46|57|76||38|47|56||38|47|63|
|380|45|56|66||45|56|75||36|45|53||36|45|60|
|390|44|55|65||44|55|74||36|44|52||36|44|59|
|400|44|54|64||44|54|72||34|42|50||34|42|56|
|410|43|53|63||43|53|71||33|41|49||33|41|55|
|420|42|52|62||42|52|70||32|40|48||32|40|54|
|430|41|51|60||41|51|68||31|38|45||31|38|51|
|440|40|49|58||40|49|66||30|37|44||30|37|50|
|450|39|48|57||39|48|64||29|36|43||29|36|48|
|460|38|47|56||38|47|63||28|34|40||28|34|46|
|470|37|46|55||37|46|62||27|33|39||27|33|44|
|480|36|45|53||36|45|60||25|31|37||25|31|42|
|490|36|44|52||36|44|59||24|30|36||24|30|40|
|500|34|42|50||34|42|56||23|28|33||23|28|38|
|510|33|41|49||33|41|55||22|27|32||22|27|36|
|520|32|40|48||32|40|54||20|25|30||20|25|34|
|530|32|39|46||32|39|52||20|24|29||20|24|32|
|540|31|38|45||31|38|51||18|22|26||18|22|30|
|550|30|37|44||30|37|50||17|21|25||17|21|28|
|560|29|36|43||29|36|48||16|19|23||16|19|26|
|570|28|34|40||28|34|46||15|18|22||15|18|24|
|580|27|33|39||27|33|44||14|17|20||14|17|23|
|590|26|32|38||26|32|43||12|15|18||12|15|20|
|600|25|31|37||25|31|42||12|14|17||12|14|19|
|610|24|30|36||24|30|40||11|13|16||11|13|18|
|620|24|29|35||24|29|39||9|11|13||9|11|15|
|630|23|28|33||23|28|38||8|10|12||8|10|14|
|640|21|26|31||21|26|35||7|8|10||7|8|11|
|650|20|25|30||20|25|34||6|7|9||6|7|10|
|660|20|24|29||20|24|32||4|5|6||4|5|7|
|670|19|23|28||19|23|31||4|4|5||4|4|6|
|680|18|22|26||18|22|30||2|2|3||2|2|3|
|690|17|21|25||17|21|28||1|1|2||1|1|2|
|700|16|19|23||16|19|26||0|0|0||0|0|0|
|710|15|18|22||15|18|24||0|0|0||0|0|0|
|720|14|17|20||14|17|23||0|0|0||0|0|0|
|730|13|16|19||13|16|22||0|0|0||0|0|0|
|740|12|15|18||12|15|20||0|0|0||0|0|0|
|750|12|14|17||12|14|19||0|0|0||0|0|0|
|760|11|13|16||11|13|18||0|0|0||0|0|0|
|770|9|11|13||9|11|15||0|0|0||0|0|0|
|780|8|10|12||8|10|14||0|0|0||0|0|0|
|790|8|9|11||8|9|12||0|0|0||0|0|0|
|800|7|8|10||7|8|11||0|0|0||0|0|0|
|810|6|7|9||6|7|10||0|0|0||0|0|0|
|820|5|6|8||5|6|8||0|0|0||0|0|0|
|830|4|5|6||4|5|7||0|0|0||0|0|0|
|840|3|3|4||3|3|4||0|0|0||0|0|0|
|850|2|2|3||2|2|3||0|0|0||0|0|0|
|860|1|1|2||1|1|2||0|0|0||0|0|0|
|870|0|0|0||0|0|0||0|0|0||0|0|0|

### [Bandit Base](https://wiki.melvoridle.com/index.php?title=Bandit_Base), Auto Eat II

|❤️|⚔️|🏹|🧙||⚔️ ☠️|🏹 ☠️|🧙 ☠️||⚔️ 💍|🏹 💍|🧙 💍||⚔️ ☠️ 💍|🏹 ☠️ 💍|🧙 ☠️ 💍|
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|100|67|~~*83*~~|~~*98*~~||67|~~*83*~~|~~*100*~~||64|80|~~*95*~~||64|80|~~*100*~~|
|110|65|81|~~*96*~~||65|81|~~*100*~~||63|78|~~*92*~~||63|78|~~*100*~~|
|120|64|79|~~*93*~~||64|79|~~*100*~~||61|76|~~*90*~~||61|76|~~*100*~~|
|130|63|78|~~*92*~~||63|78|~~*100*~~||60|74|~~*88*~~||60|74|~~*99*~~|
|140|61|76|~~*90*~~||61|76|~~*100*~~||58|72|~~*85*~~||58|72|~~*96*~~|
|150|60|74|~~*88*~~||60|74|~~*99*~~||56|70|~~*83*~~||56|70|~~*94*~~|
|160|58|72|~~*85*~~||58|72|~~*96*~~||55|68|80||55|68|~~*91*~~|
|170|57|71|~~*84*~~||57|71|~~*95*~~||53|66|78||53|66|~~*88*~~|
|180|56|69|~~*82*~~||56|69|~~*92*~~||52|64|76||52|64|~~*86*~~|
|190|54|67|79||54|67|~~*90*~~||50|62|73||50|62|~~*83*~~|
|200|52|65|77||52|65|~~*87*~~||48|60|71||48|60|80|
|210|52|64|76||52|64|~~*86*~~||47|58|69||47|58|78|
|220|50|62|73||50|62|~~*83*~~||45|56|66||45|56|75|
|230|48|60|71||48|60|80||44|54|64||44|54|72|
|240|48|59|70||48|59|79||42|52|62||42|52|70|
|250|46|57|68||46|57|76||40|50|59||40|50|67|
|260|44|55|65||44|55|74||39|48|57||39|48|64|
|270|43|53|63||43|53|71||37|46|55||37|46|62|
|280|42|52|62||42|52|70||36|44|52||36|44|59|
|290|40|50|59||40|50|67||34|42|50||34|42|56|
|300|39|48|57||39|48|64||32|40|48||32|40|54|
|310|37|46|55||37|46|62||31|38|45||31|38|51|
|320|36|45|53||36|45|60||29|36|43||29|36|48|
|330|35|43|51||35|43|58||28|34|40||28|34|46|
|340|33|41|49||33|41|55||26|32|38||26|32|43|
|350|32|40|48||32|40|54||24|30|36||24|30|40|
|360|31|38|45||31|38|51||23|28|33||23|28|38|
|370|29|36|43||29|36|48||21|26|31||21|26|35|
|380|28|34|40||28|34|46||19|23|28||19|23|31|
|390|27|33|39||27|33|44||18|22|26||18|22|30|
|400|25|31|37||25|31|42||16|19|23||16|19|26|
|410|24|29|35||24|29|39||15|18|22||15|18|24|
|420|23|28|33||23|28|38||12|15|18||12|15|20|
|430|21|26|31||21|26|35||12|14|17||12|14|19|
|440|20|24|29||20|24|32||9|11|13||9|11|15|
|450|18|22|26||18|22|30||8|10|12||8|10|14|
|460|17|21|25||17|21|28||6|7|9||6|7|10|
|470|16|19|23||16|19|26||5|6|8||5|6|8|
|480|14|17|20||14|17|23||3|3|4||3|3|4|
|490|12|15|18||12|15|20||2|2|3||2|2|3|
|500|12|14|17||12|14|19||0|0|0||0|0|0|
|510|10|12|15||10|12|16||0|0|0||0|0|0|
|520|8|10|12||8|10|14||0|0|0||0|0|0|
|530|8|9|11||8|9|12||0|0|0||0|0|0|
|540|6|7|9||6|7|10||0|0|0||0|0|0|
|550|4|5|6||4|5|7||0|0|0||0|0|0|
|560|3|3|4||3|3|4||0|0|0||0|0|0|
|570|2|2|3||2|2|3||0|0|0||0|0|0|
|580|0|0|0||0|0|0||0|0|0||0|0|0|

### [Bandit Base](https://wiki.melvoridle.com/index.php?title=Bandit_Base), Auto Eat III

|❤️|⚔️|🏹|🧙||⚔️ ☠️|🏹 ☠️|🧙 ☠️||⚔️ 💍|🏹 💍|🧙 💍||⚔️ ☠️ 💍|🏹 ☠️ 💍|🧙 ☠️ 💍|
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|100|62|77|~~*91*~~||62|77|~~*100*~~||60|74|~~*88*~~||60|74|~~*99*~~|
|110|60|75|~~*89*~~||60|75|~~*100*~~||58|72|~~*85*~~||58|72|~~*96*~~|
|120|58|72|~~*85*~~||58|72|~~*96*~~||56|69|~~*82*~~||56|69|~~*92*~~|
|130|56|70|~~*83*~~||56|70|~~*94*~~||54|67|79||54|67|~~*90*~~|
|140|55|68|80||55|68|~~*91*~~||52|64|76||52|64|~~*86*~~|
|150|52|65|77||52|65|~~*87*~~||49|61|72||49|61|~~*82*~~|
|160|51|63|75||51|63|~~*84*~~||48|59|70||48|59|79|
|170|49|61|72||49|61|~~*82*~~||45|56|66||45|56|75|
|180|48|59|70||48|59|79||43|53|63||43|53|71|
|190|45|56|66||45|56|75||41|51|60||41|51|68|
|200|44|54|64||44|54|72||39|48|57||39|48|64|
|210|42|52|62||42|52|70||37|46|55||37|46|62|
|220|40|49|58||40|49|66||35|43|51||35|43|58|
|230|38|47|56||38|47|63||33|41|49||33|41|55|
|240|36|45|53||36|45|60||31|38|45||31|38|51|
|250|34|42|50||34|42|56||29|36|43||29|36|48|
|260|32|40|48||32|40|54||27|33|39||27|33|44|
|270|31|38|45||31|38|51||24|30|36||24|30|40|
|280|29|36|43||29|36|48||23|28|33||23|28|38|
|290|27|33|39||27|33|44||20|25|30||20|25|34|
|300|25|31|37||25|31|42||18|22|26||18|22|30|
|310|24|29|35||24|29|39||16|20|24||16|20|27|
|320|21|26|31||21|26|35||14|17|20||14|17|23|
|330|20|24|29||20|24|32||12|15|18||12|15|20|
|340|18|22|26||18|22|30||10|12|15||10|12|16|
|350|16|19|23||16|19|26||8|10|12||8|10|14|
|360|14|17|20||14|17|23||6|7|9||6|7|10|
|370|12|15|18||12|15|20||4|5|6||4|5|7|
|380|11|13|16||11|13|18||2|2|3||2|2|3|
|390|8|10|12||8|10|14||0|0|0||0|0|0|
|400|7|8|10||7|8|11||0|0|0||0|0|0|
|410|5|6|8||5|6|8||0|0|0||0|0|0|
|420|3|3|4||3|3|4||0|0|0||0|0|0|
|430|1|1|2||1|1|2||0|0|0||0|0|0|
|440|0|0|0||0|0|0||0|0|0||0|0|0|

### [Hall of Wizards](https://wiki.melvoridle.com/index.php?title=Hall_of_Wizards), Auto Eat I

|❤️|⚔️|🏹|🧙||⚔️ ☠️|🏹 ☠️|🧙 ☠️||⚔️ 💍|🏹 💍|🧙 💍||⚔️ ☠️ 💍|🏹 ☠️ 💍|🧙 ☠️ 💍|
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|100|~~*100*~~|72|~~*90*~~||~~*100*~~|72|~~*90*~~||~~*100*~~|71|~~*88*~~||~~*100*~~|71|~~*88*~~|
|110|~~*100*~~|72|~~*90*~~||~~*100*~~|72|~~*90*~~||~~*100*~~|70|~~*87*~~||~~*100*~~|70|~~*87*~~|
|120|~~*100*~~|72|~~*89*~~||~~*100*~~|72|~~*89*~~||~~*100*~~|69|~~*86*~~||~~*100*~~|69|~~*86*~~|
|130|~~*100*~~|71|~~*88*~~||~~*100*~~|71|~~*88*~~||~~*100*~~|68|~~*85*~~||~~*100*~~|68|~~*85*~~|
|140|~~*100*~~|70|~~*87*~~||~~*100*~~|70|~~*87*~~||~~*100*~~|67|~~*83*~~||~~*100*~~|67|~~*83*~~|
|150|~~*100*~~|69|~~*86*~~||~~*100*~~|69|~~*86*~~||~~*100*~~|66|~~*82*~~||~~*100*~~|66|~~*82*~~|
|160|~~*100*~~|68|~~*85*~~||~~*100*~~|68|~~*85*~~||~~*100*~~|65|81||~~*100*~~|65|81|
|170|~~*100*~~|68|~~*84*~~||~~*100*~~|68|~~*84*~~||~~*100*~~|64|80||~~*100*~~|64|80|
|180|~~*100*~~|67|~~*83*~~||~~*100*~~|67|~~*83*~~||~~*100*~~|64|79||~~*100*~~|64|79|
|190|~~*100*~~|66|~~*82*~~||~~*100*~~|66|~~*82*~~||~~*100*~~|63|78||~~*100*~~|63|78|
|200|~~*100*~~|65|81||~~*100*~~|65|81||~~*100*~~|61|76||~~*100*~~|61|76|
|210|~~*100*~~|64|80||~~*100*~~|64|80||~~*100*~~|60|75||~~*100*~~|60|75|
|220|~~*100*~~|64|79||~~*100*~~|64|79||~~*100*~~|60|74||~~*100*~~|60|74|
|230|~~*100*~~|63|78||~~*100*~~|63|78||~~*100*~~|59|73||~~*100*~~|59|73|
|240|~~*100*~~|62|77||~~*100*~~|62|77||~~*100*~~|57|71||~~*100*~~|57|71|
|250|~~*100*~~|61|76||~~*100*~~|61|76||~~*100*~~|57|71||~~*100*~~|57|71|
|260|~~*100*~~|60|75||~~*100*~~|60|75||~~*100*~~|56|69||~~*100*~~|56|69|
|270|~~*100*~~|60|74||~~*100*~~|60|74||~~*100*~~|55|68||~~*100*~~|55|68|
|280|~~*100*~~|59|73||~~*100*~~|59|73||~~*100*~~|54|67||~~*100*~~|54|67|
|290|~~*100*~~|58|72||~~*100*~~|58|72||~~*100*~~|53|66||~~*100*~~|53|66|
|300|~~*100*~~|57|71||~~*100*~~|57|71||~~*100*~~|52|64||~~*100*~~|52|64|
|310|~~*100*~~|57|71||~~*100*~~|57|71||~~*100*~~|51|63||~~*100*~~|51|63|
|320|~~*100*~~|56|70||~~*100*~~|56|70||~~*100*~~|50|62||~~*100*~~|50|62|
|330|~~*100*~~|56|69||~~*100*~~|56|69||~~*100*~~|49|61||~~*100*~~|49|61|
|340|~~*100*~~|55|68||~~*100*~~|55|68||~~*100*~~|48|60||~~*100*~~|48|60|
|350|~~*100*~~|54|67||~~*100*~~|54|67||~~*100*~~|48|59||~~*100*~~|48|59|
|360|~~*100*~~|53|66||~~*100*~~|53|66||~~*100*~~|46|57||~~*100*~~|46|57|
|370|~~*100*~~|52|65||~~*100*~~|52|65||~~*100*~~|45|56||~~*100*~~|45|56|
|380|~~*100*~~|52|64||~~*100*~~|52|64||~~*100*~~|44|55||~~*100*~~|44|55|
|390|~~*100*~~|51|63||~~*100*~~|51|63||~~*100*~~|44|54||~~*100*~~|44|54|
|400|~~*100*~~|50|62||~~*100*~~|50|62||~~*100*~~|42|52||~~*100*~~|42|52|
|410|~~*100*~~|49|61||~~*100*~~|49|61||~~*100*~~|41|51||~~*100*~~|41|51|
|420|~~*100*~~|48|60||~~*100*~~|48|60||~~*100*~~|40|50||~~*100*~~|40|50|
|430|~~*100*~~|48|59||~~*100*~~|48|59||~~*98*~~|40|49||~~*100*~~|40|49|
|440|~~*100*~~|47|58||~~*100*~~|47|58||~~*96*~~|39|48||~~*100*~~|39|48|
|450|~~*100*~~|46|57||~~*100*~~|46|57||~~*94*~~|38|47||~~*100*~~|38|47|
|460|~~*100*~~|45|56||~~*100*~~|45|56||~~*90*~~|36|45||~~*100*~~|36|45|
|470|~~*100*~~|44|55||~~*100*~~|44|55||~~*88*~~|36|44||~~*100*~~|36|44|
|480|~~*100*~~|44|54||~~*100*~~|44|54||~~*86*~~|35|43||~~*100*~~|35|43|
|490|~~*100*~~|43|53||~~*100*~~|43|53||~~*84*~~|34|42||~~*100*~~|34|42|
|500|~~*100*~~|42|52||~~*100*~~|42|52||~~*82*~~|33|41||~~*100*~~|33|41|
|510|~~*100*~~|41|51||~~*100*~~|41|51||80|32|40||~~*100*~~|32|40|
|520|~~*100*~~|41|51||~~*100*~~|41|51||76|31|38||~~*100*~~|31|38|
|530|~~*100*~~|40|50||~~*100*~~|40|50||74|30|37||~~*100*~~|30|37|
|540|~~*98*~~|40|49||~~*100*~~|40|49||72|29|36||~~*100*~~|29|36|
|550|~~*96*~~|39|48||~~*100*~~|39|48||70|28|35||~~*100*~~|28|35|
|560|~~*94*~~|38|47||~~*100*~~|38|47||66|27|33||~~*100*~~|27|33|
|570|~~*92*~~|37|46||~~*100*~~|37|46||64|26|32||~~*100*~~|26|32|
|580|~~*90*~~|36|45||~~*100*~~|36|45||62|25|31||~~*100*~~|25|31|
|590|~~*88*~~|36|44||~~*100*~~|36|44||60|24|30||~~*100*~~|24|30|
|600|~~*86*~~|35|43||~~*100*~~|35|43||58|24|29||~~*100*~~|24|29|
|610|~~*84*~~|34|42||~~*100*~~|34|42||56|23|28||~~*100*~~|23|28|
|620|~~*82*~~|33|41||~~*100*~~|33|41||52|21|26||~~*100*~~|21|26|
|630|80|32|40||~~*100*~~|32|40||50|20|25||~~*100*~~|20|25|
|640|78|32|39||~~*100*~~|32|39||48|20|24||~~*96*~~|20|24|
|650|76|31|38||~~*100*~~|31|38||46|19|23||~~*92*~~|19|23|
|660|74|30|37||~~*100*~~|30|37||42|17|21||~~*84*~~|17|21|
|670|72|29|36||~~*100*~~|29|36||42|17|21||~~*84*~~|17|21|
|680|70|28|35||~~*100*~~|28|35||38|16|19||76|16|19|
|690|68|28|34||~~*100*~~|28|34||36|15|18||72|15|18|
|700|66|27|33||~~*100*~~|27|33||34|14|17||68|14|17|
|710|64|26|32||~~*100*~~|26|32||32|13|16||64|13|16|
|720|62|25|31||~~*100*~~|25|31||28|12|14||56|12|14|
|730|62|25|31||~~*100*~~|25|31||26|11|13||52|11|13|
|740|60|24|30||~~*100*~~|24|30||24|10|12||48|10|12|
|750|58|24|29||~~*100*~~|24|29||22|9|11||44|9|11|
|760|56|23|28||~~*100*~~|23|28||20|8|10||40|8|10|
|770|54|22|27||~~*100*~~|22|27||18|8|9||36|8|9|
|780|52|21|26||~~*100*~~|21|26||14|6|7||28|6|7|
|790|50|20|25||~~*100*~~|20|25||12|5|6||24|5|6|
|800|48|20|24||~~*96*~~|20|24||10|4|5||20|4|5|
|810|46|19|23||~~*92*~~|19|23||8|4|4||16|4|4|
|820|44|18|22||~~*88*~~|18|22||4|2|2||8|2|2|
|830|42|17|21||~~*84*~~|17|21||2|1|1||4|1|1|
|840|40|16|20||80|16|20||0|0|0||0|0|0|
|850|38|16|19||76|16|19||0|0|0||0|0|0|
|860|36|15|18||72|15|18||0|0|0||0|0|0|
|870|34|14|17||68|14|17||0|0|0||0|0|0|
|880|32|13|16||64|13|16||0|0|0||0|0|0|
|890|30|12|15||60|12|15||0|0|0||0|0|0|
|900|28|12|14||56|12|14||0|0|0||0|0|0|
|910|26|11|13||52|11|13||0|0|0||0|0|0|
|920|24|10|12||48|10|12||0|0|0||0|0|0|
|930|22|9|11||44|9|11||0|0|0||0|0|0|
|950|20|8|10||40|8|10||0|0|0||0|0|0|
|960|18|8|9||36|8|9||0|0|0||0|0|0|
|970|16|7|8||32|7|8||0|0|0||0|0|0|
|980|14|6|7||28|6|7||0|0|0||0|0|0|
|990|12|5|6||24|5|6||0|0|0||0|0|0|
|1000|10|4|5||20|4|5||0|0|0||0|0|0|
|1010|8|4|4||16|4|4||0|0|0||0|0|0|
|1020|6|3|3||12|3|3||0|0|0||0|0|0|
|1030|4|2|2||8|2|2||0|0|0||0|0|0|
|1040|2|1|1||4|1|1||0|0|0||0|0|0|
|1050|0|0|0||0|0|0||0|0|0||0|0|0|

### [Hall of Wizards](https://wiki.melvoridle.com/index.php?title=Hall_of_Wizards), Auto Eat II

|❤️|⚔️|🏹|🧙||⚔️ ☠️|🏹 ☠️|🧙 ☠️||⚔️ 💍|🏹 💍|🧙 💍||⚔️ ☠️ 💍|🏹 ☠️ 💍|🧙 ☠️ 💍|
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|100|~~*100*~~|69|~~*86*~~||~~*100*~~|69|~~*86*~~||~~*100*~~|67|~~*83*~~||~~*100*~~|67|~~*83*~~|
|110|~~*100*~~|68|~~*84*~~||~~*100*~~|68|~~*84*~~||~~*100*~~|66|~~*82*~~||~~*100*~~|66|~~*82*~~|
|120|~~*100*~~|67|~~*83*~~||~~*100*~~|67|~~*83*~~||~~*100*~~|64|80||~~*100*~~|64|80|
|130|~~*100*~~|65|81||~~*100*~~|65|81||~~*100*~~|64|79||~~*100*~~|64|79|
|140|~~*100*~~|64|80||~~*100*~~|64|80||~~*100*~~|62|77||~~*100*~~|62|77|
|150|~~*100*~~|64|79||~~*100*~~|64|79||~~*100*~~|60|75||~~*100*~~|60|75|
|160|~~*100*~~|62|77||~~*100*~~|62|77||~~*100*~~|59|73||~~*100*~~|59|73|
|170|~~*100*~~|61|76||~~*100*~~|61|76||~~*100*~~|58|72||~~*100*~~|58|72|
|180|~~*100*~~|60|74||~~*100*~~|60|74||~~*100*~~|57|71||~~*100*~~|57|71|
|190|~~*100*~~|59|73||~~*100*~~|59|73||~~*100*~~|56|69||~~*100*~~|56|69|
|200|~~*100*~~|57|71||~~*100*~~|57|71||~~*100*~~|54|67||~~*100*~~|54|67|
|210|~~*100*~~|56|70||~~*100*~~|56|70||~~*100*~~|52|65||~~*100*~~|52|65|
|220|~~*100*~~|56|69||~~*100*~~|56|69||~~*100*~~|51|63||~~*100*~~|51|63|
|230|~~*100*~~|54|67||~~*100*~~|54|67||~~*100*~~|50|62||~~*100*~~|50|62|
|240|~~*100*~~|53|66||~~*100*~~|53|66||~~*100*~~|48|60||~~*100*~~|48|60|
|250|~~*100*~~|52|64||~~*100*~~|52|64||~~*100*~~|48|59||~~*100*~~|48|59|
|260|~~*100*~~|51|63||~~*100*~~|51|63||~~*100*~~|46|57||~~*100*~~|46|57|
|270|~~*100*~~|49|61||~~*100*~~|49|61||~~*100*~~|44|55||~~*100*~~|44|55|
|280|~~*100*~~|48|60||~~*100*~~|48|60||~~*100*~~|43|53||~~*100*~~|43|53|
|290|~~*100*~~|48|59||~~*100*~~|48|59||~~*100*~~|42|52||~~*100*~~|42|52|
|300|~~*100*~~|46|57||~~*100*~~|46|57||~~*100*~~|40|50||~~*100*~~|40|50|
|310|~~*100*~~|45|56||~~*100*~~|45|56||~~*98*~~|40|49||~~*100*~~|40|49|
|320|~~*100*~~|44|54||~~*100*~~|44|54||~~*94*~~|38|47||~~*100*~~|38|47|
|330|~~*100*~~|43|53||~~*100*~~|43|53||~~*90*~~|36|45||~~*100*~~|36|45|
|340|~~*100*~~|41|51||~~*100*~~|41|51||~~*88*~~|36|44||~~*100*~~|36|44|
|350|~~*100*~~|40|50||~~*100*~~|40|50||~~*84*~~|34|42||~~*100*~~|34|42|
|360|~~*98*~~|40|49||~~*100*~~|40|49||~~*82*~~|33|41||~~*100*~~|33|41|
|370|~~*94*~~|38|47||~~*100*~~|38|47||78|32|39||~~*100*~~|32|39|
|380|~~*92*~~|37|46||~~*100*~~|37|46||74|30|37||~~*100*~~|30|37|
|390|~~*88*~~|36|44||~~*100*~~|36|44||70|28|35||~~*100*~~|28|35|
|400|~~*86*~~|35|43||~~*100*~~|35|43||66|27|33||~~*100*~~|27|33|
|410|~~*82*~~|33|41||~~*100*~~|33|41||64|26|32||~~*100*~~|26|32|
|420|80|32|40||~~*100*~~|32|40||60|24|30||~~*100*~~|24|30|
|430|78|32|39||~~*100*~~|32|39||58|24|29||~~*100*~~|24|29|
|440|74|30|37||~~*100*~~|30|37||54|22|27||~~*100*~~|22|27|
|450|72|29|36||~~*100*~~|29|36||50|20|25||~~*100*~~|20|25|
|460|68|28|34||~~*100*~~|28|34||46|19|23||~~*92*~~|19|23|
|470|66|27|33||~~*100*~~|27|33||44|18|22||~~*88*~~|18|22|
|480|62|25|31||~~*100*~~|25|31||40|16|20||80|16|20|
|490|60|24|30||~~*100*~~|24|30||38|16|19||76|16|19|
|500|58|24|29||~~*100*~~|24|29||34|14|17||68|14|17|
|510|54|22|27||~~*100*~~|22|27||30|12|15||60|12|15|
|520|52|21|26||~~*100*~~|21|26||26|11|13||52|11|13|
|530|48|20|24||~~*96*~~|20|24||24|10|12||48|10|12|
|540|46|19|23||~~*92*~~|19|23||20|8|10||40|8|10|
|550|42|17|21||~~*84*~~|17|21||18|8|9||36|8|9|
|560|40|16|20||80|16|20||14|6|7||28|6|7|
|570|38|16|19||76|16|19||10|4|5||20|4|5|
|580|34|14|17||68|14|17||6|3|3||12|3|3|
|590|32|13|16||64|13|16||4|2|2||8|2|2|
|600|28|12|14||56|12|14||0|0|0||0|0|0|
|610|26|11|13||52|11|13||0|0|0||0|0|0|
|620|22|9|11||44|9|11||0|0|0||0|0|0|
|630|20|8|10||40|8|10||0|0|0||0|0|0|
|640|18|8|9||36|8|9||0|0|0||0|0|0|
|650|14|6|7||28|6|7||0|0|0||0|0|0|
|660|12|5|6||24|5|6||0|0|0||0|0|0|
|670|8|4|4||16|4|4||0|0|0||0|0|0|
|680|6|3|3||12|3|3||0|0|0||0|0|0|
|690|2|1|1||4|1|1||0|0|0||0|0|0|
|700|0|0|0||0|0|0||0|0|0||0|0|0|

### [Hall of Wizards](https://wiki.melvoridle.com/index.php?title=Hall_of_Wizards), Auto Eat III

|❤️|⚔️|🏹|🧙||⚔️ ☠️|🏹 ☠️|🧙 ☠️||⚔️ 💍|🏹 💍|🧙 💍||⚔️ ☠️ 💍|🏹 ☠️ 💍|🧙 ☠️ 💍|
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|100|~~*100*~~|65|81||~~*100*~~|65|81||~~*100*~~|64|79||~~*100*~~|64|79|
|110|~~*100*~~|64|79||~~*100*~~|64|79||~~*100*~~|62|77||~~*100*~~|62|77|
|120|~~*100*~~|62|77||~~*100*~~|62|77||~~*100*~~|60|74||~~*100*~~|60|74|
|130|~~*100*~~|60|75||~~*100*~~|60|75||~~*100*~~|58|72||~~*100*~~|58|72|
|140|~~*100*~~|59|73||~~*100*~~|59|73||~~*100*~~|56|70||~~*100*~~|56|70|
|150|~~*100*~~|57|71||~~*100*~~|57|71||~~*100*~~|55|68||~~*100*~~|55|68|
|160|~~*100*~~|56|70||~~*100*~~|56|70||~~*100*~~|53|66||~~*100*~~|53|66|
|170|~~*100*~~|55|68||~~*100*~~|55|68||~~*100*~~|52|64||~~*100*~~|52|64|
|180|~~*100*~~|53|66||~~*100*~~|53|66||~~*100*~~|49|61||~~*100*~~|49|61|
|190|~~*100*~~|52|64||~~*100*~~|52|64||~~*100*~~|48|60||~~*100*~~|48|60|
|200|~~*100*~~|50|62||~~*100*~~|50|62||~~*100*~~|46|57||~~*100*~~|46|57|
|210|~~*100*~~|48|60||~~*100*~~|48|60||~~*100*~~|44|55||~~*100*~~|44|55|
|220|~~*100*~~|47|58||~~*100*~~|47|58||~~*100*~~|43|53||~~*100*~~|43|53|
|230|~~*100*~~|45|56||~~*100*~~|45|56||~~*100*~~|41|51||~~*100*~~|41|51|
|240|~~*100*~~|44|54||~~*100*~~|44|54||~~*98*~~|40|49||~~*100*~~|40|49|
|250|~~*100*~~|42|52||~~*100*~~|42|52||~~*94*~~|38|47||~~*100*~~|38|47|
|260|~~*100*~~|41|51||~~*100*~~|41|51||~~*88*~~|36|44||~~*100*~~|36|44|
|270|~~*98*~~|40|49||~~*100*~~|40|49||~~*84*~~|34|42||~~*100*~~|34|42|
|280|~~*94*~~|38|47||~~*100*~~|38|47||80|32|40||~~*100*~~|32|40|
|290|~~*90*~~|36|45||~~*100*~~|36|45||76|31|38||~~*100*~~|31|38|
|300|~~*86*~~|35|43||~~*100*~~|35|43||72|29|36||~~*100*~~|29|36|
|310|~~*82*~~|33|41||~~*100*~~|33|41||68|28|34||~~*100*~~|28|34|
|320|78|32|39||~~*100*~~|32|39||62|25|31||~~*100*~~|25|31|
|330|74|30|37||~~*100*~~|30|37||60|24|30||~~*100*~~|24|30|
|340|70|28|35||~~*100*~~|28|35||54|22|27||~~*100*~~|22|27|
|350|66|27|33||~~*100*~~|27|33||50|20|25||~~*100*~~|20|25|
|360|62|25|31||~~*100*~~|25|31||46|19|23||~~*92*~~|19|23|
|370|60|24|30||~~*100*~~|24|30||42|17|21||~~*84*~~|17|21|
|380|56|23|28||~~*100*~~|23|28||38|16|19||76|16|19|
|390|52|21|26||~~*100*~~|21|26||34|14|17||68|14|17|
|400|48|20|24||~~*96*~~|20|24||28|12|14||56|12|14|
|410|44|18|22||~~*88*~~|18|22||24|10|12||48|10|12|
|420|40|16|20||80|16|20||20|8|10||40|8|10|
|430|36|15|18||72|15|18||16|7|8||32|7|8|
|440|32|13|16||64|13|16||12|5|6||24|5|6|
|450|28|12|14||56|12|14||8|4|4||16|4|4|
|460|24|10|12||48|10|12||2|1|1||4|1|1|
|470|22|9|11||44|9|11||0|0|0||0|0|0|
|480|18|8|9||36|8|9||0|0|0||0|0|0|
|490|14|6|7||28|6|7||0|0|0||0|0|0|
|500|10|4|5||20|4|5||0|0|0||0|0|0|
|510|6|3|3||12|3|3||0|0|0||0|0|0|
|520|2|1|1||4|1|1||0|0|0||0|0|0|
|530|0|0|0||0|0|0||0|0|0||0|0|0|

### [Spider Forest](https://wiki.melvoridle.com/index.php?title=Spider_Forest), Auto Eat I

|❤️|⚔️|🏹|🧙||⚔️ ☠️|🏹 ☠️|🧙 ☠️||⚔️ 💍|🏹 💍|🧙 💍||⚔️ ☠️ 💍|🏹 ☠️ 💍|🧙 ☠️ 💍|
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|100|~~*86*~~|~~*91*~~|69||~~*86*~~|~~*100*~~|69||~~*82*~~|~~*87*~~|66||~~*82*~~|~~*100*~~|66|
|110|~~*84*~~|~~*89*~~|68||~~*84*~~|~~*100*~~|68||81|~~*86*~~|65||81|~~*100*~~|65|
|120|~~*83*~~|~~*88*~~|67||~~*83*~~|~~*100*~~|67||79|~~*84*~~|64||79|~~*100*~~|64|
|130|81|~~*86*~~|65||81|~~*100*~~|65||77|~~*82*~~|62||77|~~*100*~~|62|
|140|80|~~*85*~~|64||80|~~*100*~~|64||75|79|60||75|~~*100*~~|60|
|150|79|~~*84*~~|64||79|~~*100*~~|64||74|78|60||74|~~*99*~~|60|
|160|77|~~*82*~~|62||77|~~*100*~~|62||72|76|58||72|~~*96*~~|58|
|170|76|80|61||76|~~*100*~~|61||70|74|56||70|~~*94*~~|56|
|180|74|78|60||74|~~*99*~~|60||68|72|55||68|~~*91*~~|55|
|190|73|77|59||73|~~*98*~~|59||67|71|54||67|~~*90*~~|54|
|200|72|76|58||72|~~*96*~~|58||65|69|52||65|~~*87*~~|52|
|210|70|74|56||70|~~*94*~~|56||63|67|51||63|~~*84*~~|51|
|220|69|73|56||69|~~*92*~~|56||61|65|49||61|~~*82*~~|49|
|230|67|71|54||67|~~*90*~~|54||60|64|48||60|80|48|
|240|66|70|53||66|~~*88*~~|53||58|62|47||58|78|47|
|250|65|69|52||65|~~*87*~~|52||56|59|45||56|75|45|
|260|63|67|51||63|~~*84*~~|51||54|57|44||54|72|44|
|270|62|66|50||62|~~*83*~~|50||53|56|43||53|71|43|
|280|60|64|48||60|80|48||51|54|41||51|68|41|
|290|59|63|48||59|79|48||49|52|40||49|66|40|
|300|58|62|47||58|78|47||47|50|38||47|63|38|
|310|56|59|45||56|75|45||46|49|37||46|62|37|
|320|55|58|44||55|74|44||43|46|35||43|58|35|
|330|53|56|43||53|71|43||42|45|34||42|56|34|
|340|52|55|42||52|70|42||40|43|32||40|54|32|
|350|51|54|41||51|68|41||39|42|32||39|52|32|
|360|49|52|40||49|66|40||36|38|29||36|48|29|
|370|48|51|39||48|64|39||35|37|28||35|47|28|
|380|46|49|37||46|62|37||33|35|27||33|44|27|
|390|45|48|36||45|60|36||31|33|25||31|42|25|
|400|43|46|35||43|58|35||29|31|24||29|39|24|
|410|42|45|34||42|56|34||28|30|23||28|38|23|
|420|41|44|33||41|55|33||26|28|21||26|35|21|
|430|39|42|32||39|52|32||24|26|20||24|32|20|
|440|38|40|31||38|51|31||22|24|18||22|30|18|
|450|36|38|29||36|48|29||21|23|17||21|28|17|
|460|35|37|28||35|47|28||19|20|16||19|26|16|
|470|34|36|28||34|46|28||17|18|14||17|23|14|
|480|32|34|26||32|43|26||15|16|12||15|20|12|
|490|31|33|25||31|42|25||14|15|12||14|19|12|
|500|29|31|24||29|39|24||12|13|10||12|16|10|
|510|28|30|23||28|38|23||10|11|8||10|14|8|
|520|27|29|22||27|36|22||8|9|7||8|11|7|
|530|25|27|20||25|34|20||7|8|6||7|10|6|
|540|24|26|20||24|32|20||5|6|4||5|7|4|
|550|22|24|18||22|30|18||3|4|3||3|4|3|
|560|21|23|17||21|28|17||1|2|1||1|2|1|
|570|20|22|16||20|27|16||0|0|0||0|0|0|
|580|18|19|15||18|24|15||0|0|0||0|0|0|
|590|17|18|14||17|23|14||0|0|0||0|0|0|
|600|15|16|12||15|20|12||0|0|0||0|0|0|
|610|14|15|12||14|19|12||0|0|0||0|0|0|
|620|12|13|10||12|16|10||0|0|0||0|0|0|
|630|11|12|9||11|15|9||0|0|0||0|0|0|
|640|10|11|8||10|14|8||0|0|0||0|0|0|
|650|8|9|7||8|11|7||0|0|0||0|0|0|
|660|7|8|6||7|10|6||0|0|0||0|0|0|
|670|5|6|4||5|7|4||0|0|0||0|0|0|
|680|4|5|4||4|6|4||0|0|0||0|0|0|
|690|3|4|3||3|4|3||0|0|0||0|0|0|
|700|1|2|1||1|2|1||0|0|0||0|0|0|
|710|0|0|0||0|0|0||0|0|0||0|0|0|

### [Spider Forest](https://wiki.melvoridle.com/index.php?title=Spider_Forest), Auto Eat II

|❤️|⚔️|🏹|🧙||⚔️ ☠️|🏹 ☠️|🧙 ☠️||⚔️ 💍|🏹 💍|🧙 💍||⚔️ ☠️ 💍|🏹 ☠️ 💍|🧙 ☠️ 💍|
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|100|79|~~*84*~~|64||79|~~*100*~~|64||75|79|60||75|~~*100*~~|60|
|110|77|~~*82*~~|62||77|~~*100*~~|62||73|77|59||73|~~*98*~~|59|
|120|74|78|60||74|~~*99*~~|60||70|74|56||70|~~*94*~~|56|
|130|72|76|58||72|~~*96*~~|58||68|72|55||68|~~*91*~~|55|
|140|70|74|56||70|~~*94*~~|56||65|69|52||65|~~*87*~~|52|
|150|68|72|55||68|~~*91*~~|55||63|67|51||63|~~*84*~~|51|
|160|66|70|53||66|~~*88*~~|53||60|64|48||60|80|48|
|170|64|68|52||64|~~*86*~~|52||58|62|47||58|78|47|
|180|62|66|50||62|~~*83*~~|50||56|59|45||56|75|45|
|190|60|64|48||60|80|48||53|56|43||53|71|43|
|200|58|62|47||58|78|47||51|54|41||51|68|41|
|210|55|58|44||55|74|44||48|51|39||48|64|39|
|220|53|56|43||53|71|43||46|49|37||46|62|37|
|230|51|54|41||51|68|41||43|46|35||43|58|35|
|240|49|52|40||49|66|40||41|44|33||41|55|33|
|250|47|50|38||47|63|38||39|42|32||39|52|32|
|260|45|48|36||45|60|36||36|38|29||36|48|29|
|270|43|46|35||43|58|35||34|36|28||34|46|28|
|280|41|44|33||41|55|33||31|33|25||31|42|25|
|290|39|42|32||39|52|32||29|31|24||29|39|24|
|300|36|38|29||36|48|29||26|28|21||26|35|21|
|310|34|36|28||34|46|28||24|26|20||24|32|20|
|320|32|34|26||32|43|26||21|23|17||21|28|17|
|330|30|32|24||30|40|24||19|20|16||19|26|16|
|340|28|30|23||28|38|23||17|18|14||17|23|14|
|350|26|28|21||26|35|21||14|15|12||14|19|12|
|360|24|26|20||24|32|20||12|13|10||12|16|10|
|370|22|24|18||22|30|18||9|10|8||9|12|8|
|380|20|22|16||20|27|16||6|7|5||6|8|5|
|390|17|18|14||17|23|14||4|5|4||4|6|4|
|400|15|16|12||15|20|12||1|2|1||1|2|1|
|410|13|14|11||13|18|11||0|0|0||0|0|0|
|420|11|12|9||11|15|9||0|0|0||0|0|0|
|430|9|10|8||9|12|8||0|0|0||0|0|0|
|440|7|8|6||7|10|6||0|0|0||0|0|0|
|450|5|6|4||5|7|4||0|0|0||0|0|0|
|460|3|4|3||3|4|3||0|0|0||0|0|0|
|470|1|2|1||1|2|1||0|0|0||0|0|0|
|480|0|0|0||0|0|0||0|0|0||0|0|0|

### [Spider Forest](https://wiki.melvoridle.com/index.php?title=Spider_Forest), Auto Eat III

|❤️|⚔️|🏹|🧙||⚔️ ☠️|🏹 ☠️|🧙 ☠️||⚔️ 💍|🏹 💍|🧙 💍||⚔️ ☠️ 💍|🏹 ☠️ 💍|🧙 ☠️ 💍|
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|100|72|76|58||72|~~*96*~~|58||68|72|55||68|~~*91*~~|55|
|110|69|73|56||69|~~*92*~~|56||65|69|52||65|~~*87*~~|52|
|120|66|70|53||66|~~*88*~~|53||62|66|50||62|~~*83*~~|50|
|130|63|67|51||63|~~*84*~~|51||59|63|48||59|79|48|
|140|60|64|48||60|80|48||55|58|44||55|74|44|
|150|58|62|47||58|78|47||53|56|43||53|71|43|
|160|55|58|44||55|74|44||49|52|40||49|66|40|
|170|52|55|42||52|70|42||46|49|37||46|62|37|
|180|49|52|40||49|66|40||43|46|35||43|58|35|
|190|46|49|37||46|62|37||40|43|32||40|54|32|
|200|43|46|35||43|58|35||36|38|29||36|48|29|
|210|41|44|33||41|55|33||34|36|28||34|46|28|
|220|38|40|31||38|51|31||30|32|24||30|40|24|
|230|35|37|28||35|47|28||27|29|22||27|36|22|
|240|32|34|26||32|43|26||24|26|20||24|32|20|
|250|29|31|24||29|39|24||21|23|17||21|28|17|
|260|27|29|22||27|36|22||17|18|14||17|23|14|
|270|24|26|20||24|32|20||15|16|12||15|20|12|
|280|21|23|17||21|28|17||11|12|9||11|15|9|
|290|18|19|15||18|24|15||8|9|7||8|11|7|
|300|15|16|12||15|20|12||5|6|4||5|7|4|
|310|12|13|10||12|16|10||2|3|2||2|3|2|
|320|10|11|8||10|14|8||0|0|0||0|0|0|
|330|7|8|6||7|10|6||0|0|0||0|0|0|
|340|4|5|4||4|6|4||0|0|0||0|0|0|
|350|1|2|1||1|2|1||0|0|0||0|0|0|
|360|0|0|0||0|0|0||0|0|0||0|0|0|

### [Miolite Caves](https://wiki.melvoridle.com/index.php?title=Miolite_Caves), Auto Eat I

|❤️|⚔️|🏹|🧙||⚔️ ☠️|🏹 ☠️|🧙 ☠️||⚔️ 💍|🏹 💍|🧙 💍||⚔️ ☠️ 💍|🏹 ☠️ 💍|🧙 ☠️ 💍|
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|150|~~*100*~~|~~*85*~~|~~*85*~~||~~*100*~~|~~*100*~~|~~*85*~~||~~*100*~~|80|81||~~*100*~~|~~*100*~~|81|
|160|~~*100*~~|~~*84*~~|~~*84*~~||~~*100*~~|~~*100*~~|~~*84*~~||~~*100*~~|78|80||~~*100*~~|~~*99*~~|80|
|170|~~*100*~~|~~*82*~~|~~*83*~~||~~*100*~~|~~*100*~~|~~*83*~~||~~*100*~~|76|79||~~*100*~~|~~*96*~~|79|
|180|~~*100*~~|80|~~*82*~~||~~*100*~~|~~*100*~~|~~*82*~~||~~*100*~~|74|78||~~*100*~~|~~*94*~~|78|
|190|~~*100*~~|79|81||~~*100*~~|~~*100*~~|81||~~*100*~~|73|77||~~*100*~~|~~*92*~~|77|
|200|~~*100*~~|78|80||~~*100*~~|~~*99*~~|80||~~*100*~~|71|75||~~*100*~~|~~*90*~~|75|
|210|~~*100*~~|76|79||~~*100*~~|~~*96*~~|79||~~*100*~~|70|74||~~*100*~~|~~*88*~~|74|
|220|~~*100*~~|75|78||~~*100*~~|~~*95*~~|78||~~*100*~~|68|73||~~*100*~~|~~*86*~~|73|
|230|~~*100*~~|74|77||~~*100*~~|~~*94*~~|77||~~*100*~~|66|72||~~*100*~~|~~*83*~~|72|
|240|~~*100*~~|72|76||~~*100*~~|~~*91*~~|76||~~*100*~~|64|70||~~*100*~~|80|70|
|250|~~*100*~~|71|75||~~*100*~~|~~*90*~~|75||~~*100*~~|63|69||~~*100*~~|79|69|
|260|~~*100*~~|70|74||~~*100*~~|~~*88*~~|74||~~*100*~~|60|67||~~*100*~~|76|67|
|270|~~*100*~~|68|73||~~*100*~~|~~*86*~~|73||~~*100*~~|59|67||~~*100*~~|75|67|
|280|~~*100*~~|67|72||~~*100*~~|~~*84*~~|72||~~*100*~~|57|65||~~*100*~~|72|65|
|290|~~*100*~~|66|71||~~*100*~~|~~*83*~~|71||~~*100*~~|55|64||~~*100*~~|70|64|
|300|~~*100*~~|64|70||~~*100*~~|80|70||~~*100*~~|54|63||~~*100*~~|68|63|
|310|~~*100*~~|63|69||~~*100*~~|79|69||~~*100*~~|52|62||~~*100*~~|66|62|
|320|~~*100*~~|62|68||~~*100*~~|78|68||~~*100*~~|50|60||~~*100*~~|63|60|
|330|~~*100*~~|59|67||~~*100*~~|75|67||~~*100*~~|49|59||~~*100*~~|62|59|
|340|~~*100*~~|58|66||~~*100*~~|74|66||~~*100*~~|47|58||~~*100*~~|59|58|
|350|~~*100*~~|57|65||~~*100*~~|72|65||~~*100*~~|46|56||~~*100*~~|58|56|
|360|~~*100*~~|55|64||~~*100*~~|70|64||~~*100*~~|44|55||~~*100*~~|55|55|
|370|~~*100*~~|54|63||~~*100*~~|68|63||~~*100*~~|44|54||~~*100*~~|52|54|
|380|~~*100*~~|53|62||~~*100*~~|67|62||~~*100*~~|43|53||~~*100*~~|50|53|
|390|~~*100*~~|52|61||~~*100*~~|66|61||~~*100*~~|42|52||~~*100*~~|48|52|
|400|~~*100*~~|50|60||~~*100*~~|63|60||~~*100*~~|40|50||~~*100*~~|46|50|
|410|~~*100*~~|49|59||~~*100*~~|62|59||~~*98*~~|40|49||~~*100*~~|44|49|
|420|~~*100*~~|48|58||~~*100*~~|60|58||~~*96*~~|39|48||~~*100*~~|42|48|
|430|~~*100*~~|46|57||~~*100*~~|58|57||~~*94*~~|38|47||~~*100*~~|39|47|
|440|~~*100*~~|45|56||~~*100*~~|56|56||~~*90*~~|36|45||~~*100*~~|36|45|
|450|~~*100*~~|44|55||~~*100*~~|55|55||~~*88*~~|36|44||~~*100*~~|36|44|
|460|~~*100*~~|44|54||~~*100*~~|52|54||~~*86*~~|35|43||~~*100*~~|35|43|
|470|~~*100*~~|43|53||~~*100*~~|51|53||~~*84*~~|34|42||~~*100*~~|34|42|
|480|~~*100*~~|42|52||~~*100*~~|50|52||80|32|40||~~*100*~~|32|40|
|490|~~*100*~~|41|51||~~*100*~~|47|51||78|32|39||~~*100*~~|32|39|
|500|~~*100*~~|40|50||~~*100*~~|46|50||76|31|38||~~*100*~~|31|38|
|510|~~*98*~~|40|49||~~*100*~~|44|49||74|30|37||~~*100*~~|30|37|
|520|~~*96*~~|39|48||~~*100*~~|42|48||70|28|35||~~*100*~~|28|35|
|530|~~*94*~~|38|47||~~*100*~~|40|47||68|28|34||~~*100*~~|28|34|
|540|~~*92*~~|37|46||~~*100*~~|39|46||66|27|33||~~*100*~~|27|33|
|550|~~*90*~~|36|45||~~*100*~~|36|45||64|26|32||~~*100*~~|26|32|
|560|~~*88*~~|36|44||~~*100*~~|36|44||60|24|30||~~*100*~~|24|30|
|570|~~*86*~~|35|43||~~*100*~~|35|43||58|24|29||~~*100*~~|24|29|
|580|~~*84*~~|34|42||~~*100*~~|34|42||56|23|28||~~*100*~~|23|28|
|590|~~*82*~~|33|41||~~*100*~~|33|41||54|22|27||~~*100*~~|22|27|
|600|80|32|40||~~*100*~~|32|40||50|20|25||~~*100*~~|20|25|
|610|78|32|39||~~*100*~~|32|39||48|20|24||~~*96*~~|20|24|
|620|76|31|38||~~*100*~~|31|38||46|19|23||~~*92*~~|19|23|
|630|74|30|37||~~*100*~~|30|37||44|18|22||~~*88*~~|18|22|
|640|72|29|36||~~*100*~~|29|36||40|16|20||80|16|20|
|650|70|28|35||~~*100*~~|28|35||38|16|19||76|16|19|
|660|68|28|34||~~*100*~~|28|34||36|15|18||72|15|18|
|670|66|27|33||~~*100*~~|27|33||34|14|17||68|14|17|
|680|64|26|32||~~*100*~~|26|32||30|12|15||60|12|15|
|690|62|25|31||~~*100*~~|25|31||28|12|14||56|12|14|
|700|60|24|30||~~*100*~~|24|30||26|11|13||52|11|13|
|710|58|24|29||~~*100*~~|24|29||24|10|12||48|10|12|
|720|56|23|28||~~*100*~~|23|28||20|8|10||40|8|10|
|730|54|22|27||~~*100*~~|22|27||18|8|9||36|8|9|
|740|52|21|26||~~*100*~~|21|26||16|7|8||32|7|8|
|750|50|20|25||~~*100*~~|20|25||14|6|7||28|6|7|
|760|48|20|24||~~*96*~~|20|24||10|4|5||20|4|5|
|770|46|19|23||~~*92*~~|19|23||8|4|4||16|4|4|
|780|44|18|22||~~*88*~~|18|22||6|3|3||12|3|3|
|790|42|17|21||~~*84*~~|17|21||4|2|2||8|2|2|
|800|40|16|20||80|16|20||0|0|0||0|0|0|
|810|38|16|19||76|16|19||0|0|0||0|0|0|
|820|36|15|18||72|15|18||0|0|0||0|0|0|
|830|34|14|17||68|14|17||0|0|0||0|0|0|
|840|32|13|16||64|13|16||0|0|0||0|0|0|
|850|30|12|15||60|12|15||0|0|0||0|0|0|
|860|28|12|14||56|12|14||0|0|0||0|0|0|
|870|26|11|13||52|11|13||0|0|0||0|0|0|
|880|24|10|12||48|10|12||0|0|0||0|0|0|
|890|22|9|11||44|9|11||0|0|0||0|0|0|
|900|20|8|10||40|8|10||0|0|0||0|0|0|
|910|18|8|9||36|8|9||0|0|0||0|0|0|
|920|16|7|8||32|7|8||0|0|0||0|0|0|
|930|14|6|7||28|6|7||0|0|0||0|0|0|
|940|12|5|6||24|5|6||0|0|0||0|0|0|
|950|10|4|5||20|4|5||0|0|0||0|0|0|
|960|8|4|4||16|4|4||0|0|0||0|0|0|
|970|6|3|3||12|3|3||0|0|0||0|0|0|
|980|4|2|2||8|2|2||0|0|0||0|0|0|
|990|2|1|1||4|1|1||0|0|0||0|0|0|
|1000|0|0|0||0|0|0||0|0|0||0|0|0|

### [Miolite Caves](https://wiki.melvoridle.com/index.php?title=Miolite_Caves), Auto Eat II

|❤️|⚔️|🏹|🧙||⚔️ ☠️|🏹 ☠️|🧙 ☠️||⚔️ 💍|🏹 💍|🧙 💍||⚔️ ☠️ 💍|🏹 ☠️ 💍|🧙 ☠️ 💍|
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|110|~~*100*~~|~~*83*~~|~~*84*~~||~~*100*~~|~~*100*~~|~~*84*~~||~~*100*~~|79|81||~~*100*~~|~~*100*~~|81|
|120|~~*100*~~|80|~~*82*~~||~~*100*~~|~~*100*~~|~~*82*~~||~~*100*~~|76|79||~~*100*~~|~~*96*~~|79|
|130|~~*100*~~|78|80||~~*100*~~|~~*99*~~|80||~~*100*~~|74|78||~~*100*~~|~~*94*~~|78|
|140|~~*100*~~|76|79||~~*100*~~|~~*96*~~|79||~~*100*~~|72|76||~~*100*~~|~~*91*~~|76|
|150|~~*100*~~|74|78||~~*100*~~|~~*94*~~|78||~~*100*~~|70|74||~~*100*~~|~~*88*~~|74|
|160|~~*100*~~|72|76||~~*100*~~|~~*91*~~|76||~~*100*~~|67|72||~~*100*~~|~~*84*~~|72|
|170|~~*100*~~|70|75||~~*100*~~|~~*88*~~|75||~~*100*~~|65|71||~~*100*~~|~~*82*~~|71|
|180|~~*100*~~|68|73||~~*100*~~|~~*86*~~|73||~~*100*~~|63|69||~~*100*~~|79|69|
|190|~~*100*~~|66|72||~~*100*~~|~~*83*~~|72||~~*100*~~|59|67||~~*100*~~|75|67|
|200|~~*100*~~|64|70||~~*100*~~|80|70||~~*100*~~|57|65||~~*100*~~|72|65|
|210|~~*100*~~|62|68||~~*100*~~|78|68||~~*100*~~|55|64||~~*100*~~|70|64|
|220|~~*100*~~|59|67||~~*100*~~|75|67||~~*100*~~|52|62||~~*100*~~|66|62|
|230|~~*100*~~|57|66||~~*100*~~|72|66||~~*100*~~|50|60||~~*100*~~|63|60|
|240|~~*100*~~|55|64||~~*100*~~|70|64||~~*100*~~|48|58||~~*100*~~|60|58|
|250|~~*100*~~|54|63||~~*100*~~|68|63||~~*100*~~|46|56||~~*100*~~|58|56|
|260|~~*100*~~|52|61||~~*100*~~|66|61||~~*100*~~|44|55||~~*100*~~|54|55|
|270|~~*100*~~|50|60||~~*100*~~|63|60||~~*100*~~|43|53||~~*100*~~|51|53|
|280|~~*100*~~|48|58||~~*100*~~|60|58||~~*100*~~|41|51||~~*100*~~|47|51|
|290|~~*100*~~|46|56||~~*100*~~|58|56||~~*100*~~|40|50||~~*100*~~|44|50|
|300|~~*100*~~|44|55||~~*100*~~|55|55||~~*96*~~|39|48||~~*100*~~|42|48|
|310|~~*100*~~|44|54||~~*100*~~|52|54||~~*92*~~|37|46||~~*100*~~|39|46|
|320|~~*100*~~|42|52||~~*100*~~|50|52||~~*88*~~|36|44||~~*100*~~|36|44|
|330|~~*100*~~|41|51||~~*100*~~|47|51||~~*86*~~|35|43||~~*100*~~|35|43|
|340|~~*98*~~|40|49||~~*100*~~|44|49||~~*82*~~|33|41||~~*100*~~|33|41|
|350|~~*96*~~|39|48||~~*100*~~|42|48||78|32|39||~~*100*~~|32|39|
|360|~~*92*~~|37|46||~~*100*~~|39|46||76|31|38||~~*100*~~|31|38|
|370|~~*90*~~|36|45||~~*100*~~|36|45||72|29|36||~~*100*~~|29|36|
|380|~~*86*~~|35|43||~~*100*~~|35|43||68|28|34||~~*100*~~|28|34|
|390|~~*84*~~|34|42||~~*100*~~|34|42||64|26|32||~~*100*~~|26|32|
|400|80|32|40||~~*100*~~|32|40||60|24|30||~~*100*~~|24|30|
|410|78|32|39||~~*100*~~|32|39||58|24|29||~~*100*~~|24|29|
|420|74|30|37||~~*100*~~|30|37||54|22|27||~~*100*~~|22|27|
|430|72|29|36||~~*100*~~|29|36||50|20|25||~~*100*~~|20|25|
|440|68|28|34||~~*100*~~|28|34||46|19|23||~~*92*~~|19|23|
|450|66|27|33||~~*100*~~|27|33||44|18|22||~~*88*~~|18|22|
|460|62|25|31||~~*100*~~|25|31||40|16|20||80|16|20|
|470|60|24|30||~~*100*~~|24|30||36|15|18||72|15|18|
|480|56|23|28||~~*100*~~|23|28||32|13|16||64|13|16|
|490|54|22|27||~~*100*~~|22|27||30|12|15||60|12|15|
|500|50|20|25||~~*100*~~|20|25||26|11|13||52|11|13|
|510|48|20|24||~~*96*~~|20|24||22|9|11||44|9|11|
|520|44|18|22||~~*88*~~|18|22||18|8|9||36|8|9|
|530|42|17|21||~~*84*~~|17|21||16|7|8||32|7|8|
|540|38|16|19||76|16|19||12|5|6||24|5|6|
|550|36|15|18||72|15|18||8|4|4||16|4|4|
|560|32|13|16||64|13|16||4|2|2||8|2|2|
|570|30|12|15||60|12|15||2|1|1||4|1|1|
|580|26|11|13||52|11|13||0|0|0||0|0|0|
|590|24|10|12||48|10|12||0|0|0||0|0|0|
|600|20|8|10||40|8|10||0|0|0||0|0|0|
|610|18|8|9||36|8|9||0|0|0||0|0|0|
|620|14|6|7||28|6|7||0|0|0||0|0|0|
|630|12|5|6||24|5|6||0|0|0||0|0|0|
|640|8|4|4||16|4|4||0|0|0||0|0|0|
|650|6|3|3||12|3|3||0|0|0||0|0|0|
|660|2|1|1||4|1|1||0|0|0||0|0|0|
|670|0|0|0||0|0|0||0|0|0||0|0|0|

### [Miolite Caves](https://wiki.melvoridle.com/index.php?title=Miolite_Caves), Auto Eat III

|❤️|⚔️|🏹|🧙||⚔️ ☠️|🏹 ☠️|🧙 ☠️||⚔️ 💍|🏹 💍|🧙 💍||⚔️ ☠️ 💍|🏹 ☠️ 💍|🧙 ☠️ 💍|
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|100|~~*100*~~|78|80||~~*100*~~|~~*99*~~|80||~~*100*~~|74|78||~~*100*~~|~~*94*~~|78|
|110|~~*100*~~|75|78||~~*100*~~|~~*95*~~|78||~~*100*~~|72|76||~~*100*~~|~~*91*~~|76|
|120|~~*100*~~|72|76||~~*100*~~|~~*91*~~|76||~~*100*~~|68|73||~~*100*~~|~~*86*~~|73|
|130|~~*100*~~|70|74||~~*100*~~|~~*88*~~|74||~~*100*~~|66|71||~~*100*~~|~~*83*~~|71|
|140|~~*100*~~|67|72||~~*100*~~|~~*84*~~|72||~~*100*~~|62|68||~~*100*~~|78|68|
|150|~~*100*~~|64|70||~~*100*~~|80|70||~~*100*~~|59|67||~~*100*~~|75|67|
|160|~~*100*~~|62|68||~~*100*~~|78|68||~~*100*~~|55|64||~~*100*~~|70|64|
|170|~~*100*~~|58|66||~~*100*~~|74|66||~~*100*~~|53|62||~~*100*~~|67|62|
|180|~~*100*~~|55|64||~~*100*~~|70|64||~~*100*~~|50|60||~~*100*~~|63|60|
|190|~~*100*~~|53|62||~~*100*~~|67|62||~~*100*~~|47|58||~~*100*~~|59|58|
|200|~~*100*~~|50|60||~~*100*~~|63|60||~~*100*~~|44|55||~~*100*~~|55|55|
|210|~~*100*~~|48|58||~~*100*~~|60|58||~~*100*~~|43|53||~~*100*~~|51|53|
|220|~~*100*~~|45|56||~~*100*~~|56|56||~~*100*~~|41|51||~~*100*~~|47|51|
|230|~~*100*~~|44|54||~~*100*~~|52|54||~~*98*~~|40|49||~~*100*~~|43|49|
|240|~~*100*~~|42|52||~~*100*~~|50|52||~~*92*~~|37|46||~~*100*~~|39|46|
|250|~~*100*~~|40|50||~~*100*~~|46|50||~~*88*~~|36|44||~~*100*~~|36|44|
|260|~~*96*~~|39|48||~~*100*~~|42|48||~~*84*~~|34|42||~~*100*~~|34|42|
|270|~~*92*~~|37|46||~~*100*~~|39|46||80|32|40||~~*100*~~|32|40|
|280|~~*88*~~|36|44||~~*100*~~|36|44||74|30|37||~~*100*~~|30|37|
|290|~~*84*~~|34|42||~~*100*~~|34|42||70|28|35||~~*100*~~|28|35|
|300|80|32|40||~~*100*~~|32|40||66|27|33||~~*100*~~|27|33|
|310|76|31|38||~~*100*~~|31|38||62|25|31||~~*100*~~|25|31|
|320|72|29|36||~~*100*~~|29|36||56|23|28||~~*100*~~|23|28|
|330|68|28|34||~~*100*~~|28|34||52|21|26||~~*100*~~|21|26|
|340|64|26|32||~~*100*~~|26|32||48|20|24||~~*96*~~|20|24|
|350|60|24|30||~~*100*~~|24|30||44|18|22||~~*88*~~|18|22|
|360|56|23|28||~~*100*~~|23|28||38|16|19||76|16|19|
|370|52|21|26||~~*100*~~|21|26||34|14|17||68|14|17|
|380|48|20|24||~~*96*~~|20|24||30|12|15||60|12|15|
|390|44|18|22||~~*88*~~|18|22||26|11|13||52|11|13|
|400|40|16|20||80|16|20||20|8|10||40|8|10|
|410|36|15|18||72|15|18||16|7|8||32|7|8|
|420|32|13|16||64|13|16||12|5|6||24|5|6|
|430|28|12|14||56|12|14||8|4|4||16|4|4|
|440|24|10|12||48|10|12||2|1|1||4|1|1|
|450|20|8|10||40|8|10||0|0|0||0|0|0|
|460|16|7|8||32|7|8||0|0|0||0|0|0|
|470|12|5|6||24|5|6||0|0|0||0|0|0|
|480|8|4|4||16|4|4||0|0|0||0|0|0|
|490|4|2|2||8|2|2||0|0|0||0|0|0|
|500|0|0|0||0|0|0||0|0|0||0|0|0|

### [Deep Sea Ship](https://wiki.melvoridle.com/index.php?title=Deep_Sea_Ship), Auto Eat I

|❤️|⚔️|🏹|🧙||⚔️ ☠️|🏹 ☠️|🧙 ☠️||⚔️ 💍|🏹 💍|🧙 💍||⚔️ ☠️ 💍|🏹 ☠️ 💍|🧙 ☠️ 💍|
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|100|~~*90*~~|~~*95*~~|72||~~*90*~~|~~*100*~~|72||~~*88*~~|~~*93*~~|71||~~*88*~~|~~*100*~~|71|
|110|~~*89*~~|~~*94*~~|72||~~*89*~~|~~*100*~~|72||~~*87*~~|~~*92*~~|70||~~*87*~~|~~*100*~~|70|
|120|~~*88*~~|~~*93*~~|71||~~*88*~~|~~*100*~~|71||~~*85*~~|~~*90*~~|68||~~*85*~~|~~*100*~~|68|
|130|~~*87*~~|~~*92*~~|70||~~*87*~~|~~*100*~~|70||~~*84*~~|~~*89*~~|68||~~*84*~~|~~*100*~~|68|
|140|~~*86*~~|~~*91*~~|69||~~*86*~~|~~*100*~~|69||~~*83*~~|~~*88*~~|67||~~*83*~~|~~*100*~~|67|
|150|~~*85*~~|~~*90*~~|68||~~*85*~~|~~*100*~~|68||~~*82*~~|~~*87*~~|66||~~*82*~~|~~*100*~~|66|
|160|~~*84*~~|~~*89*~~|68||~~*84*~~|~~*100*~~|68||80|~~*85*~~|64||80|~~*100*~~|64|
|170|~~*83*~~|~~*88*~~|67||~~*83*~~|~~*100*~~|67||79|~~*84*~~|64||79|~~*100*~~|64|
|180|~~*82*~~|~~*87*~~|66||~~*82*~~|~~*100*~~|66||78|~~*83*~~|63||78|~~*100*~~|63|
|190|81|~~*86*~~|65||81|~~*100*~~|65||77|~~*82*~~|62||77|~~*100*~~|62|
|200|80|~~*85*~~|64||80|~~*100*~~|64||76|80|61||76|~~*100*~~|61|
|210|79|~~*84*~~|64||79|~~*100*~~|64||75|79|60||75|~~*100*~~|60|
|220|78|~~*83*~~|63||78|~~*100*~~|63||73|77|59||73|~~*98*~~|59|
|230|77|~~*82*~~|62||77|~~*100*~~|62||72|76|58||72|~~*96*~~|58|
|240|76|80|61||76|~~*100*~~|61||71|75|57||71|~~*95*~~|57|
|250|76|80|61||76|~~*100*~~|61||70|74|56||70|~~*94*~~|56|
|260|75|79|60||75|~~*100*~~|60||68|72|55||68|~~*91*~~|55|
|270|74|78|60||74|~~*99*~~|60||67|71|54||67|~~*90*~~|54|
|280|73|77|59||73|~~*98*~~|59||66|70|53||66|~~*88*~~|53|
|290|72|76|58||72|~~*96*~~|58||65|69|52||65|~~*87*~~|52|
|300|71|75|57||71|~~*95*~~|57||63|67|51||63|~~*84*~~|51|
|310|70|74|56||70|~~*94*~~|56||62|66|50||62|~~*83*~~|50|
|320|69|73|56||69|~~*92*~~|56||61|65|49||61|~~*82*~~|49|
|330|68|72|55||68|~~*91*~~|55||60|64|48||60|80|48|
|340|67|71|54||67|~~*90*~~|54||58|62|47||58|78|47|
|350|66|70|53||66|~~*88*~~|53||57|60|46||57|76|46|
|360|65|69|52||65|~~*87*~~|52||56|59|45||56|75|45|
|370|64|68|52||64|~~*86*~~|52||55|58|44||55|74|44|
|380|63|67|51||63|~~*84*~~|51||53|56|43||53|71|43|
|390|62|66|50||62|~~*83*~~|50||52|55|42||52|70|42|
|400|61|65|49||61|~~*82*~~|49||51|54|41||51|68|41|
|410|60|64|48||60|80|48||50|53|40||50|67|40|
|420|59|63|48||59|79|48||49|52|40||49|66|40|
|430|58|62|47||58|78|47||48|51|39||48|64|39|
|440|57|60|46||57|76|46||46|49|37||46|62|37|
|450|56|59|45||56|75|45||45|48|36||45|60|36|
|460|55|58|44||55|74|44||44|47|36||44|59|36|
|470|54|57|44||54|72|44||43|46|35||43|58|35|
|480|53|56|43||53|71|43||41|44|33||41|55|33|
|490|52|55|42||52|70|42||40|43|32||40|54|32|
|500|51|54|41||51|68|41||39|42|32||39|52|32|
|510|50|53|40||50|67|40||38|40|31||38|51|31|
|520|49|52|40||49|66|40||36|38|29||36|48|29|
|530|48|51|39||48|64|39||35|37|28||35|47|28|
|540|47|50|38||47|63|38||34|36|28||34|46|28|
|550|46|49|37||46|62|37||33|35|27||33|44|27|
|560|45|48|36||45|60|36||31|33|25||31|42|25|
|570|44|47|36||44|59|36||30|32|24||30|40|24|
|580|43|46|35||43|58|35||29|31|24||29|39|24|
|590|42|45|34||42|56|34||28|30|23||28|38|23|
|600|41|44|33||41|55|33||26|28|21||26|35|21|
|610|40|43|32||40|54|32||26|28|21||26|35|21|
|620|39|42|32||39|52|32||24|26|20||24|32|20|
|630|38|40|31||38|51|31||23|25|19||23|31|19|
|640|37|39|30||37|50|30||22|24|18||22|30|18|
|650|36|38|29||36|48|29||21|23|17||21|28|17|
|660|35|37|28||35|47|28||19|20|16||19|26|16|
|670|34|36|28||34|46|28||18|19|15||18|24|15|
|680|33|35|27||33|44|27||17|18|14||17|23|14|
|690|32|34|26||32|43|26||16|17|13||16|22|13|
|700|31|33|25||31|42|25||14|15|12||14|19|12|
|710|30|32|24||30|40|24||13|14|11||13|18|11|
|720|29|31|24||29|39|24||12|13|10||12|16|10|
|730|28|30|23||28|38|23||11|12|9||11|15|9|
|740|27|29|22||27|36|22||9|10|8||9|12|8|
|750|26|28|21||26|35|21||8|9|7||8|11|7|
|760|26|28|21||26|35|21||7|8|6||7|10|6|
|770|25|27|20||25|34|20||6|7|5||6|8|5|
|780|24|26|20||24|32|20||4|5|4||4|6|4|
|790|23|25|19||23|31|19||3|4|3||3|4|3|
|800|22|24|18||22|30|18||2|3|2||2|3|2|
|810|21|23|17||21|28|17||1|2|1||1|2|1|
|820|20|22|16||20|27|16||0|0|0||0|0|0|
|830|19|20|16||19|26|16||0|0|0||0|0|0|
|840|18|19|15||18|24|15||0|0|0||0|0|0|
|850|17|18|14||17|23|14||0|0|0||0|0|0|
|860|16|17|13||16|22|13||0|0|0||0|0|0|
|870|15|16|12||15|20|12||0|0|0||0|0|0|
|880|14|15|12||14|19|12||0|0|0||0|0|0|
|890|13|14|11||13|18|11||0|0|0||0|0|0|
|900|12|13|10||12|16|10||0|0|0||0|0|0|
|910|11|12|9||11|15|9||0|0|0||0|0|0|
|920|10|11|8||10|14|8||0|0|0||0|0|0|
|930|9|10|8||9|12|8||0|0|0||0|0|0|
|940|8|9|7||8|11|7||0|0|0||0|0|0|
|950|7|8|6||7|10|6||0|0|0||0|0|0|
|960|6|7|5||6|8|5||0|0|0||0|0|0|
|970|5|6|4||5|7|4||0|0|0||0|0|0|
|980|4|5|4||4|6|4||0|0|0||0|0|0|
|990|3|4|3||3|4|3||0|0|0||0|0|0|
|1000|2|3|2||2|3|2||0|0|0||0|0|0|
|1010|1|2|1||1|2|1||0|0|0||0|0|0|
|1020|0|0|0||0|0|0||0|0|0||0|0|0|

### [Deep Sea Ship](https://wiki.melvoridle.com/index.php?title=Deep_Sea_Ship), Auto Eat II

|❤️|⚔️|🏹|🧙||⚔️ ☠️|🏹 ☠️|🧙 ☠️||⚔️ 💍|🏹 💍|🧙 💍||⚔️ ☠️ 💍|🏹 ☠️ 💍|🧙 ☠️ 💍|
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|100|~~*85*~~|~~*90*~~|68||~~*85*~~|~~*100*~~|68||~~*83*~~|~~*88*~~|67||~~*83*~~|~~*100*~~|67|
|110|~~*84*~~|~~*89*~~|68||~~*84*~~|~~*100*~~|68||81|~~*86*~~|65||81|~~*100*~~|65|
|120|~~*82*~~|~~*87*~~|66||~~*82*~~|~~*100*~~|66||79|~~*84*~~|64||79|~~*100*~~|64|
|130|81|~~*86*~~|65||81|~~*100*~~|65||78|~~*83*~~|63||78|~~*100*~~|63|
|140|79|~~*84*~~|64||79|~~*100*~~|64||76|80|61||76|~~*100*~~|61|
|150|78|~~*83*~~|63||78|~~*100*~~|63||75|79|60||75|~~*100*~~|60|
|160|76|80|61||76|~~*100*~~|61||73|77|59||73|~~*98*~~|59|
|170|75|79|60||75|~~*100*~~|60||71|75|57||71|~~*95*~~|57|
|180|74|78|60||74|~~*99*~~|60||70|74|56||70|~~*94*~~|56|
|190|72|76|58||72|~~*96*~~|58||68|72|55||68|~~*91*~~|55|
|200|71|75|57||71|~~*95*~~|57||66|70|53||66|~~*88*~~|53|
|210|69|73|56||69|~~*92*~~|56||64|68|52||64|~~*86*~~|52|
|220|68|72|55||68|~~*91*~~|55||62|66|50||62|~~*83*~~|50|
|230|66|70|53||66|~~*88*~~|53||61|65|49||61|~~*82*~~|49|
|240|65|69|52||65|~~*87*~~|52||59|63|48||59|79|48|
|250|63|67|51||63|~~*84*~~|51||57|60|46||57|76|46|
|260|62|66|50||62|~~*83*~~|50||55|58|44||55|74|44|
|270|60|64|48||60|80|48||54|57|44||54|72|44|
|280|59|63|48||59|79|48||52|55|42||52|70|42|
|290|57|60|46||57|76|46||51|54|41||51|68|41|
|300|56|59|45||56|75|45||49|52|40||49|66|40|
|310|54|57|44||54|72|44||47|50|38||47|63|38|
|320|53|56|43||53|71|43||45|48|36||45|60|36|
|330|51|54|41||51|68|41||44|47|36||44|59|36|
|340|50|53|40||50|67|40||42|45|34||42|56|34|
|350|49|52|40||49|66|40||40|43|32||40|54|32|
|360|47|50|38||47|63|38||39|42|32||39|52|32|
|370|46|49|37||46|62|37||37|39|30||37|50|30|
|380|44|47|36||44|59|36||35|37|28||35|47|28|
|390|43|46|35||43|58|35||33|35|27||33|44|27|
|400|41|44|33||41|55|33||31|33|25||31|42|25|
|410|40|43|32||40|54|32||30|32|24||30|40|24|
|420|38|40|31||38|51|31||28|30|23||28|38|23|
|430|37|39|30||37|50|30||26|28|21||26|35|21|
|440|35|37|28||35|47|28||25|27|20||25|34|20|
|450|34|36|28||34|46|28||23|25|19||23|31|19|
|460|32|34|26||32|43|26||21|23|17||21|28|17|
|470|31|33|25||31|42|25||20|22|16||20|27|16|
|480|29|31|24||29|39|24||18|19|15||18|24|15|
|490|28|30|23||28|38|23||16|17|13||16|22|13|
|500|26|28|21||26|35|21||14|15|12||14|19|12|
|510|25|27|20||25|34|20||13|14|11||13|18|11|
|520|24|26|20||24|32|20||11|12|9||11|15|9|
|530|22|24|18||22|30|18||9|10|8||9|12|8|
|540|21|23|17||21|28|17||7|8|6||7|10|6|
|550|19|20|16||19|26|16||6|7|5||6|8|5|
|560|18|19|15||18|24|15||4|5|4||4|6|4|
|570|16|17|13||16|22|13||2|3|2||2|3|2|
|580|15|16|12||15|20|12||1|2|1||1|2|1|
|590|13|14|11||13|18|11||0|0|0||0|0|0|
|600|12|13|10||12|16|10||0|0|0||0|0|0|
|610|10|11|8||10|14|8||0|0|0||0|0|0|
|620|9|10|8||9|12|8||0|0|0||0|0|0|
|630|7|8|6||7|10|6||0|0|0||0|0|0|
|640|6|7|5||6|8|5||0|0|0||0|0|0|
|650|4|5|4||4|6|4||0|0|0||0|0|0|
|660|3|4|3||3|4|3||0|0|0||0|0|0|
|670|1|2|1||1|2|1||0|0|0||0|0|0|
|680|0|0|0||0|0|0||0|0|0||0|0|0|

### [Deep Sea Ship](https://wiki.melvoridle.com/index.php?title=Deep_Sea_Ship), Auto Eat III

|❤️|⚔️|🏹|🧙||⚔️ ☠️|🏹 ☠️|🧙 ☠️||⚔️ 💍|🏹 💍|🧙 💍||⚔️ ☠️ 💍|🏹 ☠️ 💍|🧙 ☠️ 💍|
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|100|80|~~*85*~~|64||80|~~*100*~~|64||78|~~*83*~~|63||78|~~*100*~~|63|
|110|78|~~*83*~~|63||78|~~*100*~~|63||76|80|61||76|~~*100*~~|61|
|120|76|80|61||76|~~*100*~~|61||74|78|60||74|~~*99*~~|60|
|130|75|79|60||75|~~*100*~~|60||72|76|58||72|~~*96*~~|58|
|140|73|77|59||73|~~*98*~~|59||69|73|56||69|~~*92*~~|56|
|150|71|75|57||71|~~*95*~~|57||67|71|54||67|~~*90*~~|54|
|160|69|73|56||69|~~*92*~~|56||65|69|52||65|~~*87*~~|52|
|170|67|71|54||67|~~*90*~~|54||63|67|51||63|~~*84*~~|51|
|180|65|69|52||65|~~*87*~~|52||60|64|48||60|80|48|
|190|63|67|51||63|~~*84*~~|51||58|62|47||58|78|47|
|200|61|65|49||61|~~*82*~~|49||56|59|45||56|75|45|
|210|59|63|48||59|79|48||54|57|44||54|72|44|
|220|57|60|46||57|76|46||51|54|41||51|68|41|
|230|55|58|44||55|74|44||50|53|40||50|67|40|
|240|53|56|43||53|71|43||47|50|38||47|63|38|
|250|51|54|41||51|68|41||45|48|36||45|60|36|
|260|49|52|40||49|66|40||43|46|35||43|58|35|
|270|47|50|38||47|63|38||41|44|33||41|55|33|
|280|45|48|36||45|60|36||38|40|31||38|51|31|
|290|43|46|35||43|58|35||36|38|29||36|48|29|
|300|41|44|33||41|55|33||34|36|28||34|46|28|
|310|39|42|32||39|52|32||32|34|26||32|43|26|
|320|37|39|30||37|50|30||29|31|24||29|39|24|
|330|35|37|28||35|47|28||27|29|22||27|36|22|
|340|33|35|27||33|44|27||25|27|20||25|34|20|
|350|31|33|25||31|42|25||23|25|19||23|31|19|
|360|29|31|24||29|39|24||21|23|17||21|28|17|
|370|27|29|22||27|36|22||19|20|16||19|26|16|
|380|26|28|21||26|35|21||16|17|13||16|22|13|
|390|24|26|20||24|32|20||14|15|12||14|19|12|
|400|22|24|18||22|30|18||12|13|10||12|16|10|
|410|20|22|16||20|27|16||10|11|8||10|14|8|
|420|18|19|15||18|24|15||7|8|6||7|10|6|
|430|16|17|13||16|22|13||5|6|4||5|7|4|
|440|14|15|12||14|19|12||3|4|3||3|4|3|
|450|12|13|10||12|16|10||1|2|1||1|2|1|
|460|10|11|8||10|14|8||0|0|0||0|0|0|
|470|8|9|7||8|11|7||0|0|0||0|0|0|
|480|6|7|5||6|8|5||0|0|0||0|0|0|
|490|4|5|4||4|6|4||0|0|0||0|0|0|
|500|2|3|2||2|3|2||0|0|0||0|0|0|
|510|0|0|0||0|0|0||0|0|0||0|0|0|

### [Frozen Cove](https://wiki.melvoridle.com/index.php?title=Frozen_Cove), Auto Eat I

|❤️|⚔️|🏹|🧙||⚔️ ☠️|🏹 ☠️|🧙 ☠️||⚔️ 💍|🏹 💍|🧙 💍||⚔️ ☠️ 💍|🏹 ☠️ 💍|🧙 ☠️ 💍|
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|100|~~*88*~~|~~*93*~~|71||~~*88*~~|~~*100*~~|71||~~*85*~~|~~*90*~~|68||~~*85*~~|~~*100*~~|68|
|110|~~*87*~~|~~*92*~~|70||~~*87*~~|~~*100*~~|70||~~*84*~~|~~*89*~~|68||~~*84*~~|~~*100*~~|68|
|120|~~*86*~~|~~*91*~~|69||~~*86*~~|~~*100*~~|69||~~*82*~~|~~*87*~~|66||~~*82*~~|~~*100*~~|66|
|130|~~*85*~~|~~*90*~~|68||~~*85*~~|~~*100*~~|68||81|~~*86*~~|65||81|~~*100*~~|65|
|140|~~*84*~~|~~*89*~~|68||~~*84*~~|~~*100*~~|68||80|~~*85*~~|64||80|~~*100*~~|64|
|150|~~*82*~~|~~*87*~~|66||~~*82*~~|~~*100*~~|66||78|~~*83*~~|63||78|~~*100*~~|63|
|160|81|~~*86*~~|65||81|~~*100*~~|65||77|~~*82*~~|62||77|~~*100*~~|62|
|170|80|~~*85*~~|64||80|~~*100*~~|64||76|80|61||76|~~*100*~~|61|
|180|79|~~*84*~~|64||79|~~*100*~~|64||74|78|60||74|~~*99*~~|60|
|190|78|~~*83*~~|63||78|~~*100*~~|63||73|77|59||73|~~*98*~~|59|
|200|77|~~*82*~~|62||77|~~*100*~~|62||71|75|57||71|~~*95*~~|57|
|210|76|80|61||76|~~*100*~~|61||70|74|56||70|~~*94*~~|56|
|220|74|78|60||74|~~*99*~~|60||68|72|55||68|~~*91*~~|55|
|230|73|77|59||73|~~*98*~~|59||67|71|54||67|~~*90*~~|54|
|240|72|76|58||72|~~*96*~~|58||65|69|52||65|~~*87*~~|52|
|250|71|75|57||71|~~*95*~~|57||64|68|52||64|~~*86*~~|52|
|260|70|74|56||70|~~*94*~~|56||62|66|50||62|~~*83*~~|50|
|270|69|73|56||69|~~*92*~~|56||61|65|49||61|~~*82*~~|49|
|280|67|71|54||67|~~*90*~~|54||59|63|48||59|79|48|
|290|66|70|53||66|~~*88*~~|53||58|62|47||58|78|47|
|300|65|69|52||65|~~*87*~~|52||56|59|45||56|75|45|
|310|64|68|52||64|~~*86*~~|52||55|58|44||55|74|44|
|320|63|67|51||63|~~*84*~~|51||53|56|43||53|71|43|
|330|62|66|50||62|~~*83*~~|50||52|55|42||52|70|42|
|340|60|64|48||60|80|48||51|54|41||51|68|41|
|350|59|63|48||59|79|48||49|52|40||49|66|40|
|360|58|62|47||58|78|47||48|51|39||48|64|39|
|370|57|60|46||57|76|46||46|49|37||46|62|37|
|380|56|59|45||56|75|45||45|48|36||45|60|36|
|390|55|58|44||55|74|44||44|47|36||44|59|36|
|400|53|56|43||53|71|43||42|45|34||42|56|34|
|410|52|55|42||52|70|42||41|44|33||41|55|33|
|420|51|54|41||51|68|41||39|42|32||39|52|32|
|430|50|53|40||50|67|40||38|40|31||38|51|31|
|440|49|52|40||49|66|40||36|38|29||36|48|29|
|450|48|51|39||48|64|39||35|37|28||35|47|28|
|460|46|49|37||46|62|37||33|35|27||33|44|27|
|470|45|48|36||45|60|36||32|34|26||32|43|26|
|480|44|47|36||44|59|36||30|32|24||30|40|24|
|490|43|46|35||43|58|35||29|31|24||29|39|24|
|500|42|45|34||42|56|34||27|29|22||27|36|22|
|510|41|44|33||41|55|33||26|28|21||26|35|21|
|520|39|42|32||39|52|32||24|26|20||24|32|20|
|530|38|40|31||38|51|31||23|25|19||23|31|19|
|540|37|39|30||37|50|30||21|23|17||21|28|17|
|550|36|38|29||36|48|29||20|22|16||20|27|16|
|560|35|37|28||35|47|28||19|20|16||19|26|16|
|570|34|36|28||34|46|28||17|18|14||17|23|14|
|580|32|34|26||32|43|26||16|17|13||16|22|13|
|590|31|33|25||31|42|25||14|15|12||14|19|12|
|600|30|32|24||30|40|24||13|14|11||13|18|11|
|610|29|31|24||29|39|24||12|13|10||12|16|10|
|620|28|30|23||28|38|23||10|11|8||10|14|8|
|630|27|29|22||27|36|22||9|10|8||9|12|8|
|640|26|28|21||26|35|21||7|8|6||7|10|6|
|650|24|26|20||24|32|20||6|7|5||6|8|5|
|660|23|25|19||23|31|19||4|5|4||4|6|4|
|670|22|24|18||22|30|18||3|4|3||3|4|3|
|680|21|23|17||21|28|17||1|2|1||1|2|1|
|690|20|22|16||20|27|16||0|0|0||0|0|0|
|700|19|20|16||19|26|16||0|0|0||0|0|0|
|710|17|18|14||17|23|14||0|0|0||0|0|0|
|720|16|17|13||16|22|13||0|0|0||0|0|0|
|730|15|16|12||15|20|12||0|0|0||0|0|0|
|740|14|15|12||14|19|12||0|0|0||0|0|0|
|750|13|14|11||13|18|11||0|0|0||0|0|0|
|760|12|13|10||12|16|10||0|0|0||0|0|0|
|770|10|11|8||10|14|8||0|0|0||0|0|0|
|780|9|10|8||9|12|8||0|0|0||0|0|0|
|790|8|9|7||8|11|7||0|0|0||0|0|0|
|800|7|8|6||7|10|6||0|0|0||0|0|0|
|810|6|7|5||6|8|5||0|0|0||0|0|0|
|820|5|6|4||5|7|4||0|0|0||0|0|0|
|830|3|4|3||3|4|3||0|0|0||0|0|0|
|840|2|3|2||2|3|2||0|0|0||0|0|0|
|850|1|2|1||1|2|1||0|0|0||0|0|0|
|860|0|0|0||0|0|0||0|0|0||0|0|0|

### [Frozen Cove](https://wiki.melvoridle.com/index.php?title=Frozen_Cove), Auto Eat II

|❤️|⚔️|🏹|🧙||⚔️ ☠️|🏹 ☠️|🧙 ☠️||⚔️ 💍|🏹 💍|🧙 💍||⚔️ ☠️ 💍|🏹 ☠️ 💍|🧙 ☠️ 💍|
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|100|~~*82*~~|~~*87*~~|66||~~*82*~~|~~*100*~~|66||80|~~*85*~~|64||80|~~*100*~~|64|
|110|81|~~*86*~~|65||81|~~*100*~~|65||78|~~*83*~~|63||78|~~*100*~~|63|
|120|79|~~*84*~~|64||79|~~*100*~~|64||76|80|61||76|~~*100*~~|61|
|130|77|~~*82*~~|62||77|~~*100*~~|62||74|78|60||74|~~*99*~~|60|
|140|76|80|61||76|~~*100*~~|61||71|75|57||71|~~*95*~~|57|
|150|74|78|60||74|~~*99*~~|60||70|74|56||70|~~*94*~~|56|
|160|72|76|58||72|~~*96*~~|58||67|71|54||67|~~*90*~~|54|
|170|70|74|56||70|~~*94*~~|56||66|70|53||66|~~*88*~~|53|
|180|69|73|56||69|~~*92*~~|56||64|68|52||64|~~*86*~~|52|
|190|67|71|54||67|~~*90*~~|54||62|66|50||62|~~*83*~~|50|
|200|65|69|52||65|~~*87*~~|52||59|63|48||59|79|48|
|210|63|67|51||63|~~*84*~~|51||57|60|46||57|76|46|
|220|62|66|50||62|~~*83*~~|50||55|58|44||55|74|44|
|230|60|64|48||60|80|48||53|56|43||53|71|43|
|240|58|62|47||58|78|47||51|54|41||51|68|41|
|250|56|59|45||56|75|45||49|52|40||49|66|40|
|260|55|58|44||55|74|44||47|50|38||47|63|38|
|270|53|56|43||53|71|43||45|48|36||45|60|36|
|280|51|54|41||51|68|41||43|46|35||43|58|35|
|290|49|52|40||49|66|40||41|44|33||41|55|33|
|300|48|51|39||48|64|39||39|42|32||39|52|32|
|310|46|49|37||46|62|37||37|39|30||37|50|30|
|320|44|47|36||44|59|36||35|37|28||35|47|28|
|330|42|45|34||42|56|34||33|35|27||33|44|27|
|340|41|44|33||41|55|33||31|33|25||31|42|25|
|350|39|42|32||39|52|32||29|31|24||29|39|24|
|360|37|39|30||37|50|30||27|29|22||27|36|22|
|370|35|37|28||35|47|28||25|27|20||25|34|20|
|380|34|36|28||34|46|28||23|25|19||23|31|19|
|390|32|34|26||32|43|26||21|23|17||21|28|17|
|400|30|32|24||30|40|24||19|20|16||19|26|16|
|410|28|30|23||28|38|23||17|18|14||17|23|14|
|420|27|29|22||27|36|22||14|15|12||14|19|12|
|430|25|27|20||25|34|20||13|14|11||13|18|11|
|440|23|25|19||23|31|19||10|11|8||10|14|8|
|450|21|23|17||21|28|17||9|10|8||9|12|8|
|460|20|22|16||20|27|16||6|7|5||6|8|5|
|470|18|19|15||18|24|15||5|6|4||5|7|4|
|480|16|17|13||16|22|13||2|3|2||2|3|2|
|490|14|15|12||14|19|12||1|2|1||1|2|1|
|500|13|14|11||13|18|11||0|0|0||0|0|0|
|510|11|12|9||11|15|9||0|0|0||0|0|0|
|520|9|10|8||9|12|8||0|0|0||0|0|0|
|530|7|8|6||7|10|6||0|0|0||0|0|0|
|540|6|7|5||6|8|5||0|0|0||0|0|0|
|550|4|5|4||4|6|4||0|0|0||0|0|0|
|560|2|3|2||2|3|2||0|0|0||0|0|0|
|570|1|2|1||1|2|1||0|0|0||0|0|0|
|580|0|0|0||0|0|0||0|0|0||0|0|0|

### [Frozen Cove](https://wiki.melvoridle.com/index.php?title=Frozen_Cove), Auto Eat III

|❤️|⚔️|🏹|🧙||⚔️ ☠️|🏹 ☠️|🧙 ☠️||⚔️ 💍|🏹 💍|🧙 💍||⚔️ ☠️ 💍|🏹 ☠️ 💍|🧙 ☠️ 💍|
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|100|77|~~*82*~~|62||77|~~*100*~~|62||74|78|60||74|~~*99*~~|60|
|110|74|78|60||74|~~*99*~~|60||71|75|57||71|~~*95*~~|57|
|120|72|76|58||72|~~*96*~~|58||69|73|56||69|~~*92*~~|56|
|130|70|74|56||70|~~*94*~~|56||66|70|53||66|~~*88*~~|53|
|140|67|71|54||67|~~*90*~~|54||63|67|51||63|~~*84*~~|51|
|150|65|69|52||65|~~*87*~~|52||61|65|49||61|~~*82*~~|49|
|160|63|67|51||63|~~*84*~~|51||58|62|47||58|78|47|
|170|60|64|48||60|80|48||56|59|45||56|75|45|
|180|58|62|47||58|78|47||53|56|43||53|71|43|
|190|56|59|45||56|75|45||51|54|41||51|68|41|
|200|53|56|43||53|71|43||48|51|39||48|64|39|
|210|51|54|41||51|68|41||45|48|36||45|60|36|
|220|49|52|40||49|66|40||42|45|34||42|56|34|
|230|46|49|37||46|62|37||40|43|32||40|54|32|
|240|44|47|36||44|59|36||37|39|30||37|50|30|
|250|42|45|34||42|56|34||35|37|28||35|47|28|
|260|39|42|32||39|52|32||32|34|26||32|43|26|
|270|37|39|30||37|50|30||30|32|24||30|40|24|
|280|35|37|28||35|47|28||27|29|22||27|36|22|
|290|32|34|26||32|43|26||24|26|20||24|32|20|
|300|30|32|24||30|40|24||21|23|17||21|28|17|
|310|28|30|23||28|38|23||19|20|16||19|26|16|
|320|26|28|21||26|35|21||16|17|13||16|22|13|
|330|23|25|19||23|31|19||14|15|12||14|19|12|
|340|21|23|17||21|28|17||11|12|9||11|15|9|
|350|19|20|16||19|26|16||9|10|8||9|12|8|
|360|16|17|13||16|22|13||6|7|5||6|8|5|
|370|14|15|12||14|19|12||3|4|3||3|4|3|
|380|12|13|10||12|16|10||1|2|1||1|2|1|
|390|9|10|8||9|12|8||0|0|0||0|0|0|
|400|7|8|6||7|10|6||0|0|0||0|0|0|
|410|5|6|4||5|7|4||0|0|0||0|0|0|
|420|2|3|2||2|3|2||0|0|0||0|0|0|
|430|0|0|0||0|0|0||0|0|0||0|0|0|

### [Dragons Den](https://wiki.melvoridle.com/index.php?title=Dragons_Den), Auto Eat I

|❤️|⚔️|🏹|🧙||⚔️ ☠️|🏹 ☠️|🧙 ☠️||⚔️ 💍|🏹 💍|🧙 💍||⚔️ ☠️ 💍|🏹 ☠️ 💍|🧙 ☠️ 💍|
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|100|~~*96*~~|~~*100*~~|77||~~*96*~~|~~*100*~~|77||~~*95*~~|~~*100*~~|76||~~*95*~~|~~*100*~~|76|
|120|~~*95*~~|~~*100*~~|76||~~*95*~~|~~*100*~~|76||~~*94*~~|~~*99*~~|76||~~*94*~~|~~*100*~~|76|
|130|~~*95*~~|~~*100*~~|76||~~*95*~~|~~*100*~~|76||~~*93*~~|~~*98*~~|75||~~*93*~~|~~*100*~~|75|
|140|~~*94*~~|~~*99*~~|76||~~*94*~~|~~*100*~~|76||~~*93*~~|~~*98*~~|75||~~*93*~~|~~*100*~~|75|
|150|~~*94*~~|~~*99*~~|76||~~*94*~~|~~*100*~~|76||~~*92*~~|~~*97*~~|74||~~*92*~~|~~*100*~~|74|
|160|~~*93*~~|~~*98*~~|75||~~*93*~~|~~*100*~~|75||~~*92*~~|~~*97*~~|74||~~*92*~~|~~*100*~~|74|
|170|~~*93*~~|~~*98*~~|75||~~*93*~~|~~*100*~~|75||~~*91*~~|~~*96*~~|73||~~*91*~~|~~*100*~~|73|
|190|~~*92*~~|~~*97*~~|74||~~*92*~~|~~*100*~~|74||~~*90*~~|~~*95*~~|72||~~*90*~~|~~*100*~~|72|
|210|~~*91*~~|~~*96*~~|73||~~*91*~~|~~*100*~~|73||~~*89*~~|~~*94*~~|72||~~*89*~~|~~*100*~~|72|
|230|~~*90*~~|~~*95*~~|72||~~*90*~~|~~*100*~~|72||~~*88*~~|~~*93*~~|71||~~*88*~~|~~*100*~~|71|
|250|~~*90*~~|~~*95*~~|72||~~*90*~~|~~*100*~~|72||~~*87*~~|~~*92*~~|70||~~*87*~~|~~*100*~~|70|
|260|~~*89*~~|~~*94*~~|72||~~*89*~~|~~*100*~~|72||~~*86*~~|~~*91*~~|69||~~*86*~~|~~*100*~~|69|
|280|~~*88*~~|~~*93*~~|71||~~*88*~~|~~*100*~~|71||~~*85*~~|~~*90*~~|68||~~*85*~~|~~*100*~~|68|
|300|~~*88*~~|~~*93*~~|71||~~*88*~~|~~*100*~~|71||~~*84*~~|~~*89*~~|68||~~*84*~~|~~*100*~~|68|
|310|~~*87*~~|~~*92*~~|70||~~*87*~~|~~*100*~~|70||~~*84*~~|~~*89*~~|68||~~*84*~~|~~*100*~~|68|
|320|~~*87*~~|~~*92*~~|70||~~*87*~~|~~*100*~~|70||~~*83*~~|~~*88*~~|67||~~*83*~~|~~*100*~~|67|
|330|~~*86*~~|~~*91*~~|69||~~*86*~~|~~*100*~~|69||~~*83*~~|~~*88*~~|67||~~*83*~~|~~*100*~~|67|
|340|~~*86*~~|~~*91*~~|69||~~*86*~~|~~*100*~~|69||~~*82*~~|~~*87*~~|66||~~*82*~~|~~*100*~~|66|
|350|~~*85*~~|~~*90*~~|68||~~*85*~~|~~*100*~~|68||~~*82*~~|~~*87*~~|66||~~*82*~~|~~*100*~~|66|
|360|~~*85*~~|~~*90*~~|68||~~*85*~~|~~*100*~~|68||81|~~*86*~~|65||81|~~*100*~~|65|
|380|~~*84*~~|~~*89*~~|68||~~*84*~~|~~*100*~~|68||80|~~*85*~~|64||80|~~*100*~~|64|
|400|~~*83*~~|~~*88*~~|67||~~*83*~~|~~*100*~~|67||79|~~*84*~~|64||79|~~*100*~~|64|
|420|~~*82*~~|~~*87*~~|66||~~*82*~~|~~*100*~~|66||78|~~*83*~~|63||78|~~*100*~~|63|
|440|~~*82*~~|~~*87*~~|66||~~*82*~~|~~*100*~~|66||77|~~*82*~~|62||77|~~*100*~~|62|
|450|81|~~*86*~~|65||81|~~*100*~~|65||76|80|61||76|~~*100*~~|61|
|470|80|~~*85*~~|64||80|~~*100*~~|64||75|79|60||75|~~*100*~~|60|
|490|79|~~*84*~~|64||79|~~*100*~~|64||74|78|60||74|~~*99*~~|60|
|510|79|~~*84*~~|64||79|~~*100*~~|64||73|77|59||73|~~*98*~~|59|
|520|78|~~*83*~~|63||78|~~*100*~~|63||73|77|59||73|~~*98*~~|59|
|530|78|~~*83*~~|63||78|~~*100*~~|63||72|76|58||72|~~*96*~~|58|
|540|77|~~*82*~~|62||77|~~*100*~~|62||72|76|58||72|~~*96*~~|58|
|550|77|~~*82*~~|62||77|~~*100*~~|62||71|75|57||71|~~*95*~~|57|
|560|76|80|61||76|~~*100*~~|61||71|75|57||71|~~*95*~~|57|
|570|76|80|61||76|~~*100*~~|61||70|74|56||70|~~*94*~~|56|
|580|76|80|61||76|~~*100*~~|61||69|73|56||69|~~*92*~~|56|
|590|75|79|60||75|~~*100*~~|60||69|73|56||69|~~*92*~~|56|
|600|75|79|60||75|~~*100*~~|60||68|72|55||68|~~*91*~~|55|
|610|74|78|60||74|~~*99*~~|60||68|72|55||68|~~*91*~~|55|
|620|74|78|60||74|~~*99*~~|60||67|71|54||67|~~*90*~~|54|
|630|73|77|59||73|~~*98*~~|59||67|71|54||67|~~*90*~~|54|
|640|73|77|59||73|~~*98*~~|59||66|70|53||66|~~*88*~~|53|
|660|72|76|58||72|~~*96*~~|58||65|69|52||65|~~*87*~~|52|
|680|71|75|57||71|~~*95*~~|57||64|68|52||64|~~*86*~~|52|
|700|71|75|57||71|~~*95*~~|57||63|67|51||63|~~*84*~~|51|
|710|70|74|56||70|~~*94*~~|56||63|67|51||63|~~*84*~~|51|
|720|70|74|56||70|~~*94*~~|56||62|66|50||62|~~*83*~~|50|
|730|69|73|56||69|~~*92*~~|56||62|66|50||62|~~*83*~~|50|
|740|69|73|56||69|~~*92*~~|56||61|65|49||61|~~*82*~~|49|
|750|68|72|55||68|~~*91*~~|55||61|65|49||61|~~*82*~~|49|
|760|68|72|55||68|~~*91*~~|55||60|64|48||60|80|48|
|770|68|72|55||68|~~*91*~~|55||59|63|48||59|79|48|
|780|67|71|54||67|~~*90*~~|54||59|63|48||59|79|48|
|790|67|71|54||67|~~*90*~~|54||58|62|47||58|78|47|
|800|66|70|53||66|~~*88*~~|53||58|62|47||58|78|47|
|810|66|70|53||66|~~*88*~~|53||57|60|46||57|76|46|
|820|65|69|52||65|~~*87*~~|52||57|60|46||57|76|46|
|830|65|69|52||65|~~*87*~~|52||56|59|45||56|75|45|
|850|64|68|52||64|~~*86*~~|52||55|58|44||55|74|44|
|870|63|67|51||63|~~*84*~~|51||54|57|44||54|72|44|
|880|63|67|51||63|~~*84*~~|51||53|56|43||53|71|43|
|890|62|66|50||62|~~*83*~~|50||53|56|43||53|71|43|
|900|62|66|50||62|~~*83*~~|50||52|55|42||52|70|42|
|920|61|65|49||61|~~*82*~~|49||51|54|41||51|68|41|
|940|60|64|48||60|80|48||50|53|40||50|67|40|
|960|59|63|48||59|79|48||49|52|40||49|66|40|
|980|59|63|48||59|79|48||48|51|39||48|64|39|
|990|58|62|47||58|78|47||48|51|39||48|64|39|
|1000|58|62|47||58|78|47||47|50|38||47|63|38|
|1010|57|60|46||57|76|46||47|50|38||47|63|38|
|1020|57|60|46||57|76|46||46|49|37||46|62|37|
|1030|56|59|45||56|75|45||46|49|37||46|62|37|
|1040|56|59|45||56|75|45||45|48|36||45|60|36|
|1060|55|58|44||55|74|44||44|47|36||44|59|36|
|1070|55|58|44||55|74|44||43|46|35||43|58|35|
|1080|54|57|44||54|72|44||43|46|35||43|58|35|
|1090|54|57|44||54|72|44||42|45|34||42|56|34|
|1100|53|56|43||53|71|43||42|45|34||42|56|34|
|1110|53|56|43||53|71|43||41|44|33||41|55|33|
|1130|52|55|42||52|70|42||40|43|32||40|54|32|
|1150|51|54|41||51|68|41||39|42|32||39|52|32|
|1160|51|54|41||51|68|41||39|42|32||39|52|32|

### [Dragons Den](https://wiki.melvoridle.com/index.php?title=Dragons_Den), Auto Eat II

|❤️|⚔️|🏹|🧙||⚔️ ☠️|🏹 ☠️|🧙 ☠️||⚔️ 💍|🏹 💍|🧙 💍||⚔️ ☠️ 💍|🏹 ☠️ 💍|🧙 ☠️ 💍|
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|100|~~*94*~~|~~*99*~~|76||~~*94*~~|~~*100*~~|76||~~*93*~~|~~*98*~~|75||~~*93*~~|~~*100*~~|75|
|110|~~*93*~~|~~*98*~~|75||~~*93*~~|~~*100*~~|75||~~*92*~~|~~*97*~~|74||~~*92*~~|~~*100*~~|74|
|120|~~*93*~~|~~*98*~~|75||~~*93*~~|~~*100*~~|75||~~*91*~~|~~*96*~~|73||~~*91*~~|~~*100*~~|73|
|130|~~*92*~~|~~*97*~~|74||~~*92*~~|~~*100*~~|74||~~*91*~~|~~*96*~~|73||~~*91*~~|~~*100*~~|73|
|140|~~*91*~~|~~*96*~~|73||~~*91*~~|~~*100*~~|73||~~*90*~~|~~*95*~~|72||~~*90*~~|~~*100*~~|72|
|150|~~*91*~~|~~*96*~~|73||~~*91*~~|~~*100*~~|73||~~*89*~~|~~*94*~~|72||~~*89*~~|~~*100*~~|72|
|160|~~*90*~~|~~*95*~~|72||~~*90*~~|~~*100*~~|72||~~*88*~~|~~*93*~~|71||~~*88*~~|~~*100*~~|71|
|170|~~*89*~~|~~*94*~~|72||~~*89*~~|~~*100*~~|72||~~*88*~~|~~*93*~~|71||~~*88*~~|~~*100*~~|71|
|180|~~*89*~~|~~*94*~~|72||~~*89*~~|~~*100*~~|72||~~*87*~~|~~*92*~~|70||~~*87*~~|~~*100*~~|70|
|190|~~*88*~~|~~*93*~~|71||~~*88*~~|~~*100*~~|71||~~*86*~~|~~*91*~~|69||~~*86*~~|~~*100*~~|69|
|200|~~*88*~~|~~*93*~~|71||~~*88*~~|~~*100*~~|71||~~*85*~~|~~*90*~~|68||~~*85*~~|~~*100*~~|68|
|210|~~*87*~~|~~*92*~~|70||~~*87*~~|~~*100*~~|70||~~*85*~~|~~*90*~~|68||~~*85*~~|~~*100*~~|68|
|220|~~*86*~~|~~*91*~~|69||~~*86*~~|~~*100*~~|69||~~*84*~~|~~*89*~~|68||~~*84*~~|~~*100*~~|68|
|230|~~*86*~~|~~*91*~~|69||~~*86*~~|~~*100*~~|69||~~*83*~~|~~*88*~~|67||~~*83*~~|~~*100*~~|67|
|240|~~*85*~~|~~*90*~~|68||~~*85*~~|~~*100*~~|68||~~*82*~~|~~*87*~~|66||~~*82*~~|~~*100*~~|66|
|250|~~*84*~~|~~*89*~~|68||~~*84*~~|~~*100*~~|68||~~*82*~~|~~*87*~~|66||~~*82*~~|~~*100*~~|66|
|260|~~*84*~~|~~*89*~~|68||~~*84*~~|~~*100*~~|68||81|~~*86*~~|65||81|~~*100*~~|65|
|270|~~*83*~~|~~*88*~~|67||~~*83*~~|~~*100*~~|67||80|~~*85*~~|64||80|~~*100*~~|64|
|280|~~*82*~~|~~*87*~~|66||~~*82*~~|~~*100*~~|66||79|~~*84*~~|64||79|~~*100*~~|64|
|300|81|~~*86*~~|65||81|~~*100*~~|65||78|~~*83*~~|63||78|~~*100*~~|63|
|310|80|~~*85*~~|64||80|~~*100*~~|64||77|~~*82*~~|62||77|~~*100*~~|62|
|320|80|~~*85*~~|64||80|~~*100*~~|64||76|80|61||76|~~*100*~~|61|
|330|79|~~*84*~~|64||79|~~*100*~~|64||76|80|61||76|~~*100*~~|61|
|340|79|~~*84*~~|64||79|~~*100*~~|64||75|79|60||75|~~*100*~~|60|
|350|78|~~*83*~~|63||78|~~*100*~~|63||74|78|60||74|~~*99*~~|60|
|360|77|~~*82*~~|62||77|~~*100*~~|62||74|78|60||74|~~*99*~~|60|
|370|77|~~*82*~~|62||77|~~*100*~~|62||73|77|59||73|~~*98*~~|59|
|380|76|80|61||76|~~*100*~~|61||72|76|58||72|~~*96*~~|58|
|390|75|79|60||75|~~*100*~~|60||71|75|57||71|~~*95*~~|57|
|410|74|78|60||74|~~*99*~~|60||70|74|56||70|~~*94*~~|56|
|420|73|77|59||73|~~*98*~~|59||69|73|56||69|~~*92*~~|56|
|430|73|77|59||73|~~*98*~~|59||68|72|55||68|~~*91*~~|55|
|440|72|76|58||72|~~*96*~~|58||68|72|55||68|~~*91*~~|55|
|450|72|76|58||72|~~*96*~~|58||67|71|54||67|~~*90*~~|54|
|460|71|75|57||71|~~*95*~~|57||66|70|53||66|~~*88*~~|53|
|470|70|74|56||70|~~*94*~~|56||65|69|52||65|~~*87*~~|52|
|490|69|73|56||69|~~*92*~~|56||64|68|52||64|~~*86*~~|52|
|500|68|72|55||68|~~*91*~~|55||63|67|51||63|~~*84*~~|51|
|510|68|72|55||68|~~*91*~~|55||62|66|50||62|~~*83*~~|50|
|520|67|71|54||67|~~*90*~~|54||62|66|50||62|~~*83*~~|50|
|530|66|70|53||66|~~*88*~~|53||61|65|49||61|~~*82*~~|49|
|540|66|70|53||66|~~*88*~~|53||60|64|48||60|80|48|
|550|65|69|52||65|~~*87*~~|52||59|63|48||59|79|48|
|570|64|68|52||64|~~*86*~~|52||58|62|47||58|78|47|
|580|63|67|51||63|~~*84*~~|51||57|60|46||57|76|46|
|590|63|67|51||63|~~*84*~~|51||56|59|45||56|75|45|
|600|62|66|50||62|~~*83*~~|50||56|59|45||56|75|45|
|610|61|65|49||61|~~*82*~~|49||55|58|44||55|74|44|
|620|61|65|49||61|~~*82*~~|49||54|57|44||54|72|44|
|630|60|64|48||60|80|48||53|56|43||53|71|43|
|640|59|63|48||59|79|48||53|56|43||53|71|43|
|650|59|63|48||59|79|48||52|55|42||52|70|42|
|660|58|62|47||58|78|47||51|54|41||51|68|41|
|680|57|60|46||57|76|46||50|53|40||50|67|40|
|690|56|59|45||56|75|45||49|52|40||49|66|40|
|700|56|59|45||56|75|45||48|51|39||48|64|39|
|710|55|58|44||55|74|44||48|51|39||48|64|39|
|720|54|57|44||54|72|44||47|50|38||47|63|38|
|730|54|57|44||54|72|44||46|49|37||46|62|37|
|740|53|56|43||53|71|43||45|48|36||45|60|36|
|750|52|55|42||52|70|42||45|48|36||45|60|36|
|760|52|55|42||52|70|42||44|47|36||44|59|36|
|770|51|54|41||51|68|41||43|46|35||43|58|35|
|780|51|54|41||51|68|41||42|45|34||42|56|34|
|790|50|53|40||50|67|40||42|45|34||42|56|34|
|800|49|52|40||49|66|40||41|44|33||41|55|33|
|810|49|52|40||49|66|40||40|43|32||40|54|32|
|820|48|51|39||48|64|39||39|42|32||39|52|32|
|830|47|50|38||47|63|38||39|42|32||39|52|32|
|840|47|50|38||47|63|38||38|40|31||38|51|31|
|850|46|49|37||46|62|37||37|39|30||37|50|30|
|860|45|48|36||45|60|36||36|38|29||36|48|29|
|880|44|47|36||44|59|36||35|37|28||35|47|28|
|890|43|46|35||43|58|35||34|36|28||34|46|28|
|900|43|46|35||43|58|35||33|35|27||33|44|27|
|910|42|45|34||42|56|34||33|35|27||33|44|27|
|920|42|45|34||42|56|34||32|34|26||32|43|26|
|930|41|44|33||41|55|33||31|33|25||31|42|25|
|940|40|43|32||40|54|32||30|32|24||30|40|24|
|960|39|42|32||39|52|32||29|31|24||29|39|24|
|970|38|40|31||38|51|31||28|30|23||28|38|23|
|980|38|40|31||38|51|31||27|29|22||27|36|22|
|990|37|39|30||37|50|30||27|29|22||27|36|22|
|1000|36|38|29||36|48|29||26|28|21||26|35|21|
|1010|36|38|29||36|48|29||25|27|20||25|34|20|
|1020|35|37|28||35|47|28||24|26|20||24|32|20|
|1040|34|36|28||34|46|28||23|25|19||23|31|19|
|1050|33|35|27||33|44|27||22|24|18||22|30|18|
|1060|33|35|27||33|44|27||21|23|17||21|28|17|
|1070|32|34|26||32|43|26||21|23|17||21|28|17|
|1080|31|33|25||31|42|25||20|22|16||20|27|16|
|1090|31|33|25||31|42|25||19|20|16||19|26|16|
|1100|30|32|24||30|40|24||18|19|15||18|24|15|
|1110|29|31|24||29|39|24||18|19|15||18|24|15|
|1120|29|31|24||29|39|24||17|18|14||17|23|14|
|1130|28|30|23||28|38|23||16|17|13||16|22|13|
|1140|28|30|23||28|38|23||15|16|12||15|20|12|
|1150|27|29|22||27|36|22||15|16|12||15|20|12|
|1160|26|28|21||26|35|21||14|15|12||14|19|12|

### [Dragons Den](https://wiki.melvoridle.com/index.php?title=Dragons_Den), Auto Eat III

|❤️|⚔️|🏹|🧙||⚔️ ☠️|🏹 ☠️|🧙 ☠️||⚔️ 💍|🏹 💍|🧙 💍||⚔️ ☠️ 💍|🏹 ☠️ 💍|🧙 ☠️ 💍|
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|100|~~*92*~~|~~*97*~~|74||~~*92*~~|~~*100*~~|74||~~*91*~~|~~*96*~~|73||~~*91*~~|~~*100*~~|73|
|110|~~*91*~~|~~*96*~~|73||~~*91*~~|~~*100*~~|73||~~*90*~~|~~*95*~~|72||~~*90*~~|~~*100*~~|72|
|120|~~*90*~~|~~*95*~~|72||~~*90*~~|~~*100*~~|72||~~*89*~~|~~*94*~~|72||~~*89*~~|~~*100*~~|72|
|130|~~*89*~~|~~*94*~~|72||~~*89*~~|~~*100*~~|72||~~*88*~~|~~*93*~~|71||~~*88*~~|~~*100*~~|71|
|140|~~*88*~~|~~*93*~~|71||~~*88*~~|~~*100*~~|71||~~*87*~~|~~*92*~~|70||~~*87*~~|~~*100*~~|70|
|150|~~*88*~~|~~*93*~~|71||~~*88*~~|~~*100*~~|71||~~*86*~~|~~*91*~~|69||~~*86*~~|~~*100*~~|69|
|160|~~*87*~~|~~*92*~~|70||~~*87*~~|~~*100*~~|70||~~*85*~~|~~*90*~~|68||~~*85*~~|~~*100*~~|68|
|170|~~*86*~~|~~*91*~~|69||~~*86*~~|~~*100*~~|69||~~*84*~~|~~*89*~~|68||~~*84*~~|~~*100*~~|68|
|180|~~*85*~~|~~*90*~~|68||~~*85*~~|~~*100*~~|68||~~*83*~~|~~*88*~~|67||~~*83*~~|~~*100*~~|67|
|190|~~*84*~~|~~*89*~~|68||~~*84*~~|~~*100*~~|68||~~*82*~~|~~*87*~~|66||~~*82*~~|~~*100*~~|66|
|200|~~*83*~~|~~*88*~~|67||~~*83*~~|~~*100*~~|67||81|~~*86*~~|65||81|~~*100*~~|65|
|210|~~*82*~~|~~*87*~~|66||~~*82*~~|~~*100*~~|66||80|~~*85*~~|64||80|~~*100*~~|64|
|220|~~*82*~~|~~*87*~~|66||~~*82*~~|~~*100*~~|66||79|~~*84*~~|64||79|~~*100*~~|64|
|230|81|~~*86*~~|65||81|~~*100*~~|65||78|~~*83*~~|63||78|~~*100*~~|63|
|240|80|~~*85*~~|64||80|~~*100*~~|64||77|~~*82*~~|62||77|~~*100*~~|62|
|250|79|~~*84*~~|64||79|~~*100*~~|64||76|80|61||76|~~*100*~~|61|
|260|78|~~*83*~~|63||78|~~*100*~~|63||75|79|60||75|~~*100*~~|60|
|270|77|~~*82*~~|62||77|~~*100*~~|62||75|79|60||75|~~*100*~~|60|
|280|76|80|61||76|~~*100*~~|61||73|77|59||73|~~*98*~~|59|
|300|75|79|60||75|~~*100*~~|60||72|76|58||72|~~*96*~~|58|
|310|74|78|60||74|~~*99*~~|60||71|75|57||71|~~*95*~~|57|
|320|73|77|59||73|~~*98*~~|59||70|74|56||70|~~*94*~~|56|
|330|72|76|58||72|~~*96*~~|58||69|73|56||69|~~*92*~~|56|
|340|71|75|57||71|~~*95*~~|57||68|72|55||68|~~*91*~~|55|
|350|71|75|57||71|~~*95*~~|57||67|71|54||67|~~*90*~~|54|
|360|70|74|56||70|~~*94*~~|56||66|70|53||66|~~*88*~~|53|
|370|69|73|56||69|~~*92*~~|56||65|69|52||65|~~*87*~~|52|
|380|68|72|55||68|~~*91*~~|55||64|68|52||64|~~*86*~~|52|
|390|67|71|54||67|~~*90*~~|54||63|67|51||63|~~*84*~~|51|
|400|66|70|53||66|~~*88*~~|53||62|66|50||62|~~*83*~~|50|
|410|65|69|52||65|~~*87*~~|52||61|65|49||61|~~*82*~~|49|
|420|65|69|52||65|~~*87*~~|52||60|64|48||60|80|48|
|430|64|68|52||64|~~*86*~~|52||59|63|48||59|79|48|
|440|63|67|51||63|~~*84*~~|51||58|62|47||58|78|47|
|450|62|66|50||62|~~*83*~~|50||57|60|46||57|76|46|
|460|61|65|49||61|~~*82*~~|49||56|59|45||56|75|45|
|470|60|64|48||60|80|48||55|58|44||55|74|44|
|480|59|63|48||59|79|48||54|57|44||54|72|44|
|490|59|63|48||59|79|48||53|56|43||53|71|43|
|500|58|62|47||58|78|47||52|55|42||52|70|42|
|510|57|60|46||57|76|46||52|55|42||52|70|42|
|520|56|59|45||56|75|45||51|54|41||51|68|41|
|530|55|58|44||55|74|44||50|53|40||50|67|40|
|540|54|57|44||54|72|44||49|52|40||49|66|40|
|550|53|56|43||53|71|43||48|51|39||48|64|39|
|560|53|56|43||53|71|43||47|50|38||47|63|38|
|570|52|55|42||52|70|42||46|49|37||46|62|37|
|580|51|54|41||51|68|41||45|48|36||45|60|36|
|590|50|53|40||50|67|40||44|47|36||44|59|36|
|600|49|52|40||49|66|40||43|46|35||43|58|35|
|610|48|51|39||48|64|39||42|45|34||42|56|34|
|620|48|51|39||48|64|39||41|44|33||41|55|33|
|630|47|50|38||47|63|38||40|43|32||40|54|32|
|640|46|49|37||46|62|37||39|42|32||39|52|32|
|650|45|48|36||45|60|36||38|40|31||38|51|31|
|660|44|47|36||44|59|36||37|39|30||37|50|30|
|670|43|46|35||43|58|35||36|38|29||36|48|29|
|680|42|45|34||42|56|34||35|37|28||35|47|28|
|690|42|45|34||42|56|34||34|36|28||34|46|28|
|700|41|44|33||41|55|33||33|35|27||33|44|27|
|710|40|43|32||40|54|32||32|34|26||32|43|26|
|720|39|42|32||39|52|32||31|33|25||31|42|25|
|730|38|40|31||38|51|31||31|33|25||31|42|25|
|740|37|39|30||37|50|30||29|31|24||29|39|24|
|750|36|38|29||36|48|29||29|31|24||29|39|24|
|760|36|38|29||36|48|29||28|30|23||28|38|23|
|770|35|37|28||35|47|28||27|29|22||27|36|22|
|780|34|36|28||34|46|28||26|28|21||26|35|21|
|790|33|35|27||33|44|27||25|27|20||25|34|20|
|800|32|34|26||32|43|26||24|26|20||24|32|20|
|810|31|33|25||31|42|25||23|25|19||23|31|19|
|820|31|33|25||31|42|25||22|24|18||22|30|18|
|830|30|32|24||30|40|24||21|23|17||21|28|17|
|840|29|31|24||29|39|24||20|22|16||20|27|16|
|850|28|30|23||28|38|23||19|20|16||19|26|16|
|860|27|29|22||27|36|22||18|19|15||18|24|15|
|870|26|28|21||26|35|21||17|18|14||17|23|14|
|880|25|27|20||25|34|20||16|17|13||16|22|13|
|890|25|27|20||25|34|20||15|16|12||15|20|12|
|900|24|26|20||24|32|20||14|15|12||14|19|12|
|910|23|25|19||23|31|19||13|14|11||13|18|11|
|920|22|24|18||22|30|18||12|13|10||12|16|10|
|930|21|23|17||21|28|17||11|12|9||11|15|9|
|940|20|22|16||20|27|16||10|11|8||10|14|8|
|950|19|20|16||19|26|16||9|10|8||9|12|8|
|960|19|20|16||19|26|16||8|9|7||8|11|7|
|970|18|19|15||18|24|15||8|9|7||8|11|7|
|980|17|18|14||17|23|14||6|7|5||6|8|5|
|990|16|17|13||16|22|13||6|7|5||6|8|5|
|1000|15|16|12||15|20|12||5|6|4||5|7|4|
|1010|14|15|12||14|19|12||4|5|4||4|6|4|
|1020|13|14|11||13|18|11||3|4|3||3|4|3|
|1030|13|14|11||13|18|11||2|3|2||2|3|2|
|1040|12|13|10||12|16|10||1|2|1||1|2|1|
|1050|11|12|9||11|15|9||0|0|0||0|0|0|
|1060|10|11|8||10|14|8||0|0|0||0|0|0|
|1070|9|10|8||9|12|8||0|0|0||0|0|0|
|1080|8|9|7||8|11|7||0|0|0||0|0|0|
|1100|7|8|6||7|10|6||0|0|0||0|0|0|
|1110|6|7|5||6|8|5||0|0|0||0|0|0|
|1120|5|6|4||5|7|4||0|0|0||0|0|0|
|1130|4|5|4||4|6|4||0|0|0||0|0|0|
|1140|3|4|3||3|4|3||0|0|0||0|0|0|
|1150|2|3|2||2|3|2||0|0|0||0|0|0|
|1160|2|3|2||2|3|2||0|0|0||0|0|0|

### [Volcanic Cave](https://wiki.melvoridle.com/index.php?title=Volcanic_Cave), Auto Eat I

|❤️|⚔️|🏹|🧙||⚔️ ☠️|🏹 ☠️|🧙 ☠️||⚔️ 💍|🏹 💍|🧙 💍||⚔️ ☠️ 💍|🏹 ☠️ 💍|🧙 ☠️ 💍|
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|500|~~*100*~~|~~*86*~~|~~*95*~~||~~*100*~~|~~*100*~~|~~*100*~~||~~*100*~~|80|~~*89*~~||~~*100*~~|~~*100*~~|~~*100*~~|
|520|~~*100*~~|~~*85*~~|~~*95*~~||~~*100*~~|~~*100*~~|~~*100*~~||~~*100*~~|79|~~*88*~~||~~*100*~~|~~*100*~~|~~*99*~~|
|530|~~*100*~~|~~*85*~~|~~*93*~~||~~*100*~~|~~*100*~~|~~*100*~~||~~*100*~~|79|~~*88*~~||~~*100*~~|~~*100*~~|~~*99*~~|
|540|~~*100*~~|~~*85*~~|~~*93*~~||~~*100*~~|~~*100*~~|~~*100*~~||~~*100*~~|78|~~*86*~~||~~*100*~~|~~*99*~~|~~*98*~~|
|550|~~*100*~~|~~*84*~~|~~*92*~~||~~*100*~~|~~*100*~~|~~*100*~~||~~*100*~~|78|~~*86*~~||~~*100*~~|~~*99*~~|~~*98*~~|
|560|~~*100*~~|~~*84*~~|~~*92*~~||~~*100*~~|~~*100*~~|~~*100*~~||~~*100*~~|77|~~*85*~~||~~*100*~~|~~*98*~~|~~*96*~~|
|570|~~*100*~~|~~*83*~~|~~*92*~~||~~*100*~~|~~*100*~~|~~*100*~~||~~*100*~~|77|~~*85*~~||~~*100*~~|~~*98*~~|~~*96*~~|
|580|~~*100*~~|~~*83*~~|~~*91*~~||~~*100*~~|~~*100*~~|~~*100*~~||~~*100*~~|76|~~*84*~~||~~*100*~~|~~*96*~~|~~*95*~~|
|600|~~*100*~~|~~*82*~~|~~*90*~~||~~*100*~~|~~*100*~~|~~*100*~~||~~*100*~~|75|~~*83*~~||~~*100*~~|~~*95*~~|~~*94*~~|
|620|~~*100*~~|80|~~*90*~~||~~*100*~~|~~*100*~~|~~*100*~~||~~*100*~~|75|~~*82*~~||~~*100*~~|~~*95*~~|~~*92*~~|
|630|~~*100*~~|80|~~*89*~~||~~*100*~~|~~*100*~~|~~*100*~~||~~*100*~~|74|~~*82*~~||~~*100*~~|~~*94*~~|~~*92*~~|
|640|~~*100*~~|80|~~*89*~~||~~*100*~~|~~*100*~~|~~*100*~~||~~*100*~~|74|80||~~*100*~~|~~*94*~~|~~*91*~~|
|650|~~*100*~~|79|~~*88*~~||~~*100*~~|~~*100*~~|~~*99*~~||~~*100*~~|73|80||~~*100*~~|~~*92*~~|~~*91*~~|
|660|~~*100*~~|79|~~*88*~~||~~*100*~~|~~*100*~~|~~*99*~~||~~*100*~~|73|79||~~*100*~~|~~*92*~~|~~*90*~~|
|670|~~*100*~~|79|~~*88*~~||~~*100*~~|~~*100*~~|~~*99*~~||~~*100*~~|72|79||~~*100*~~|~~*91*~~|~~*90*~~|
|680|~~*100*~~|78|~~*86*~~||~~*100*~~|~~*99*~~|~~*98*~~||~~*100*~~|72|78||~~*100*~~|~~*91*~~|~~*88*~~|
|690|~~*100*~~|78|~~*86*~~||~~*100*~~|~~*99*~~|~~*98*~~||~~*100*~~|71|78||~~*100*~~|~~*90*~~|~~*88*~~|
|700|~~*100*~~|77|~~*85*~~||~~*100*~~|~~*98*~~|~~*96*~~||~~*98*~~|71|77||~~*100*~~|~~*90*~~|~~*87*~~|
|710|~~*100*~~|77|~~*85*~~||~~*100*~~|~~*98*~~|~~*96*~~||~~*96*~~|70|77||~~*100*~~|~~*88*~~|~~*87*~~|
|720|~~*100*~~|77|~~*85*~~||~~*100*~~|~~*98*~~|~~*96*~~||~~*94*~~|70|76||~~*100*~~|~~*88*~~|~~*86*~~|
|730|~~*100*~~|76|~~*84*~~||~~*100*~~|~~*96*~~|~~*95*~~||~~*94*~~|69|76||~~*100*~~|~~*87*~~|~~*86*~~|
|740|~~*100*~~|76|~~*84*~~||~~*100*~~|~~*96*~~|~~*95*~~||~~*92*~~|69|75||~~*100*~~|~~*87*~~|~~*84*~~|
|750|~~*100*~~|75|~~*83*~~||~~*100*~~|~~*95*~~|~~*94*~~||~~*90*~~|68|75||~~*100*~~|~~*86*~~|~~*84*~~|
|760|~~*100*~~|75|~~*83*~~||~~*100*~~|~~*95*~~|~~*94*~~||~~*88*~~|68|73||~~*100*~~|~~*86*~~|~~*83*~~|
|770|~~*100*~~|75|~~*83*~~||~~*100*~~|~~*95*~~|~~*94*~~||~~*88*~~|67|73||~~*100*~~|~~*84*~~|~~*83*~~|
|780|~~*100*~~|74|~~*82*~~||~~*100*~~|~~*94*~~|~~*92*~~||~~*86*~~|67|72||~~*100*~~|~~*84*~~|~~*82*~~|
|790|~~*100*~~|74|~~*82*~~||~~*100*~~|~~*94*~~|~~*92*~~||~~*84*~~|66|72||~~*100*~~|~~*83*~~|~~*82*~~|
|800|~~*100*~~|74|80||~~*100*~~|~~*94*~~|~~*91*~~||~~*82*~~|66|71||~~*100*~~|~~*83*~~|80|
|810|~~*100*~~|73|80||~~*100*~~|~~*92*~~|~~*91*~~||~~*82*~~|65|71||~~*100*~~|~~*82*~~|80|
|820|~~*100*~~|73|80||~~*100*~~|~~*92*~~|~~*91*~~||80|65|70||~~*100*~~|~~*82*~~|79|
|830|~~*100*~~|72|79||~~*100*~~|~~*91*~~|~~*90*~~||78|65|70||~~*100*~~|~~*82*~~|79|
|840|~~*100*~~|72|79||~~*100*~~|~~*91*~~|~~*90*~~||76|64|69||~~*100*~~|80|78|
|850|~~*100*~~|72|78||~~*100*~~|~~*91*~~|~~*88*~~||76|64|69||~~*100*~~|80|78|
|860|~~*100*~~|71|78||~~*100*~~|~~*90*~~|~~*88*~~||74|63|68||~~*100*~~|79|76|
|870|~~*98*~~|71|78||~~*100*~~|~~*90*~~|~~*88*~~||72|63|68||~~*100*~~|79|76|
|880|~~*96*~~|70|77||~~*100*~~|~~*88*~~|~~*87*~~||72|62|66||~~*100*~~|78|75|
|890|~~*96*~~|70|77||~~*100*~~|~~*88*~~|~~*87*~~||70|62|66||~~*100*~~|78|75|
|900|~~*94*~~|70|76||~~*100*~~|~~*88*~~|~~*86*~~||68|60|65||~~*100*~~|76|74|
|910|~~*94*~~|69|76||~~*100*~~|~~*87*~~|~~*86*~~||66|60|65||~~*100*~~|76|74|
|920|~~*92*~~|69|76||~~*100*~~|~~*87*~~|~~*86*~~||66|59|64||~~*100*~~|75|72|
|930|~~*92*~~|69|75||~~*100*~~|~~*87*~~|~~*84*~~||64|59|64||~~*100*~~|75|72|
|940|~~*90*~~|68|75||~~*100*~~|~~*86*~~|~~*84*~~||62|58|63||~~*100*~~|74|71|
|950|~~*88*~~|68|73||~~*100*~~|~~*86*~~|~~*83*~~||60|58|63||~~*100*~~|74|71|
|960|~~*88*~~|67|73||~~*100*~~|~~*84*~~|~~*83*~~||60|57|62||~~*100*~~|72|70|
|970|~~*86*~~|67|73||~~*100*~~|~~*84*~~|~~*83*~~||58|57|62||~~*100*~~|72|70|
|980|~~*86*~~|67|72||~~*100*~~|~~*84*~~|~~*82*~~||56|56|60||~~*100*~~|71|68|
|990|~~*84*~~|66|72||~~*100*~~|~~*83*~~|~~*82*~~||56|56|60||~~*100*~~|71|68|
|1000|~~*82*~~|66|71||~~*100*~~|~~*83*~~|80||54|55|59||~~*100*~~|70|67|
|1010|~~*82*~~|65|71||~~*100*~~|~~*82*~~|80||52|55|59||~~*100*~~|70|67|
|1020|80|65|71||~~*100*~~|~~*82*~~|80||51|54|58||~~*100*~~|68|66|
|1030|80|65|70||~~*100*~~|~~*82*~~|79||51|54|58||~~*100*~~|68|66|
|1040|78|64|70||~~*100*~~|80|79||50|53|57||~~*96*~~|67|64|
|1050|76|64|69||~~*100*~~|80|78||50|53|57||~~*92*~~|67|64|
|1060|76|64|69||~~*100*~~|80|78||49|52|56||~~*88*~~|66|63|
|1070|74|63|69||~~*100*~~|79|78||49|52|56||~~*88*~~|66|63|
|1080|74|63|68||~~*100*~~|79|76||48|51|55||~~*84*~~|64|62|
|1090|72|62|68||~~*100*~~|78|76||48|51|55||80|64|62|
|1100|72|62|66||~~*100*~~|78|75||47|50|53||76|63|60|
|1110|70|62|66||~~*100*~~|78|75||47|50|53||76|63|60|
|1120|68|60|66||~~*100*~~|76|75||46|49|52||72|62|59|
|1130|68|60|65||~~*100*~~|76|74||46|49|52||68|62|59|
|1140|66|59|65||~~*100*~~|75|74||46|49|51||64|62|58|
|1150|66|59|64||~~*100*~~|75|72||45|48|51||64|60|58|
|1160|64|59|64||~~*100*~~|75|72||45|48|50||60|60|56|

### [Volcanic Cave](https://wiki.melvoridle.com/index.php?title=Volcanic_Cave), Auto Eat II

|❤️|⚔️|🏹|🧙||⚔️ ☠️|🏹 ☠️|🧙 ☠️||⚔️ 💍|🏹 💍|🧙 💍||⚔️ ☠️ 💍|🏹 ☠️ 💍|🧙 ☠️ 💍|
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|360|~~*100*~~|~~*85*~~|~~*93*~~||~~*100*~~|~~*100*~~|~~*100*~~||~~*100*~~|80|~~*89*~~||~~*100*~~|~~*100*~~|~~*100*~~|
|370|~~*100*~~|~~*84*~~|~~*92*~~||~~*100*~~|~~*100*~~|~~*100*~~||~~*100*~~|80|~~*89*~~||~~*100*~~|~~*100*~~|~~*100*~~|
|380|~~*100*~~|~~*83*~~|~~*92*~~||~~*100*~~|~~*100*~~|~~*100*~~||~~*100*~~|79|~~*88*~~||~~*100*~~|~~*100*~~|~~*99*~~|
|390|~~*100*~~|~~*83*~~|~~*91*~~||~~*100*~~|~~*100*~~|~~*100*~~||~~*100*~~|78|~~*86*~~||~~*100*~~|~~*99*~~|~~*98*~~|
|400|~~*100*~~|~~*82*~~|~~*90*~~||~~*100*~~|~~*100*~~|~~*100*~~||~~*100*~~|77|~~*85*~~||~~*100*~~|~~*98*~~|~~*96*~~|
|420|~~*100*~~|80|~~*89*~~||~~*100*~~|~~*100*~~|~~*100*~~||~~*100*~~|76|~~*84*~~||~~*100*~~|~~*96*~~|~~*95*~~|
|430|~~*100*~~|80|~~*89*~~||~~*100*~~|~~*100*~~|~~*100*~~||~~*100*~~|75|~~*83*~~||~~*100*~~|~~*95*~~|~~*94*~~|
|440|~~*100*~~|79|~~*88*~~||~~*100*~~|~~*100*~~|~~*99*~~||~~*100*~~|75|~~*83*~~||~~*100*~~|~~*95*~~|~~*94*~~|
|450|~~*100*~~|78|~~*86*~~||~~*100*~~|~~*99*~~|~~*98*~~||~~*100*~~|74|~~*82*~~||~~*100*~~|~~*94*~~|~~*92*~~|
|460|~~*100*~~|78|~~*86*~~||~~*100*~~|~~*99*~~|~~*98*~~||~~*100*~~|73|80||~~*100*~~|~~*92*~~|~~*91*~~|
|470|~~*100*~~|77|~~*85*~~||~~*100*~~|~~*98*~~|~~*96*~~||~~*100*~~|73|80||~~*100*~~|~~*92*~~|~~*91*~~|
|480|~~*100*~~|77|~~*85*~~||~~*100*~~|~~*98*~~|~~*96*~~||~~*100*~~|72|79||~~*100*~~|~~*91*~~|~~*90*~~|
|490|~~*100*~~|76|~~*84*~~||~~*100*~~|~~*96*~~|~~*95*~~||~~*100*~~|71|78||~~*100*~~|~~*90*~~|~~*88*~~|
|500|~~*100*~~|75|~~*83*~~||~~*100*~~|~~*95*~~|~~*94*~~||~~*98*~~|71|77||~~*100*~~|~~*90*~~|~~*87*~~|
|510|~~*100*~~|75|~~*83*~~||~~*100*~~|~~*95*~~|~~*94*~~||~~*96*~~|70|77||~~*100*~~|~~*88*~~|~~*87*~~|
|520|~~*100*~~|74|~~*82*~~||~~*100*~~|~~*94*~~|~~*92*~~||~~*94*~~|69|76||~~*100*~~|~~*87*~~|~~*86*~~|
|530|~~*100*~~|74|~~*82*~~||~~*100*~~|~~*94*~~|~~*92*~~||~~*92*~~|69|75||~~*100*~~|~~*87*~~|~~*84*~~|
|540|~~*100*~~|73|80||~~*100*~~|~~*92*~~|~~*91*~~||~~*90*~~|68|75||~~*100*~~|~~*86*~~|~~*84*~~|
|550|~~*100*~~|73|79||~~*100*~~|~~*92*~~|~~*90*~~||~~*88*~~|67|73||~~*100*~~|~~*84*~~|~~*83*~~|
|560|~~*100*~~|72|79||~~*100*~~|~~*91*~~|~~*90*~~||~~*86*~~|67|72||~~*100*~~|~~*84*~~|~~*82*~~|
|570|~~*100*~~|71|78||~~*100*~~|~~*90*~~|~~*88*~~||~~*84*~~|66|72||~~*100*~~|~~*83*~~|~~*82*~~|
|580|~~*98*~~|71|78||~~*100*~~|~~*90*~~|~~*88*~~||~~*82*~~|65|71||~~*100*~~|~~*82*~~|80|
|590|~~*96*~~|70|77||~~*100*~~|~~*88*~~|~~*87*~~||80|65|70||~~*100*~~|~~*82*~~|79|
|600|~~*94*~~|70|76||~~*100*~~|~~*88*~~|~~*86*~~||76|64|69||~~*100*~~|80|78|
|610|~~*92*~~|69|76||~~*100*~~|~~*87*~~|~~*86*~~||76|63|69||~~*100*~~|79|78|
|620|~~*92*~~|69|75||~~*100*~~|~~*87*~~|~~*84*~~||72|63|68||~~*100*~~|79|76|
|630|~~*90*~~|68|75||~~*100*~~|~~*86*~~|~~*84*~~||72|62|66||~~*100*~~|78|75|
|640|~~*88*~~|67|73||~~*100*~~|~~*84*~~|~~*83*~~||68|60|66||~~*100*~~|76|75|
|650|~~*86*~~|67|72||~~*100*~~|~~*84*~~|~~*82*~~||66|60|65||~~*100*~~|76|74|
|660|~~*84*~~|66|72||~~*100*~~|~~*83*~~|~~*82*~~||66|59|64||~~*100*~~|75|72|
|670|~~*82*~~|66|71||~~*100*~~|~~*83*~~|80||62|58|64||~~*100*~~|74|72|
|680|80|65|71||~~*100*~~|~~*82*~~|80||60|58|63||~~*100*~~|74|71|
|690|78|65|70||~~*100*~~|~~*82*~~|79||58|57|62||~~*100*~~|72|70|
|700|76|64|69||~~*100*~~|80|78||56|56|62||~~*100*~~|71|70|
|710|76|63|69||~~*100*~~|79|78||54|56|60||~~*100*~~|71|68|
|720|74|63|68||~~*100*~~|79|76||52|55|59||~~*100*~~|70|67|
|730|72|62|68||~~*100*~~|78|76||51|54|58||~~*100*~~|68|66|
|740|70|62|66||~~*100*~~|78|75||51|54|58||~~*96*~~|68|66|
|750|68|60|65||~~*100*~~|76|74||50|53|57||~~*92*~~|67|64|
|760|66|59|65||~~*100*~~|75|74||49|52|56||~~*88*~~|66|63|
|770|64|59|64||~~*100*~~|75|72||49|52|56||~~*84*~~|66|63|
|780|62|58|64||~~*100*~~|74|72||48|51|55||80|64|62|
|790|60|58|63||~~*100*~~|74|71||47|50|53||76|63|60|
|800|60|57|62||~~*100*~~|72|70||46|49|52||72|62|59|
|810|58|57|62||~~*100*~~|72|70||46|49|52||68|62|59|
|820|56|56|60||~~*100*~~|71|68||45|48|51||64|60|58|
|830|54|55|60||~~*100*~~|70|68||45|48|50||60|60|56|
|840|52|55|59||~~*100*~~|70|67||44|47|50||56|59|56|
|850|51|54|58||~~*100*~~|68|66||43|46|49||52|58|55|
|860|51|54|58||~~*96*~~|68|66||42|45|48||48|56|54|
|870|50|53|57||~~*92*~~|67|64||42|45|48||44|56|54|
|880|50|53|57||~~*92*~~|67|64||41|44|46||41|55|52|
|890|49|52|56||~~*88*~~|66|63||41|44|45||41|55|51|
|900|48|51|55||~~*84*~~|64|62||40|43|44||40|54|50|
|910|48|51|55||80|64|62||39|42|44||39|52|50|
|920|47|50|53||76|63|60||38|40|43||38|51|48|
|930|47|50|53||72|63|60||38|40|42||38|51|47|
|940|46|49|52||68|62|59||37|39|42||37|50|47|
|950|46|49|51||64|62|58||36|38|40||36|48|46|
|960|45|48|51||64|60|58||36|38|39||36|48|44|
|970|44|47|50||60|59|56||35|37|39||35|47|44|
|980|44|47|50||56|59|56||34|36|38||34|46|43|
|990|43|46|49||52|58|55||34|36|37||34|46|42|
|1000|43|46|48||48|58|54||33|35|36||33|44|40|
|1010|42|45|48||44|56|54||32|34|36||32|43|40|
|1020|41|44|46||41|55|52||32|34|35||32|43|39|
|1030|41|44|46||41|55|52||31|33|33||31|42|38|
|1040|40|43|45||40|54|51||30|32|33||30|40|38|
|1050|40|43|44||40|54|50||30|32|32||30|40|36|
|1060|39|42|44||39|52|50||29|31|31||29|39|35|
|1070|39|42|43||39|52|48||28|30|31||28|38|35|
|1080|38|40|43||38|51|48||28|30|30||28|38|34|
|1090|37|39|42||37|50|47||27|29|29||27|36|32|
|1100|37|39|40||37|50|46||26|28|28||26|35|31|
|1110|36|38|40||36|48|46||26|28|28||26|35|31|
|1120|36|38|39||36|48|44||25|27|26||25|34|30|
|1130|35|37|39||35|47|44||24|26|25||24|32|28|
|1140|35|37|38||35|47|43||24|26|25||24|32|28|
|1150|34|36|37||34|46|42||23|25|24||23|31|27|
|1160|33|35|37||33|44|42||22|24|23||22|30|26|

### [Volcanic Cave](https://wiki.melvoridle.com/index.php?title=Volcanic_Cave), Auto Eat III

|❤️|⚔️|🏹|🧙||⚔️ ☠️|🏹 ☠️|🧙 ☠️||⚔️ 💍|🏹 💍|🧙 💍||⚔️ ☠️ 💍|🏹 ☠️ 💍|🧙 ☠️ 💍|
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|280|~~*100*~~|~~*84*~~|~~*92*~~||~~*100*~~|~~*100*~~|~~*100*~~||~~*100*~~|80|~~*89*~~||~~*100*~~|~~*100*~~|~~*100*~~|
|290|~~*100*~~|~~*83*~~|~~*91*~~||~~*100*~~|~~*100*~~|~~*100*~~||~~*100*~~|79|~~*88*~~||~~*100*~~|~~*100*~~|~~*99*~~|
|300|~~*100*~~|~~*82*~~|~~*90*~~||~~*100*~~|~~*100*~~|~~*100*~~||~~*100*~~|78|~~*86*~~||~~*100*~~|~~*99*~~|~~*98*~~|
|310|~~*100*~~|80|~~*90*~~||~~*100*~~|~~*100*~~|~~*100*~~||~~*100*~~|78|~~*86*~~||~~*100*~~|~~*99*~~|~~*98*~~|
|320|~~*100*~~|80|~~*89*~~||~~*100*~~|~~*100*~~|~~*100*~~||~~*100*~~|77|~~*85*~~||~~*100*~~|~~*98*~~|~~*96*~~|
|330|~~*100*~~|79|~~*88*~~||~~*100*~~|~~*100*~~|~~*99*~~||~~*100*~~|76|~~*84*~~||~~*100*~~|~~*96*~~|~~*95*~~|
|340|~~*100*~~|78|~~*86*~~||~~*100*~~|~~*99*~~|~~*98*~~||~~*100*~~|75|~~*83*~~||~~*100*~~|~~*95*~~|~~*94*~~|
|350|~~*100*~~|77|~~*85*~~||~~*100*~~|~~*98*~~|~~*96*~~||~~*100*~~|74|~~*82*~~||~~*100*~~|~~*94*~~|~~*92*~~|
|360|~~*100*~~|77|~~*85*~~||~~*100*~~|~~*98*~~|~~*96*~~||~~*100*~~|73|80||~~*100*~~|~~*92*~~|~~*91*~~|
|370|~~*100*~~|76|~~*84*~~||~~*100*~~|~~*96*~~|~~*95*~~||~~*100*~~|72|79||~~*100*~~|~~*91*~~|~~*90*~~|
|380|~~*100*~~|75|~~*83*~~||~~*100*~~|~~*95*~~|~~*94*~~||~~*100*~~|71|78||~~*100*~~|~~*90*~~|~~*88*~~|
|390|~~*100*~~|74|~~*82*~~||~~*100*~~|~~*94*~~|~~*92*~~||~~*98*~~|71|77||~~*100*~~|~~*90*~~|~~*87*~~|
|400|~~*100*~~|74|80||~~*100*~~|~~*94*~~|~~*91*~~||~~*94*~~|70|76||~~*100*~~|~~*88*~~|~~*86*~~|
|410|~~*100*~~|73|80||~~*100*~~|~~*92*~~|~~*91*~~||~~*92*~~|69|76||~~*100*~~|~~*87*~~|~~*86*~~|
|420|~~*100*~~|72|79||~~*100*~~|~~*91*~~|~~*90*~~||~~*90*~~|68|75||~~*100*~~|~~*86*~~|~~*84*~~|
|430|~~*100*~~|71|78||~~*100*~~|~~*90*~~|~~*88*~~||~~*86*~~|67|73||~~*100*~~|~~*84*~~|~~*83*~~|
|440|~~*96*~~|70|77||~~*100*~~|~~*88*~~|~~*87*~~||~~*84*~~|66|72||~~*100*~~|~~*83*~~|~~*82*~~|
|450|~~*94*~~|70|76||~~*100*~~|~~*88*~~|~~*86*~~||~~*82*~~|65|71||~~*100*~~|~~*82*~~|80|
|460|~~*92*~~|69|76||~~*100*~~|~~*87*~~|~~*86*~~||78|65|70||~~*100*~~|~~*82*~~|79|
|470|~~*90*~~|68|75||~~*100*~~|~~*86*~~|~~*84*~~||76|64|69||~~*100*~~|80|78|
|480|~~*88*~~|67|73||~~*100*~~|~~*84*~~|~~*83*~~||74|63|68||~~*100*~~|79|76|
|490|~~*86*~~|67|72||~~*100*~~|~~*84*~~|~~*82*~~||72|62|66||~~*100*~~|78|75|
|500|~~*82*~~|66|71||~~*100*~~|~~*83*~~|80||68|60|65||~~*100*~~|76|74|
|510|80|65|71||~~*100*~~|~~*82*~~|80||66|59|65||~~*100*~~|75|74|
|520|78|64|70||~~*100*~~|80|79||62|58|64||~~*100*~~|74|72|
|530|76|64|69||~~*100*~~|80|78||60|58|63||~~*100*~~|74|71|
|540|74|63|68||~~*100*~~|79|76||58|57|62||~~*100*~~|72|70|
|550|72|62|66||~~*100*~~|78|75||56|56|60||~~*100*~~|71|68|
|560|68|60|66||~~*100*~~|76|75||52|55|59||~~*100*~~|70|67|
|570|66|59|65||~~*100*~~|75|74||51|54|58||~~*100*~~|68|66|
|580|64|59|64||~~*100*~~|75|72||50|53|57||~~*92*~~|67|64|
|590|62|58|63||~~*100*~~|74|71||49|52|56||~~*88*~~|66|63|
|600|60|57|62||~~*100*~~|72|70||48|51|55||~~*84*~~|64|62|
|610|56|56|62||~~*100*~~|71|70||48|51|55||80|64|62|
|620|54|56|60||~~*100*~~|71|68||47|50|53||72|63|60|
|630|52|55|59||~~*100*~~|70|67||46|49|52||68|62|59|
|640|51|54|58||~~*100*~~|68|66||45|48|51||64|60|58|
|650|50|53|57||~~*96*~~|67|64||44|47|50||56|59|56|
|660|50|53|57||~~*92*~~|67|64||43|46|49||52|58|55|
|670|49|52|56||~~*84*~~|66|63||42|45|48||48|56|54|
|680|48|51|55||80|64|62||41|44|46||41|55|52|
|690|47|50|53||76|63|60||41|44|45||41|55|51|
|700|46|49|52||72|62|59||40|43|44||40|54|50|
|710|46|49|52||68|62|59||39|42|44||39|52|50|
|720|45|48|51||64|60|58||38|40|43||38|51|48|
|730|44|47|50||56|59|56||37|39|42||37|50|47|
|740|43|46|49||52|58|55||36|38|40||36|48|46|
|750|43|46|48||48|58|54||36|38|39||36|48|44|
|760|42|45|48||44|56|54||35|37|38||35|47|43|
|770|41|44|46||41|55|52||34|36|37||34|46|42|
|780|40|43|45||40|54|51||33|35|36||33|44|40|
|790|40|43|44||40|54|50||32|34|35||32|43|39|
|800|39|42|43||39|52|48||31|33|33||31|42|38|
|810|38|40|43||38|51|48||30|32|33||30|40|38|
|820|37|39|42||37|50|47||29|31|32||29|39|36|
|830|36|38|40||36|48|46||29|31|31||29|39|35|
|840|36|38|39||36|48|44||28|30|30||28|38|34|
|850|35|37|38||35|47|43||27|29|29||27|36|32|
|860|34|36|38||34|46|43||26|28|28||26|35|31|
|870|33|35|37||33|44|42||25|27|26||25|34|30|
|880|33|35|36||33|44|40||24|26|25||24|32|28|
|890|32|34|35||32|43|39||23|25|24||23|31|27|
|900|31|33|33||31|42|38||22|24|23||22|30|26|
|910|30|32|33||30|40|38||22|24|23||22|30|26|
|920|30|32|32||30|40|36||21|23|22||21|28|24|
|930|29|31|31||29|39|35||20|22|20||20|27|23|
|940|28|30|30||28|38|34||19|20|19||19|26|22|
|950|27|29|29||27|36|32||18|19|18||18|24|20|
|960|26|28|29||26|35|32||17|18|17||17|23|19|
|970|26|28|28||26|35|31||16|17|16||16|22|18|
|980|25|27|26||25|34|30||16|17|15||16|22|16|
|990|24|26|25||24|32|28||15|16|13||15|20|15|
|1000|23|25|24||23|31|27||14|15|12||14|19|14|
|1010|23|25|24||23|31|27||13|14|12||13|18|14|
|1020|22|24|23||22|30|26||12|13|11||12|16|12|
|1030|21|23|22||21|28|24||11|12|10||11|15|11|
|1040|20|22|20||20|27|23||10|11|9||10|14|10|
|1050|20|22|19||20|27|22||10|11|8||10|14|8|
|1060|19|20|19||19|26|22||9|10|8||9|12|8|
|1070|18|19|18||18|24|20||8|9|7||8|11|7|
|1080|17|18|17||17|23|19||7|8|6||7|10|6|
|1090|16|17|16||16|22|18||6|7|5||6|8|5|
|1100|16|17|15||16|22|16||5|6|4||5|7|4|
|1110|15|16|15||15|20|16||4|5|4||4|6|4|
|1120|14|15|13||14|19|15||3|4|3||3|4|3|
|1130|13|14|12||13|18|14||3|4|3||3|4|3|
|1140|13|14|11||13|18|12||2|3|2||2|3|2|
|1150|12|13|10||12|16|11||1|2|1||1|2|1|
|1160|11|12|10||11|15|11||0|0|0||0|0|0|

### [Infernal Stronghold](https://wiki.melvoridle.com/index.php?title=Infernal_Stronghold), Auto Eat I

|❤️|⚔️|🏹|🧙||⚔️ ☠️|🏹 ☠️|🧙 ☠️||⚔️ 💍|🏹 💍|🧙 💍||⚔️ ☠️ 💍|🏹 ☠️ 💍|🧙 ☠️ 💍|
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|670|~~*100*~~|~~*88*~~|~~*89*~~||~~*100*~~|~~*100*~~|~~*100*~~||~~*100*~~|~~*83*~~|80||~~*100*~~|~~*100*~~|~~*91*~~|
|680|~~*100*~~|~~*87*~~|~~*88*~~||~~*100*~~|~~*100*~~|~~*99*~~||~~*100*~~|~~*83*~~|80||~~*100*~~|~~*100*~~|~~*91*~~|
|690|~~*100*~~|~~*87*~~|~~*88*~~||~~*100*~~|~~*100*~~|~~*99*~~||~~*100*~~|~~*82*~~|79||~~*100*~~|~~*100*~~|~~*90*~~|
|700|~~*100*~~|~~*87*~~|~~*86*~~||~~*100*~~|~~*100*~~|~~*98*~~||~~*100*~~|~~*82*~~|79||~~*100*~~|~~*100*~~|~~*90*~~|
|710|~~*100*~~|~~*86*~~|~~*86*~~||~~*100*~~|~~*100*~~|~~*98*~~||~~*100*~~|~~*82*~~|78||~~*100*~~|~~*100*~~|~~*88*~~|
|720|~~*100*~~|~~*86*~~|~~*86*~~||~~*100*~~|~~*100*~~|~~*98*~~||~~*100*~~|80|78||~~*100*~~|~~*100*~~|~~*88*~~|
|730|~~*100*~~|~~*86*~~|~~*85*~~||~~*100*~~|~~*100*~~|~~*96*~~||~~*100*~~|80|77||~~*100*~~|~~*100*~~|~~*87*~~|
|750|~~*100*~~|~~*85*~~|~~*85*~~||~~*100*~~|~~*100*~~|~~*96*~~||~~*100*~~|79|76||~~*100*~~|~~*100*~~|~~*86*~~|
|760|~~*100*~~|~~*85*~~|~~*84*~~||~~*100*~~|~~*100*~~|~~*95*~~||~~*98*~~|79|76||~~*100*~~|~~*100*~~|~~*86*~~|
|770|~~*100*~~|~~*85*~~|~~*84*~~||~~*100*~~|~~*100*~~|~~*95*~~||~~*96*~~|79|76||~~*100*~~|~~*100*~~|~~*86*~~|
|780|~~*100*~~|~~*85*~~|~~*83*~~||~~*100*~~|~~*100*~~|~~*94*~~||~~*96*~~|78|75||~~*100*~~|~~*99*~~|~~*84*~~|
|790|~~*100*~~|~~*84*~~|~~*83*~~||~~*100*~~|~~*100*~~|~~*94*~~||~~*94*~~|78|75||~~*100*~~|~~*99*~~|~~*84*~~|
|800|~~*100*~~|~~*84*~~|~~*83*~~||~~*100*~~|~~*100*~~|~~*94*~~||~~*92*~~|78|73||~~*100*~~|~~*99*~~|~~*83*~~|
|810|~~*100*~~|~~*84*~~|~~*82*~~||~~*100*~~|~~*100*~~|~~*92*~~||~~*92*~~|77|73||~~*100*~~|~~*98*~~|~~*83*~~|
|820|~~*100*~~|~~*83*~~|~~*82*~~||~~*100*~~|~~*100*~~|~~*92*~~||~~*90*~~|77|72||~~*100*~~|~~*98*~~|~~*82*~~|
|830|~~*100*~~|~~*83*~~|~~*82*~~||~~*100*~~|~~*100*~~|~~*92*~~||~~*88*~~|77|72||~~*100*~~|~~*98*~~|~~*82*~~|
|840|~~*100*~~|~~*83*~~|80||~~*100*~~|~~*100*~~|~~*91*~~||~~*88*~~|76|71||~~*100*~~|~~*96*~~|80|
|850|~~*100*~~|~~*83*~~|80||~~*100*~~|~~*100*~~|~~*91*~~||~~*86*~~|76|71||~~*100*~~|~~*96*~~|80|
|860|~~*100*~~|~~*82*~~|79||~~*100*~~|~~*100*~~|~~*90*~~||~~*84*~~|76|70||~~*100*~~|~~*96*~~|79|
|870|~~*100*~~|~~*82*~~|79||~~*100*~~|~~*100*~~|~~*90*~~||~~*84*~~|75|70||~~*100*~~|~~*95*~~|79|
|880|~~*100*~~|~~*82*~~|79||~~*100*~~|~~*100*~~|~~*90*~~||~~*82*~~|75|69||~~*100*~~|~~*95*~~|78|
|890|~~*100*~~|~~*82*~~|78||~~*100*~~|~~*100*~~|~~*88*~~||80|75|69||~~*100*~~|~~*95*~~|78|
|900|~~*100*~~|80|78||~~*100*~~|~~*100*~~|~~*88*~~||80|74|68||~~*100*~~|~~*94*~~|76|
|910|~~*100*~~|80|77||~~*100*~~|~~*100*~~|~~*87*~~||78|74|68||~~*100*~~|~~*94*~~|76|
|920|~~*100*~~|80|77||~~*100*~~|~~*100*~~|~~*87*~~||76|74|66||~~*100*~~|~~*94*~~|75|
|930|~~*100*~~|80|77||~~*100*~~|~~*100*~~|~~*87*~~||76|73|66||~~*100*~~|~~*92*~~|75|
|940|~~*100*~~|79|76||~~*100*~~|~~*100*~~|~~*86*~~||74|73|65||~~*100*~~|~~*92*~~|74|
|950|~~*98*~~|79|76||~~*100*~~|~~*100*~~|~~*86*~~||72|73|65||~~*100*~~|~~*92*~~|74|
|960|~~*96*~~|79|76||~~*100*~~|~~*100*~~|~~*86*~~||72|72|64||~~*100*~~|~~*91*~~|72|
|970|~~*96*~~|79|75||~~*100*~~|~~*100*~~|~~*84*~~||70|72|64||~~*100*~~|~~*91*~~|72|
|980|~~*94*~~|78|75||~~*100*~~|~~*99*~~|~~*84*~~||68|72|63||~~*100*~~|~~*91*~~|71|
|990|~~*94*~~|78|73||~~*100*~~|~~*99*~~|~~*83*~~||68|71|63||~~*100*~~|~~*90*~~|71|
|1000|~~*92*~~|78|73||~~*100*~~|~~*99*~~|~~*83*~~||67|71|62||~~*100*~~|~~*90*~~|70|
|1010|~~*92*~~|77|73||~~*100*~~|~~*98*~~|~~*83*~~||67|71|62||~~*100*~~|~~*90*~~|70|
|1020|~~*90*~~|77|72||~~*100*~~|~~*98*~~|~~*82*~~||66|70|60||~~*100*~~|~~*88*~~|68|
|1040|~~*88*~~|77|71||~~*100*~~|~~*98*~~|80||66|70|60||~~*100*~~|~~*88*~~|68|
|1050|~~*88*~~|76|71||~~*100*~~|~~*96*~~|80||65|69|59||~~*100*~~|~~*87*~~|67|
|1060|~~*86*~~|76|71||~~*100*~~|~~*96*~~|80||65|69|59||~~*100*~~|~~*87*~~|67|
|1070|~~*86*~~|76|70||~~*100*~~|~~*96*~~|79||65|69|58||~~*100*~~|~~*87*~~|66|
|1080|~~*84*~~|76|70||~~*100*~~|~~*96*~~|79||64|68|58||~~*100*~~|~~*86*~~|66|
|1090|~~*82*~~|75|70||~~*100*~~|~~*95*~~|79||64|68|57||~~*100*~~|~~*86*~~|64|
|1100|~~*82*~~|75|69||~~*100*~~|~~*95*~~|78||64|68|57||~~*100*~~|~~*86*~~|64|
|1110|80|75|69||~~*100*~~|~~*95*~~|78||63|67|56||~~*100*~~|~~*84*~~|63|
|1120|80|75|68||~~*100*~~|~~*95*~~|76||63|67|56||~~*100*~~|~~*84*~~|63|
|1130|78|74|68||~~*100*~~|~~*94*~~|76||63|67|55||~~*96*~~|~~*84*~~|62|
|1140|78|74|68||~~*100*~~|~~*94*~~|76||62|66|55||~~*92*~~|~~*83*~~|62|
|1150|76|74|66||~~*100*~~|~~*94*~~|75||62|66|53||~~*92*~~|~~*83*~~|60|
|1160|76|73|66||~~*100*~~|~~*92*~~|75||62|66|53||~~*88*~~|~~*83*~~|60|

### [Infernal Stronghold](https://wiki.melvoridle.com/index.php?title=Infernal_Stronghold), Auto Eat II

|❤️|⚔️|🏹|🧙||⚔️ ☠️|🏹 ☠️|🧙 ☠️||⚔️ 💍|🏹 💍|🧙 💍||⚔️ ☠️ 💍|🏹 ☠️ 💍|🧙 ☠️ 💍|
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|480|~~*100*~~|~~*86*~~|~~*86*~~||~~*100*~~|~~*100*~~|~~*98*~~||~~*100*~~|~~*83*~~|80||~~*100*~~|~~*100*~~|~~*91*~~|
|490|~~*100*~~|~~*86*~~|~~*85*~~||~~*100*~~|~~*100*~~|~~*96*~~||~~*100*~~|~~*83*~~|80||~~*100*~~|~~*100*~~|~~*91*~~|
|500|~~*100*~~|~~*85*~~|~~*85*~~||~~*100*~~|~~*100*~~|~~*96*~~||~~*100*~~|~~*82*~~|79||~~*100*~~|~~*100*~~|~~*90*~~|
|510|~~*100*~~|~~*85*~~|~~*84*~~||~~*100*~~|~~*100*~~|~~*95*~~||~~*100*~~|~~*82*~~|78||~~*100*~~|~~*100*~~|~~*88*~~|
|520|~~*100*~~|~~*85*~~|~~*83*~~||~~*100*~~|~~*100*~~|~~*94*~~||~~*100*~~|80|77||~~*100*~~|~~*100*~~|~~*87*~~|
|530|~~*100*~~|~~*84*~~|~~*83*~~||~~*100*~~|~~*100*~~|~~*94*~~||~~*100*~~|80|77||~~*100*~~|~~*100*~~|~~*87*~~|
|540|~~*100*~~|~~*84*~~|~~*82*~~||~~*100*~~|~~*100*~~|~~*92*~~||~~*98*~~|79|76||~~*100*~~|~~*100*~~|~~*86*~~|
|550|~~*100*~~|~~*83*~~|~~*82*~~||~~*100*~~|~~*100*~~|~~*92*~~||~~*96*~~|79|76||~~*100*~~|~~*100*~~|~~*86*~~|
|560|~~*100*~~|~~*83*~~|80||~~*100*~~|~~*100*~~|~~*91*~~||~~*94*~~|78|75||~~*100*~~|~~*99*~~|~~*84*~~|
|570|~~*100*~~|~~*83*~~|80||~~*100*~~|~~*100*~~|~~*91*~~||~~*94*~~|78|73||~~*100*~~|~~*99*~~|~~*83*~~|
|580|~~*100*~~|~~*82*~~|79||~~*100*~~|~~*100*~~|~~*90*~~||~~*92*~~|77|72||~~*100*~~|~~*98*~~|~~*82*~~|
|590|~~*100*~~|~~*82*~~|78||~~*100*~~|~~*100*~~|~~*88*~~||~~*90*~~|77|72||~~*100*~~|~~*98*~~|~~*82*~~|
|600|~~*100*~~|80|78||~~*100*~~|~~*100*~~|~~*88*~~||~~*88*~~|76|71||~~*100*~~|~~*96*~~|80|
|610|~~*100*~~|80|77||~~*100*~~|~~*100*~~|~~*87*~~||~~*86*~~|76|71||~~*100*~~|~~*96*~~|80|
|620|~~*100*~~|80|77||~~*100*~~|~~*100*~~|~~*87*~~||~~*84*~~|75|70||~~*100*~~|~~*95*~~|79|
|630|~~*98*~~|79|76||~~*100*~~|~~*100*~~|~~*86*~~||~~*82*~~|75|69||~~*100*~~|~~*95*~~|78|
|640|~~*96*~~|79|76||~~*100*~~|~~*100*~~|~~*86*~~||80|75|68||~~*100*~~|~~*95*~~|76|
|650|~~*96*~~|78|75||~~*100*~~|~~*99*~~|~~*84*~~||78|74|68||~~*100*~~|~~*94*~~|76|
|660|~~*94*~~|78|73||~~*100*~~|~~*99*~~|~~*83*~~||76|74|66||~~*100*~~|~~*94*~~|75|
|670|~~*92*~~|78|73||~~*100*~~|~~*99*~~|~~*83*~~||74|73|65||~~*100*~~|~~*92*~~|74|
|680|~~*90*~~|77|72||~~*100*~~|~~*98*~~|~~*82*~~||72|73|65||~~*100*~~|~~*92*~~|74|
|690|~~*88*~~|77|72||~~*100*~~|~~*98*~~|~~*82*~~||70|72|64||~~*100*~~|~~*91*~~|72|
|700|~~*88*~~|76|71||~~*100*~~|~~*96*~~|80||68|72|64||~~*100*~~|~~*91*~~|72|
|710|~~*86*~~|76|71||~~*100*~~|~~*96*~~|80||67|71|63||~~*100*~~|~~*90*~~|71|
|720|~~*84*~~|76|70||~~*100*~~|~~*96*~~|79||67|71|62||~~*100*~~|~~*90*~~|70|
|730|~~*82*~~|75|69||~~*100*~~|~~*95*~~|78||66|70|60||~~*100*~~|~~*88*~~|68|
|740|80|75|69||~~*100*~~|~~*95*~~|78||66|70|60||~~*100*~~|~~*88*~~|68|
|750|80|74|68||~~*100*~~|~~*94*~~|76||65|69|59||~~*100*~~|~~*87*~~|67|
|760|78|74|68||~~*100*~~|~~*94*~~|76||65|69|58||~~*100*~~|~~*87*~~|66|
|770|76|74|66||~~*100*~~|~~*94*~~|75||65|69|58||~~*100*~~|~~*87*~~|66|
|780|74|73|65||~~*100*~~|~~*92*~~|74||64|68|57||~~*100*~~|~~*86*~~|64|
|790|72|73|65||~~*100*~~|~~*92*~~|74||64|68|56||~~*100*~~|~~*86*~~|63|
|800|72|72|64||~~*100*~~|~~*91*~~|72||63|67|56||~~*100*~~|~~*84*~~|63|
|810|70|72|64||~~*100*~~|~~*91*~~|72||63|67|55||~~*96*~~|~~*84*~~|62|
|820|68|72|63||~~*100*~~|~~*91*~~|71||62|66|53||~~*92*~~|~~*83*~~|60|
|830|67|71|63||~~*100*~~|~~*90*~~|71||62|66|53||~~*88*~~|~~*83*~~|60|
|840|67|71|62||~~*100*~~|~~*90*~~|70||61|65|52||~~*84*~~|~~*82*~~|59|
|850|66|70|60||~~*100*~~|~~*88*~~|68||61|65|51||80|~~*82*~~|58|
|860|66|70|60||~~*100*~~|~~*88*~~|68||60|64|51||76|80|58|
|870|66|70|59||~~*100*~~|~~*88*~~|67||60|64|50||72|80|56|
|880|65|69|59||~~*100*~~|~~*87*~~|67||59|63|49||68|79|55|
|890|65|69|58||~~*100*~~|~~*87*~~|66||59|63|49||64|79|55|
|900|64|68|58||~~*100*~~|~~*86*~~|66||58|62|48||60|78|54|
|910|64|68|57||~~*100*~~|~~*86*~~|64||58|62|47||60|78|52|
|920|64|68|56||~~*100*~~|~~*86*~~|63||57|60|46||57|76|52|
|930|63|67|56||~~*100*~~|~~*84*~~|63||57|60|46||57|76|51|
|940|63|67|55||~~*96*~~|~~*84*~~|62||56|59|45||56|75|50|
|950|62|66|55||~~*92*~~|~~*83*~~|62||56|59|45||56|75|50|
|960|62|66|53||~~*92*~~|~~*83*~~|60||56|59|45||56|75|48|
|970|62|66|53||~~*88*~~|~~*83*~~|60||55|58|44||55|74|47|
|980|61|65|52||~~*84*~~|~~*82*~~|59||55|58|44||55|74|47|
|990|61|65|51||80|~~*82*~~|58||54|57|44||54|72|46|
|1000|60|64|51||76|80|58||54|57|44||54|72|44|
|1010|60|64|50||76|80|56||53|56|43||53|71|44|
|1020|60|64|50||72|80|56||53|56|43||53|71|43|
|1030|59|63|49||68|79|55||52|55|42||52|70|42|
|1040|59|63|49||64|79|55||52|55|42||52|70|42|
|1050|58|62|48||60|78|54||51|54|41||51|68|41|
|1060|58|62|47||60|78|52||51|54|41||51|68|41|
|1070|58|62|47||58|78|52||51|54|41||51|68|41|
|1080|57|60|46||57|76|51||50|53|40||50|67|40|
|1100|56|59|45||56|75|50||49|52|40||49|66|40|
|1120|56|59|45||56|75|48||48|51|39||48|64|39|
|1130|55|58|44||55|74|47||48|51|39||48|64|39|
|1140|55|58|44||55|74|47||47|50|38||47|63|38|
|1150|54|57|44||54|72|46||47|50|38||47|63|38|
|1160|54|57|44||54|72|46||46|49|37||46|62|37|

### [Infernal Stronghold](https://wiki.melvoridle.com/index.php?title=Infernal_Stronghold), Auto Eat III

|❤️|⚔️|🏹|🧙||⚔️ ☠️|🏹 ☠️|🧙 ☠️||⚔️ 💍|🏹 💍|🧙 💍||⚔️ ☠️ 💍|🏹 ☠️ 💍|🧙 ☠️ 💍|
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|380|~~*100*~~|~~*85*~~|~~*84*~~||~~*100*~~|~~*100*~~|~~*95*~~||~~*100*~~|~~*83*~~|80||~~*100*~~|~~*100*~~|~~*91*~~|
|390|~~*100*~~|~~*85*~~|~~*83*~~||~~*100*~~|~~*100*~~|~~*94*~~||~~*100*~~|~~*82*~~|79||~~*100*~~|~~*100*~~|~~*90*~~|
|400|~~*100*~~|~~*84*~~|~~*83*~~||~~*100*~~|~~*100*~~|~~*94*~~||~~*100*~~|80|78||~~*100*~~|~~*100*~~|~~*88*~~|
|410|~~*100*~~|~~*83*~~|~~*82*~~||~~*100*~~|~~*100*~~|~~*92*~~||~~*100*~~|80|77||~~*100*~~|~~*100*~~|~~*87*~~|
|420|~~*100*~~|~~*83*~~|80||~~*100*~~|~~*100*~~|~~*91*~~||~~*98*~~|79|76||~~*100*~~|~~*100*~~|~~*86*~~|
|430|~~*100*~~|~~*82*~~|79||~~*100*~~|~~*100*~~|~~*90*~~||~~*96*~~|79|75||~~*100*~~|~~*100*~~|~~*84*~~|
|440|~~*100*~~|~~*82*~~|79||~~*100*~~|~~*100*~~|~~*90*~~||~~*94*~~|78|73||~~*100*~~|~~*99*~~|~~*83*~~|
|450|~~*100*~~|80|78||~~*100*~~|~~*100*~~|~~*88*~~||~~*92*~~|77|73||~~*100*~~|~~*98*~~|~~*83*~~|
|460|~~*100*~~|80|77||~~*100*~~|~~*100*~~|~~*87*~~||~~*88*~~|77|72||~~*100*~~|~~*98*~~|~~*82*~~|
|470|~~*100*~~|79|76||~~*100*~~|~~*100*~~|~~*86*~~||~~*86*~~|76|71||~~*100*~~|~~*96*~~|80|
|480|~~*96*~~|79|76||~~*100*~~|~~*100*~~|~~*86*~~||~~*84*~~|76|70||~~*100*~~|~~*96*~~|79|
|490|~~*94*~~|78|75||~~*100*~~|~~*99*~~|~~*84*~~||~~*82*~~|75|69||~~*100*~~|~~*95*~~|78|
|500|~~*92*~~|78|73||~~*100*~~|~~*99*~~|~~*83*~~||80|74|68||~~*100*~~|~~*94*~~|76|
|510|~~*90*~~|77|72||~~*100*~~|~~*98*~~|~~*82*~~||78|74|66||~~*100*~~|~~*94*~~|75|
|520|~~*88*~~|77|71||~~*100*~~|~~*98*~~|80||74|73|65||~~*100*~~|~~*92*~~|74|
|530|~~*86*~~|76|71||~~*100*~~|~~*96*~~|80||72|73|65||~~*100*~~|~~*92*~~|74|
|540|~~*84*~~|76|70||~~*100*~~|~~*96*~~|79||70|72|64||~~*100*~~|~~*91*~~|72|
|550|~~*82*~~|75|69||~~*100*~~|~~*95*~~|78||68|71|63||~~*100*~~|~~*90*~~|71|
|560|80|75|68||~~*100*~~|~~*95*~~|76||67|71|62||~~*100*~~|~~*90*~~|70|
|570|78|74|68||~~*100*~~|~~*94*~~|76||66|70|60||~~*100*~~|~~*88*~~|68|
|580|76|73|66||~~*100*~~|~~*92*~~|75||66|70|59||~~*100*~~|~~*88*~~|67|
|590|74|73|65||~~*100*~~|~~*92*~~|74||65|69|59||~~*100*~~|~~*87*~~|67|
|600|72|72|64||~~*100*~~|~~*91*~~|72||64|68|58||~~*100*~~|~~*86*~~|66|
|610|68|72|64||~~*100*~~|~~*91*~~|72||64|68|57||~~*100*~~|~~*86*~~|64|
|620|67|71|63||~~*100*~~|~~*90*~~|71||63|67|56||~~*100*~~|~~*84*~~|63|
|630|67|71|62||~~*100*~~|~~*90*~~|70||63|67|55||~~*96*~~|~~*84*~~|62|
|640|66|70|60||~~*100*~~|~~*88*~~|68||62|66|53||~~*92*~~|~~*83*~~|60|
|650|66|70|60||~~*100*~~|~~*88*~~|68||61|65|52||~~*88*~~|~~*82*~~|59|
|660|65|69|59||~~*100*~~|~~*87*~~|67||61|65|51||80|~~*82*~~|58|
|670|65|69|58||~~*100*~~|~~*87*~~|66||60|64|51||76|80|58|
|680|64|68|57||~~*100*~~|~~*86*~~|64||60|64|50||72|80|56|
|690|64|68|56||~~*100*~~|~~*86*~~|63||59|63|49||68|79|55|
|700|63|67|56||~~*100*~~|~~*84*~~|63||58|62|48||60|78|54|
|710|63|67|55||~~*96*~~|~~*84*~~|62||58|62|47||58|78|52|
|720|62|66|53||~~*92*~~|~~*83*~~|60||57|60|46||57|76|51|
|730|61|65|52||~~*88*~~|~~*82*~~|59||57|60|46||57|76|50|
|740|61|65|52||80|~~*82*~~|59||56|59|45||56|75|50|
|750|60|64|51||76|80|58||55|58|44||55|74|48|
|760|60|64|50||72|80|56||55|58|44||55|74|47|
|770|59|63|49||68|79|55||54|57|44||54|72|46|
|780|59|63|49||64|79|55||54|57|44||54|72|44|
|790|58|62|48||60|78|54||53|56|43||53|71|43|
|800|58|62|47||58|78|52||52|55|42||52|70|42|
|810|57|60|46||57|76|51||52|55|42||52|70|42|
|820|57|60|46||57|76|50||51|54|41||51|68|41|
|830|56|59|45||56|75|50||51|54|41||51|68|41|
|840|56|59|45||56|75|48||50|53|40||50|67|40|
|850|55|58|44||55|74|47||49|52|40||49|66|40|
|860|55|58|44||55|74|46||49|52|40||49|66|40|
|870|54|57|44||54|72|46||48|51|39||48|64|39|
|880|53|56|43||53|71|44||48|51|39||48|64|39|
|890|53|56|43||53|71|43||47|50|38||47|63|38|
|900|52|55|42||52|70|42||46|49|37||46|62|37|
|920|51|54|41||51|68|41||45|48|36||45|60|36|
|940|50|53|40||50|67|40||44|47|36||44|59|36|
|950|50|53|40||50|67|40||43|46|35||43|58|35|
|960|49|52|40||49|66|40||43|46|35||43|58|35|
|970|49|52|40||49|66|40||42|45|34||42|56|34|
|980|48|51|39||48|64|39||42|45|34||42|56|34|
|990|48|51|39||48|64|39||41|44|33||41|55|33|
|1000|47|50|38||47|63|38||40|43|32||40|54|32|
|1020|46|49|37||46|62|37||39|42|32||39|52|32|
|1030|45|48|36||45|60|36||39|42|32||39|52|32|
|1040|45|48|36||45|60|36||38|40|31||38|51|31|
|1050|44|47|36||44|59|36||37|39|30||37|50|30|
|1070|43|46|35||43|58|35||36|38|29||36|48|29|
|1090|42|45|34||42|56|34||35|37|28||35|47|28|
|1100|42|45|34||42|56|34||34|36|28||34|46|28|
|1110|41|44|33||41|55|33||34|36|28||34|46|28|
|1120|41|44|33||41|55|33||33|35|27||33|44|27|
|1130|40|43|32||40|54|32||33|35|27||33|44|27|
|1140|40|43|32||40|54|32||32|34|26||32|43|26|
|1150|39|42|32||39|52|32||31|33|25||31|42|25|
|1160|39|42|32||39|52|32||31|33|25||31|42|25|

### [Air God Dungeon](https://wiki.melvoridle.com/index.php?title=Air_God_Dungeon), Auto Eat I

|❤️|⚔️|🏹|🧙||⚔️ ☠️|🏹 ☠️|🧙 ☠️||⚔️ 💍|🏹 💍|🧙 💍||⚔️ ☠️ 💍|🏹 ☠️ 💍|🧙 ☠️ 💍|
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|530|~~*85*~~|~~*90*~~|~~*100*~~||~~*85*~~|~~*100*~~|~~*100*~~||81|~~*86*~~|~~*97*~~||81|~~*100*~~|~~*100*~~|
|540|~~*85*~~|~~*90*~~|~~*100*~~||~~*85*~~|~~*100*~~|~~*100*~~||81|~~*86*~~|~~*96*~~||81|~~*100*~~|~~*100*~~|
|560|~~*84*~~|~~*89*~~|~~*100*~~||~~*84*~~|~~*100*~~|~~*100*~~||80|~~*85*~~|~~*96*~~||80|~~*100*~~|~~*100*~~|
|570|~~*84*~~|~~*89*~~|~~*99*~~||~~*84*~~|~~*100*~~|~~*100*~~||80|~~*85*~~|~~*95*~~||80|~~*100*~~|~~*100*~~|
|590|~~*83*~~|~~*88*~~|~~*99*~~||~~*83*~~|~~*100*~~|~~*100*~~||79|~~*84*~~|~~*95*~~||79|~~*100*~~|~~*100*~~|
|600|~~*83*~~|~~*88*~~|~~*99*~~||~~*83*~~|~~*100*~~|~~*100*~~||79|~~*84*~~|~~*93*~~||79|~~*100*~~|~~*100*~~|
|610|~~*83*~~|~~*88*~~|~~*98*~~||~~*83*~~|~~*100*~~|~~*100*~~||79|~~*84*~~|~~*93*~~||79|~~*100*~~|~~*100*~~|
|620|~~*83*~~|~~*88*~~|~~*98*~~||~~*83*~~|~~*100*~~|~~*100*~~||78|~~*83*~~|~~*93*~~||78|~~*100*~~|~~*100*~~|
|630|~~*82*~~|~~*87*~~|~~*98*~~||~~*82*~~|~~*100*~~|~~*100*~~||78|~~*83*~~|~~*92*~~||78|~~*100*~~|~~*100*~~|
|640|~~*82*~~|~~*87*~~|~~*97*~~||~~*82*~~|~~*100*~~|~~*100*~~||77|~~*82*~~|~~*92*~~||77|~~*100*~~|~~*100*~~|
|660|81|~~*86*~~|~~*97*~~||81|~~*100*~~|~~*100*~~||77|~~*82*~~|~~*91*~~||77|~~*100*~~|~~*100*~~|
|670|81|~~*86*~~|~~*97*~~||81|~~*100*~~|~~*100*~~||76|80|~~*91*~~||76|~~*100*~~|~~*100*~~|
|680|81|~~*86*~~|~~*96*~~||81|~~*100*~~|~~*100*~~||76|80|~~*91*~~||76|~~*100*~~|~~*100*~~|
|690|81|~~*86*~~|~~*96*~~||81|~~*100*~~|~~*100*~~||76|80|~~*90*~~||76|~~*100*~~|~~*100*~~|
|700|80|~~*85*~~|~~*96*~~||80|~~*100*~~|~~*100*~~||75|79|~~*90*~~||75|~~*100*~~|~~*100*~~|
|710|80|~~*85*~~|~~*95*~~||80|~~*100*~~|~~*100*~~||75|79|~~*90*~~||75|~~*100*~~|~~*100*~~|
|720|80|~~*85*~~|~~*95*~~||80|~~*100*~~|~~*100*~~||75|79|~~*89*~~||75|~~*100*~~|~~*100*~~|
|730|79|~~*84*~~|~~*95*~~||79|~~*100*~~|~~*100*~~||74|78|~~*89*~~||74|~~*99*~~|~~*100*~~|
|740|79|~~*84*~~|~~*95*~~||79|~~*100*~~|~~*100*~~||74|78|~~*88*~~||74|~~*99*~~|~~*99*~~|
|750|79|~~*84*~~|~~*93*~~||79|~~*100*~~|~~*100*~~||74|78|~~*88*~~||74|~~*99*~~|~~*99*~~|
|760|79|~~*84*~~|~~*93*~~||79|~~*100*~~|~~*100*~~||73|77|~~*88*~~||73|~~*98*~~|~~*99*~~|
|770|78|~~*83*~~|~~*93*~~||78|~~*100*~~|~~*100*~~||73|77|~~*86*~~||73|~~*98*~~|~~*98*~~|
|780|78|~~*83*~~|~~*92*~~||78|~~*100*~~|~~*100*~~||72|76|~~*86*~~||72|~~*96*~~|~~*98*~~|
|800|77|~~*82*~~|~~*92*~~||77|~~*100*~~|~~*100*~~||72|76|~~*85*~~||72|~~*96*~~|~~*96*~~|
|810|77|~~*82*~~|~~*92*~~||77|~~*100*~~|~~*100*~~||71|75|~~*85*~~||71|~~*95*~~|~~*96*~~|
|820|77|~~*82*~~|~~*91*~~||77|~~*100*~~|~~*100*~~||71|75|~~*85*~~||71|~~*95*~~|~~*96*~~|
|830|77|~~*82*~~|~~*91*~~||77|~~*100*~~|~~*100*~~||71|75|~~*84*~~||71|~~*95*~~|~~*95*~~|
|840|76|80|~~*91*~~||76|~~*100*~~|~~*100*~~||70|74|~~*84*~~||70|~~*94*~~|~~*95*~~|
|860|76|80|~~*90*~~||76|~~*100*~~|~~*100*~~||70|74|~~*83*~~||70|~~*94*~~|~~*94*~~|
|870|75|79|~~*90*~~||75|~~*100*~~|~~*100*~~||69|73|~~*83*~~||69|~~*92*~~|~~*94*~~|
|890|75|79|~~*89*~~||75|~~*100*~~|~~*100*~~||69|73|~~*82*~~||69|~~*92*~~|~~*92*~~|
|900|75|79|~~*89*~~||75|~~*100*~~|~~*100*~~||68|72|~~*82*~~||68|~~*91*~~|~~*92*~~|
|910|74|78|~~*89*~~||74|~~*99*~~|~~*100*~~||68|72|~~*82*~~||68|~~*91*~~|~~*92*~~|
|920|74|78|~~*89*~~||74|~~*99*~~|~~*100*~~||67|71|80||67|~~*90*~~|~~*91*~~|
|930|74|78|~~*88*~~||74|~~*99*~~|~~*99*~~||67|71|80||67|~~*90*~~|~~*91*~~|
|940|73|77|~~*88*~~||73|~~*98*~~|~~*99*~~||67|71|79||67|~~*90*~~|~~*90*~~|
|950|73|77|~~*88*~~||73|~~*98*~~|~~*99*~~||66|70|79||66|~~*88*~~|~~*90*~~|
|960|73|77|~~*86*~~||73|~~*98*~~|~~*98*~~||66|70|79||66|~~*88*~~|~~*90*~~|
|970|73|77|~~*86*~~||73|~~*98*~~|~~*98*~~||66|70|78||66|~~*88*~~|~~*88*~~|
|980|72|76|~~*86*~~||72|~~*96*~~|~~*98*~~||65|69|78||65|~~*87*~~|~~*88*~~|
|1000|72|76|~~*85*~~||72|~~*96*~~|~~*96*~~||65|69|77||65|~~*87*~~|~~*87*~~|
|1010|71|75|~~*85*~~||71|~~*95*~~|~~*96*~~||64|68|77||64|~~*86*~~|~~*87*~~|
|1030|71|75|~~*84*~~||71|~~*95*~~|~~*95*~~||64|68|76||64|~~*86*~~|~~*86*~~|
|1040|71|75|~~*84*~~||71|~~*95*~~|~~*95*~~||63|67|76||63|~~*84*~~|~~*86*~~|
|1050|70|74|~~*84*~~||70|~~*94*~~|~~*95*~~||63|67|76||63|~~*84*~~|~~*86*~~|
|1060|70|74|~~*84*~~||70|~~*94*~~|~~*95*~~||62|66|75||62|~~*83*~~|~~*84*~~|
|1070|70|74|~~*83*~~||70|~~*94*~~|~~*94*~~||62|66|75||62|~~*83*~~|~~*84*~~|
|1080|69|73|~~*83*~~||69|~~*92*~~|~~*94*~~||62|66|73||62|~~*83*~~|~~*83*~~|
|1090|69|73|~~*83*~~||69|~~*92*~~|~~*94*~~||61|65|73||61|~~*82*~~|~~*83*~~|
|1110|69|73|~~*82*~~||69|~~*92*~~|~~*92*~~||61|65|73||61|~~*82*~~|~~*83*~~|
|1120|68|72|~~*82*~~||68|~~*91*~~|~~*92*~~||60|64|72||60|80|~~*82*~~|
|1140|68|72|80||68|~~*91*~~|~~*91*~~||60|64|71||60|80|80|
|1150|67|71|80||67|~~*90*~~|~~*91*~~||59|63|71||59|79|80|
|1160|67|71|80||67|~~*90*~~|~~*91*~~||59|63|71||59|79|80|

### [Air God Dungeon](https://wiki.melvoridle.com/index.php?title=Air_God_Dungeon), Auto Eat II

|❤️|⚔️|🏹|🧙||⚔️ ☠️|🏹 ☠️|🧙 ☠️||⚔️ 💍|🏹 💍|🧙 💍||⚔️ ☠️ 💍|🏹 ☠️ 💍|🧙 ☠️ 💍|
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|380|~~*84*~~|~~*89*~~|~~*99*~~||~~*84*~~|~~*100*~~|~~*100*~~||81|~~*86*~~|~~*97*~~||81|~~*100*~~|~~*100*~~|
|390|~~*84*~~|~~*89*~~|~~*99*~~||~~*84*~~|~~*100*~~|~~*100*~~||81|~~*86*~~|~~*96*~~||81|~~*100*~~|~~*100*~~|
|400|~~*83*~~|~~*88*~~|~~*99*~~||~~*83*~~|~~*100*~~|~~*100*~~||80|~~*85*~~|~~*96*~~||80|~~*100*~~|~~*100*~~|
|410|~~*83*~~|~~*88*~~|~~*98*~~||~~*83*~~|~~*100*~~|~~*100*~~||80|~~*85*~~|~~*95*~~||80|~~*100*~~|~~*100*~~|
|420|~~*82*~~|~~*87*~~|~~*98*~~||~~*82*~~|~~*100*~~|~~*100*~~||79|~~*84*~~|~~*95*~~||79|~~*100*~~|~~*100*~~|
|430|~~*82*~~|~~*87*~~|~~*97*~~||~~*82*~~|~~*100*~~|~~*100*~~||79|~~*84*~~|~~*93*~~||79|~~*100*~~|~~*100*~~|
|440|81|~~*86*~~|~~*97*~~||81|~~*100*~~|~~*100*~~||78|~~*83*~~|~~*93*~~||78|~~*100*~~|~~*100*~~|
|450|81|~~*86*~~|~~*96*~~||81|~~*100*~~|~~*100*~~||78|~~*83*~~|~~*92*~~||78|~~*100*~~|~~*100*~~|
|460|81|~~*86*~~|~~*96*~~||81|~~*100*~~|~~*100*~~||77|~~*82*~~|~~*92*~~||77|~~*100*~~|~~*100*~~|
|470|80|~~*85*~~|~~*96*~~||80|~~*100*~~|~~*100*~~||77|~~*82*~~|~~*91*~~||77|~~*100*~~|~~*100*~~|
|480|80|~~*85*~~|~~*95*~~||80|~~*100*~~|~~*100*~~||76|80|~~*91*~~||76|~~*100*~~|~~*100*~~|
|490|79|~~*84*~~|~~*95*~~||79|~~*100*~~|~~*100*~~||76|80|~~*90*~~||76|~~*100*~~|~~*100*~~|
|500|79|~~*84*~~|~~*93*~~||79|~~*100*~~|~~*100*~~||75|79|~~*90*~~||75|~~*100*~~|~~*100*~~|
|510|78|~~*83*~~|~~*93*~~||78|~~*100*~~|~~*100*~~||75|79|~~*89*~~||75|~~*100*~~|~~*100*~~|
|520|78|~~*83*~~|~~*92*~~||78|~~*100*~~|~~*100*~~||74|78|~~*89*~~||74|~~*99*~~|~~*100*~~|
|530|78|~~*83*~~|~~*92*~~||78|~~*100*~~|~~*100*~~||74|78|~~*88*~~||74|~~*99*~~|~~*99*~~|
|540|77|~~*82*~~|~~*92*~~||77|~~*100*~~|~~*100*~~||73|77|~~*88*~~||73|~~*98*~~|~~*99*~~|
|550|77|~~*82*~~|~~*91*~~||77|~~*100*~~|~~*100*~~||73|77|~~*86*~~||73|~~*98*~~|~~*98*~~|
|560|76|80|~~*91*~~||76|~~*100*~~|~~*100*~~||72|76|~~*86*~~||72|~~*96*~~|~~*98*~~|
|570|76|80|~~*90*~~||76|~~*100*~~|~~*100*~~||72|76|~~*85*~~||72|~~*96*~~|~~*96*~~|
|580|75|79|~~*90*~~||75|~~*100*~~|~~*100*~~||71|75|~~*85*~~||71|~~*95*~~|~~*96*~~|
|590|75|79|~~*90*~~||75|~~*100*~~|~~*100*~~||71|75|~~*84*~~||71|~~*95*~~|~~*95*~~|
|600|75|79|~~*89*~~||75|~~*100*~~|~~*100*~~||70|74|~~*84*~~||70|~~*94*~~|~~*95*~~|
|610|74|78|~~*89*~~||74|~~*99*~~|~~*100*~~||70|74|~~*83*~~||70|~~*94*~~|~~*94*~~|
|620|74|78|~~*88*~~||74|~~*99*~~|~~*99*~~||69|73|~~*83*~~||69|~~*92*~~|~~*94*~~|
|630|73|77|~~*88*~~||73|~~*98*~~|~~*99*~~||69|73|~~*83*~~||69|~~*92*~~|~~*94*~~|
|640|73|77|~~*86*~~||73|~~*98*~~|~~*98*~~||68|72|~~*82*~~||68|~~*91*~~|~~*92*~~|
|650|72|76|~~*86*~~||72|~~*96*~~|~~*98*~~||68|72|~~*82*~~||68|~~*91*~~|~~*92*~~|
|660|72|76|~~*86*~~||72|~~*96*~~|~~*98*~~||67|71|80||67|~~*90*~~|~~*91*~~|
|670|72|76|~~*85*~~||72|~~*96*~~|~~*96*~~||67|71|80||67|~~*90*~~|~~*91*~~|
|680|71|75|~~*85*~~||71|~~*95*~~|~~*96*~~||66|70|79||66|~~*88*~~|~~*90*~~|
|690|71|75|~~*84*~~||71|~~*95*~~|~~*95*~~||66|70|79||66|~~*88*~~|~~*90*~~|
|700|70|74|~~*84*~~||70|~~*94*~~|~~*95*~~||65|69|78||65|~~*87*~~|~~*88*~~|
|710|70|74|~~*83*~~||70|~~*94*~~|~~*94*~~||65|69|78||65|~~*87*~~|~~*88*~~|
|720|69|73|~~*83*~~||69|~~*92*~~|~~*94*~~||64|68|77||64|~~*86*~~|~~*87*~~|
|740|69|73|~~*82*~~||69|~~*92*~~|~~*92*~~||63|67|76||63|~~*84*~~|~~*86*~~|
|750|68|72|~~*82*~~||68|~~*91*~~|~~*92*~~||63|67|76||63|~~*84*~~|~~*86*~~|
|760|68|72|80||68|~~*91*~~|~~*91*~~||62|66|75||62|~~*83*~~|~~*84*~~|
|770|67|71|80||67|~~*90*~~|~~*91*~~||62|66|75||62|~~*83*~~|~~*84*~~|
|780|67|71|80||67|~~*90*~~|~~*91*~~||61|65|73||61|~~*82*~~|~~*83*~~|
|790|66|70|79||66|~~*88*~~|~~*90*~~||61|65|73||61|~~*82*~~|~~*83*~~|
|800|66|70|79||66|~~*88*~~|~~*90*~~||60|64|72||60|80|~~*82*~~|
|810|66|70|78||66|~~*88*~~|~~*88*~~||60|64|72||60|80|~~*82*~~|
|820|65|69|78||65|~~*87*~~|~~*88*~~||59|63|71||59|79|80|
|830|65|69|77||65|~~*87*~~|~~*87*~~||59|63|71||59|79|80|
|840|64|68|77||64|~~*86*~~|~~*87*~~||58|62|70||58|78|79|
|860|63|67|76||63|~~*84*~~|~~*86*~~||57|60|69||57|76|78|
|880|63|67|75||63|~~*84*~~|~~*84*~~||56|59|68||56|75|76|
|890|62|66|75||62|~~*83*~~|~~*84*~~||56|59|68||56|75|76|
|900|62|66|73||62|~~*83*~~|~~*83*~~||55|58|66||55|74|75|
|910|61|65|73||61|~~*82*~~|~~*83*~~||55|58|66||55|74|75|
|920|61|65|73||61|~~*82*~~|~~*83*~~||54|57|65||54|72|74|
|930|60|64|72||60|80|~~*82*~~||54|57|65||54|72|74|
|940|60|64|72||60|80|~~*82*~~||53|56|64||53|71|72|
|950|60|64|71||60|80|80||53|56|64||53|71|72|
|960|59|63|71||59|79|80||52|55|63||52|70|71|
|980|58|62|70||58|78|79||51|54|62||51|68|70|
|1000|57|60|69||57|76|78||50|53|60||50|67|68|
|1020|57|60|68||57|76|76||49|52|59||49|66|67|
|1030|56|59|68||56|75|76||49|52|59||49|66|67|
|1040|56|59|68||56|75|76||48|51|58||48|64|66|
|1050|55|58|66||55|74|75||48|51|58||48|64|66|
|1060|55|58|66||55|74|75||47|50|57||47|63|64|
|1070|54|57|65||54|72|74||47|50|57||47|63|64|
|1080|54|57|65||54|72|74||46|49|56||46|62|63|
|1090|54|57|64||54|72|72||46|49|56||46|62|63|
|1100|53|56|64||53|71|72||45|48|55||45|60|62|
|1120|52|55|63||52|70|71||44|47|53||44|59|60|
|1140|51|54|62||51|68|70||43|46|52||43|58|59|
|1160|51|54|62||51|68|70||42|45|51||42|56|58|

### [Air God Dungeon](https://wiki.melvoridle.com/index.php?title=Air_God_Dungeon), Auto Eat III

|❤️|⚔️|🏹|🧙||⚔️ ☠️|🏹 ☠️|🧙 ☠️||⚔️ 💍|🏹 💍|🧙 💍||⚔️ ☠️ 💍|🏹 ☠️ 💍|🧙 ☠️ 💍|
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|300|~~*83*~~|~~*88*~~|~~*99*~~||~~*83*~~|~~*100*~~|~~*100*~~||81|~~*86*~~|~~*96*~~||81|~~*100*~~|~~*100*~~|
|310|~~*83*~~|~~*88*~~|~~*98*~~||~~*83*~~|~~*100*~~|~~*100*~~||80|~~*85*~~|~~*96*~~||80|~~*100*~~|~~*100*~~|
|320|~~*82*~~|~~*87*~~|~~*97*~~||~~*82*~~|~~*100*~~|~~*100*~~||80|~~*85*~~|~~*95*~~||80|~~*100*~~|~~*100*~~|
|330|81|~~*86*~~|~~*97*~~||81|~~*100*~~|~~*100*~~||79|~~*84*~~|~~*95*~~||79|~~*100*~~|~~*100*~~|
|340|81|~~*86*~~|~~*96*~~||81|~~*100*~~|~~*100*~~||78|~~*83*~~|~~*93*~~||78|~~*100*~~|~~*100*~~|
|350|80|~~*85*~~|~~*96*~~||80|~~*100*~~|~~*100*~~||78|~~*83*~~|~~*92*~~||78|~~*100*~~|~~*100*~~|
|360|80|~~*85*~~|~~*95*~~||80|~~*100*~~|~~*100*~~||77|~~*82*~~|~~*92*~~||77|~~*100*~~|~~*100*~~|
|370|79|~~*84*~~|~~*95*~~||79|~~*100*~~|~~*100*~~||77|~~*82*~~|~~*91*~~||77|~~*100*~~|~~*100*~~|
|380|79|~~*84*~~|~~*93*~~||79|~~*100*~~|~~*100*~~||76|80|~~*90*~~||76|~~*100*~~|~~*100*~~|
|390|78|~~*83*~~|~~*92*~~||78|~~*100*~~|~~*100*~~||75|79|~~*90*~~||75|~~*100*~~|~~*100*~~|
|400|77|~~*82*~~|~~*92*~~||77|~~*100*~~|~~*100*~~||75|79|~~*89*~~||75|~~*100*~~|~~*100*~~|
|410|77|~~*82*~~|~~*91*~~||77|~~*100*~~|~~*100*~~||74|78|~~*89*~~||74|~~*99*~~|~~*100*~~|
|420|76|80|~~*91*~~||76|~~*100*~~|~~*100*~~||73|77|~~*88*~~||73|~~*98*~~|~~*99*~~|
|430|76|80|~~*90*~~||76|~~*100*~~|~~*100*~~||73|77|~~*86*~~||73|~~*98*~~|~~*98*~~|
|440|75|79|~~*90*~~||75|~~*100*~~|~~*100*~~||72|76|~~*86*~~||72|~~*96*~~|~~*98*~~|
|450|75|79|~~*89*~~||75|~~*100*~~|~~*100*~~||71|75|~~*85*~~||71|~~*95*~~|~~*96*~~|
|460|74|78|~~*89*~~||74|~~*99*~~|~~*100*~~||71|75|~~*84*~~||71|~~*95*~~|~~*95*~~|
|470|73|77|~~*88*~~||73|~~*98*~~|~~*99*~~||70|74|~~*84*~~||70|~~*94*~~|~~*95*~~|
|480|73|77|~~*86*~~||73|~~*98*~~|~~*98*~~||69|73|~~*83*~~||69|~~*92*~~|~~*94*~~|
|490|72|76|~~*86*~~||72|~~*96*~~|~~*98*~~||69|73|~~*83*~~||69|~~*92*~~|~~*94*~~|
|500|72|76|~~*85*~~||72|~~*96*~~|~~*96*~~||68|72|~~*82*~~||68|~~*91*~~|~~*92*~~|
|510|71|75|~~*85*~~||71|~~*95*~~|~~*96*~~||68|72|80||68|~~*91*~~|~~*91*~~|
|520|71|75|~~*84*~~||71|~~*95*~~|~~*95*~~||67|71|80||67|~~*90*~~|~~*91*~~|
|530|70|74|~~*84*~~||70|~~*94*~~|~~*95*~~||66|70|79||66|~~*88*~~|~~*90*~~|
|540|69|73|~~*83*~~||69|~~*92*~~|~~*94*~~||66|70|78||66|~~*88*~~|~~*88*~~|
|550|69|73|~~*83*~~||69|~~*92*~~|~~*94*~~||65|69|78||65|~~*87*~~|~~*88*~~|
|560|68|72|~~*82*~~||68|~~*91*~~|~~*92*~~||64|68|77||64|~~*86*~~|~~*87*~~|
|570|68|72|80||68|~~*91*~~|~~*91*~~||64|68|76||64|~~*86*~~|~~*86*~~|
|580|67|71|80||67|~~*90*~~|~~*91*~~||63|67|76||63|~~*84*~~|~~*86*~~|
|590|67|71|79||67|~~*90*~~|~~*90*~~||62|66|75||62|~~*83*~~|~~*84*~~|
|600|66|70|79||66|~~*88*~~|~~*90*~~||62|66|73||62|~~*83*~~|~~*83*~~|
|610|65|69|78||65|~~*87*~~|~~*88*~~||61|65|73||61|~~*82*~~|~~*83*~~|
|620|65|69|78||65|~~*87*~~|~~*88*~~||60|64|72||60|80|~~*82*~~|
|630|64|68|77||64|~~*86*~~|~~*87*~~||60|64|72||60|80|~~*82*~~|
|640|64|68|76||64|~~*86*~~|~~*86*~~||59|63|71||59|79|80|
|650|63|67|76||63|~~*84*~~|~~*86*~~||59|63|70||59|79|79|
|660|63|67|75||63|~~*84*~~|~~*84*~~||58|62|70||58|78|79|
|670|62|66|75||62|~~*83*~~|~~*84*~~||57|60|69||57|76|78|
|680|61|65|73||61|~~*82*~~|~~*83*~~||57|60|68||57|76|76|
|690|61|65|73||61|~~*82*~~|~~*83*~~||56|59|68||56|75|76|
|700|60|64|72||60|80|~~*82*~~||55|58|66||55|74|75|
|720|59|63|71||59|79|80||54|57|65||54|72|74|
|730|59|63|70||59|79|79||53|56|64||53|71|72|
|740|58|62|70||58|78|79||53|56|64||53|71|72|
|750|57|60|69||57|76|78||52|55|63||52|70|71|
|760|57|60|69||57|76|78||51|54|62||51|68|70|
|770|56|59|68||56|75|76||51|54|62||51|68|70|
|780|56|59|68||56|75|76||50|53|60||50|67|68|
|790|55|58|66||55|74|75||50|53|60||50|67|68|
|800|55|58|65||55|74|74||49|52|59||49|66|67|
|810|54|57|65||54|72|74||48|51|58||48|64|66|
|820|53|56|64||53|71|72||48|51|58||48|64|66|
|830|53|56|64||53|71|72||47|50|57||47|63|64|
|840|52|55|63||52|70|71||46|49|56||46|62|63|
|860|51|54|62||51|68|70||45|48|55||45|60|62|
|870|51|54|62||51|68|70||44|47|55||44|59|62|
|880|50|53|60||50|67|68||44|47|53||44|59|60|
|890|49|52|59||49|66|67||43|46|52||43|58|59|
|900|49|52|59||49|66|67||42|45|52||42|56|59|
|910|48|51|58||48|64|66||42|45|51||42|56|58|
|920|48|51|58||48|64|66||41|44|50||41|55|56|
|930|47|50|57||47|63|64||41|44|50||41|55|56|
|940|47|50|57||47|63|64||40|43|49||40|54|55|
|950|46|49|56||46|62|63||39|42|48||39|52|54|
|960|45|48|56||45|60|63||39|42|48||39|52|54|
|970|45|48|55||45|60|62||38|40|46||38|51|52|
|980|44|47|53||44|59|60||37|39|46||37|50|52|
|990|44|47|53||44|59|60||37|39|45||37|50|51|
|1000|43|46|52||43|58|59||36|38|44||36|48|50|
|1010|43|46|52||43|58|59||35|37|44||35|47|50|
|1020|42|45|51||42|56|58||35|37|43||35|47|48|
|1030|41|44|51||41|55|58||34|36|42||34|46|47|
|1040|41|44|50||41|55|56||33|35|42||33|44|47|
|1050|40|43|49||40|54|55||33|35|40||33|44|46|
|1060|40|43|49||40|54|55||32|34|39||32|43|44|
|1070|39|42|48||39|52|54||32|34|39||32|43|44|
|1080|39|42|48||39|52|54||31|33|38||31|42|43|
|1090|38|40|46||38|51|52||30|32|38||30|40|43|
|1100|37|39|46||37|50|52||30|32|37||30|40|42|
|1110|37|39|45||37|50|51||29|31|36||29|39|40|
|1120|36|38|45||36|48|51||28|30|36||28|38|40|
|1130|36|38|44||36|48|50||28|30|35||28|38|39|
|1140|35|37|43||35|47|48||27|29|33||27|36|38|
|1150|35|37|43||35|47|48||26|28|33||26|35|38|
|1160|34|36|42||34|46|47||26|28|32||26|35|36|

### [Water God Dungeon](https://wiki.melvoridle.com/index.php?title=Water_God_Dungeon), Auto Eat I

|❤️|⚔️|🏹|🧙||⚔️ ☠️|🏹 ☠️|🧙 ☠️||⚔️ 💍|🏹 💍|🧙 💍||⚔️ ☠️ 💍|🏹 ☠️ 💍|🧙 ☠️ 💍|
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|700|~~*100*~~|~~*86*~~|~~*87*~~||~~*100*~~|~~*100*~~|~~*87*~~||~~*100*~~|80|~~*84*~~||~~*100*~~|~~*100*~~|~~*84*~~|
|730|~~*100*~~|~~*85*~~|~~*87*~~||~~*100*~~|~~*100*~~|~~*87*~~||~~*100*~~|80|~~*84*~~||~~*100*~~|~~*100*~~|~~*84*~~|
|740|~~*100*~~|~~*85*~~|~~*87*~~||~~*100*~~|~~*100*~~|~~*87*~~||~~*100*~~|79|~~*83*~~||~~*100*~~|~~*100*~~|~~*83*~~|
|760|~~*100*~~|~~*85*~~|~~*86*~~||~~*100*~~|~~*100*~~|~~*86*~~||~~*100*~~|78|~~*83*~~||~~*100*~~|~~*99*~~|~~*83*~~|
|770|~~*100*~~|~~*84*~~|~~*86*~~||~~*100*~~|~~*100*~~|~~*86*~~||~~*100*~~|78|~~*83*~~||~~*100*~~|~~*99*~~|~~*83*~~|
|780|~~*100*~~|~~*84*~~|~~*86*~~||~~*100*~~|~~*100*~~|~~*86*~~||~~*100*~~|78|~~*82*~~||~~*100*~~|~~*99*~~|~~*82*~~|
|790|~~*100*~~|~~*84*~~|~~*86*~~||~~*100*~~|~~*100*~~|~~*86*~~||~~*100*~~|77|~~*82*~~||~~*100*~~|~~*98*~~|~~*82*~~|
|810|~~*100*~~|~~*83*~~|~~*85*~~||~~*100*~~|~~*100*~~|~~*85*~~||~~*100*~~|77|~~*82*~~||~~*100*~~|~~*98*~~|~~*82*~~|
|820|~~*100*~~|~~*83*~~|~~*85*~~||~~*100*~~|~~*100*~~|~~*85*~~||~~*100*~~|76|81||~~*100*~~|~~*96*~~|81|
|840|~~*100*~~|~~*82*~~|~~*85*~~||~~*100*~~|~~*100*~~|~~*85*~~||~~*100*~~|76|81||~~*100*~~|~~*96*~~|81|
|850|~~*100*~~|~~*82*~~|~~*85*~~||~~*100*~~|~~*100*~~|~~*85*~~||~~*100*~~|75|81||~~*100*~~|~~*95*~~|81|
|860|~~*100*~~|~~*82*~~|~~*84*~~||~~*100*~~|~~*100*~~|~~*84*~~||~~*100*~~|75|80||~~*100*~~|~~*95*~~|80|
|880|~~*100*~~|80|~~*84*~~||~~*100*~~|~~*100*~~|~~*84*~~||~~*100*~~|74|80||~~*100*~~|~~*94*~~|80|
|910|~~*100*~~|80|~~*84*~~||~~*100*~~|~~*100*~~|~~*84*~~||~~*100*~~|73|79||~~*100*~~|~~*92*~~|79|
|920|~~*100*~~|79|~~*83*~~||~~*100*~~|~~*100*~~|~~*83*~~||~~*100*~~|73|79||~~*100*~~|~~*92*~~|79|
|940|~~*100*~~|79|~~*83*~~||~~*100*~~|~~*100*~~|~~*83*~~||~~*100*~~|72|79||~~*100*~~|~~*91*~~|79|
|950|~~*100*~~|78|~~*83*~~||~~*100*~~|~~*99*~~|~~*83*~~||~~*100*~~|72|78||~~*100*~~|~~*91*~~|78|
|970|~~*100*~~|78|~~*82*~~||~~*100*~~|~~*99*~~|~~*82*~~||~~*100*~~|71|78||~~*100*~~|~~*90*~~|78|
|990|~~*100*~~|77|~~*82*~~||~~*100*~~|~~*98*~~|~~*82*~~||~~*100*~~|71|78||~~*100*~~|~~*90*~~|78|
|1000|~~*100*~~|77|~~*82*~~||~~*100*~~|~~*98*~~|~~*82*~~||~~*100*~~|70|77||~~*100*~~|~~*88*~~|77|
|1030|~~*100*~~|76|81||~~*100*~~|~~*96*~~|81||~~*100*~~|69|77||~~*100*~~|~~*87*~~|77|
|1040|~~*100*~~|76|81||~~*100*~~|~~*96*~~|81||~~*100*~~|69|76||~~*100*~~|~~*87*~~|76|
|1060|~~*100*~~|75|81||~~*100*~~|~~*95*~~|81||~~*100*~~|68|76||~~*100*~~|~~*86*~~|76|
|1080|~~*100*~~|75|80||~~*100*~~|~~*95*~~|80||~~*100*~~|68|75||~~*100*~~|~~*86*~~|75|
|1090|~~*100*~~|75|80||~~*100*~~|~~*95*~~|80||~~*100*~~|67|75||~~*100*~~|~~*84*~~|75|
|1100|~~*100*~~|74|80||~~*100*~~|~~*94*~~|80||~~*100*~~|67|75||~~*100*~~|~~*84*~~|75|
|1120|~~*100*~~|74|80||~~*100*~~|~~*94*~~|80||~~*100*~~|66|74||~~*100*~~|~~*83*~~|74|
|1130|~~*100*~~|74|79||~~*100*~~|~~*94*~~|79||~~*100*~~|66|74||~~*100*~~|~~*83*~~|74|
|1140|~~*100*~~|73|79||~~*100*~~|~~*92*~~|79||~~*100*~~|65|74||~~*100*~~|~~*82*~~|74|

### [Water God Dungeon](https://wiki.melvoridle.com/index.php?title=Water_God_Dungeon), Auto Eat II

|❤️|⚔️|🏹|🧙||⚔️ ☠️|🏹 ☠️|🧙 ☠️||⚔️ 💍|🏹 💍|🧙 💍||⚔️ ☠️ 💍|🏹 ☠️ 💍|🧙 ☠️ 💍|
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|500|~~*100*~~|~~*85*~~|~~*87*~~||~~*100*~~|~~*100*~~|~~*87*~~||~~*100*~~|80|~~*84*~~||~~*100*~~|~~*100*~~|~~*84*~~|
|510|~~*100*~~|~~*84*~~|~~*86*~~||~~*100*~~|~~*100*~~|~~*86*~~||~~*100*~~|80|~~*84*~~||~~*100*~~|~~*100*~~|~~*84*~~|
|530|~~*100*~~|~~*84*~~|~~*86*~~||~~*100*~~|~~*100*~~|~~*86*~~||~~*100*~~|79|~~*83*~~||~~*100*~~|~~*100*~~|~~*83*~~|
|540|~~*100*~~|~~*83*~~|~~*85*~~||~~*100*~~|~~*100*~~|~~*85*~~||~~*100*~~|79|~~*83*~~||~~*100*~~|~~*100*~~|~~*83*~~|
|550|~~*100*~~|~~*83*~~|~~*85*~~||~~*100*~~|~~*100*~~|~~*85*~~||~~*100*~~|78|~~*83*~~||~~*100*~~|~~*99*~~|~~*83*~~|
|560|~~*100*~~|~~*82*~~|~~*85*~~||~~*100*~~|~~*100*~~|~~*85*~~||~~*100*~~|78|~~*82*~~||~~*100*~~|~~*99*~~|~~*82*~~|
|570|~~*100*~~|~~*82*~~|~~*85*~~||~~*100*~~|~~*100*~~|~~*85*~~||~~*100*~~|77|~~*82*~~||~~*100*~~|~~*98*~~|~~*82*~~|
|580|~~*100*~~|~~*82*~~|~~*84*~~||~~*100*~~|~~*100*~~|~~*84*~~||~~*100*~~|77|~~*82*~~||~~*100*~~|~~*98*~~|~~*82*~~|
|590|~~*100*~~|80|~~*84*~~||~~*100*~~|~~*100*~~|~~*84*~~||~~*100*~~|76|81||~~*100*~~|~~*96*~~|81|
|610|~~*100*~~|79|~~*83*~~||~~*100*~~|~~*100*~~|~~*83*~~||~~*100*~~|75|81||~~*100*~~|~~*95*~~|81|
|620|~~*100*~~|79|~~*83*~~||~~*100*~~|~~*100*~~|~~*83*~~||~~*100*~~|75|80||~~*100*~~|~~*95*~~|80|
|630|~~*100*~~|79|~~*83*~~||~~*100*~~|~~*100*~~|~~*83*~~||~~*100*~~|74|80||~~*100*~~|~~*94*~~|80|
|640|~~*100*~~|78|~~*83*~~||~~*100*~~|~~*99*~~|~~*83*~~||~~*100*~~|74|80||~~*100*~~|~~*94*~~|80|
|650|~~*100*~~|78|~~*82*~~||~~*100*~~|~~*99*~~|~~*82*~~||~~*100*~~|73|79||~~*100*~~|~~*92*~~|79|
|660|~~*100*~~|77|~~*82*~~||~~*100*~~|~~*98*~~|~~*82*~~||~~*100*~~|73|79||~~*100*~~|~~*92*~~|79|
|670|~~*100*~~|77|~~*82*~~||~~*100*~~|~~*98*~~|~~*82*~~||~~*100*~~|72|79||~~*100*~~|~~*91*~~|79|
|680|~~*100*~~|77|~~*82*~~||~~*100*~~|~~*98*~~|~~*82*~~||~~*100*~~|72|78||~~*100*~~|~~*91*~~|78|
|690|~~*100*~~|76|81||~~*100*~~|~~*96*~~|81||~~*100*~~|71|78||~~*100*~~|~~*90*~~|78|
|710|~~*100*~~|75|81||~~*100*~~|~~*95*~~|81||~~*100*~~|71|77||~~*100*~~|~~*90*~~|77|
|720|~~*100*~~|75|80||~~*100*~~|~~*95*~~|80||~~*100*~~|70|77||~~*100*~~|~~*88*~~|77|
|730|~~*100*~~|74|80||~~*100*~~|~~*94*~~|80||~~*100*~~|70|77||~~*100*~~|~~*88*~~|77|
|740|~~*100*~~|74|80||~~*100*~~|~~*94*~~|80||~~*100*~~|69|76||~~*100*~~|~~*87*~~|76|
|760|~~*100*~~|73|79||~~*100*~~|~~*92*~~|79||~~*100*~~|68|76||~~*100*~~|~~*86*~~|76|
|780|~~*100*~~|72|79||~~*100*~~|~~*91*~~|79||~~*100*~~|67|75||~~*100*~~|~~*84*~~|75|
|790|~~*100*~~|72|78||~~*100*~~|~~*91*~~|78||~~*100*~~|67|75||~~*100*~~|~~*84*~~|75|
|800|~~*100*~~|72|78||~~*100*~~|~~*91*~~|78||~~*100*~~|66|74||~~*100*~~|~~*83*~~|74|
|810|~~*100*~~|71|78||~~*100*~~|~~*90*~~|78||~~*100*~~|66|74||~~*100*~~|~~*83*~~|74|
|820|~~*100*~~|71|78||~~*100*~~|~~*90*~~|78||~~*100*~~|65|74||~~*100*~~|~~*82*~~|74|
|830|~~*100*~~|70|77||~~*100*~~|~~*88*~~|77||~~*100*~~|65|74||~~*100*~~|~~*82*~~|74|
|840|~~*100*~~|70|77||~~*100*~~|~~*88*~~|77||~~*100*~~|64|73||~~*100*~~|80|73|
|860|~~*100*~~|69|77||~~*100*~~|~~*87*~~|77||~~*100*~~|63|73||~~*100*~~|79|73|
|870|~~*100*~~|69|76||~~*100*~~|~~*87*~~|76||~~*100*~~|63|72||~~*100*~~|79|72|
|880|~~*100*~~|68|76||~~*100*~~|~~*86*~~|76||~~*100*~~|62|72||~~*100*~~|78|72|
|900|~~*100*~~|68|75||~~*100*~~|~~*86*~~|75||~~*100*~~|60|71||~~*100*~~|76|71|
|910|~~*100*~~|67|75||~~*100*~~|~~*84*~~|75||~~*100*~~|60|71||~~*100*~~|76|71|
|920|~~*100*~~|67|75||~~*100*~~|~~*84*~~|75||~~*100*~~|59|71||~~*100*~~|75|71|
|930|~~*100*~~|66|75||~~*100*~~|~~*83*~~|75||~~*100*~~|59|70||~~*100*~~|75|70|
|940|~~*100*~~|66|74||~~*100*~~|~~*83*~~|74||~~*100*~~|58|70||~~*100*~~|74|70|
|950|~~*100*~~|65|74||~~*100*~~|~~*82*~~|74||~~*100*~~|58|70||~~*100*~~|74|70|
|960|~~*100*~~|65|74||~~*100*~~|~~*82*~~|74||~~*100*~~|58|69||~~*100*~~|74|69|
|970|~~*100*~~|65|73||~~*100*~~|~~*82*~~|73||~~*100*~~|57|69||~~*100*~~|72|69|
|980|~~*100*~~|64|73||~~*100*~~|80|73||~~*100*~~|57|69||~~*100*~~|72|69|
|990|~~*100*~~|64|73||~~*100*~~|80|73||~~*100*~~|56|68||~~*100*~~|71|68|
|1000|~~*100*~~|63|73||~~*100*~~|79|73||~~*100*~~|56|68||~~*100*~~|71|68|
|1010|~~*100*~~|63|72||~~*100*~~|79|72||~~*100*~~|55|68||~~*100*~~|70|68|
|1020|~~*100*~~|63|72||~~*100*~~|79|72||~~*100*~~|55|67||~~*100*~~|70|67|
|1030|~~*100*~~|62|72||~~*100*~~|78|72||~~*100*~~|54|67||~~*100*~~|68|67|
|1050|~~*100*~~|60|71||~~*100*~~|76|71||~~*100*~~|53|66||~~*100*~~|67|66|
|1070|~~*100*~~|60|71||~~*100*~~|76|71||~~*100*~~|53|66||~~*100*~~|66|66|
|1080|~~*100*~~|59|70||~~*100*~~|75|70||~~*100*~~|52|65||~~*100*~~|66|65|
|1090|~~*100*~~|59|70||~~*100*~~|75|70||~~*100*~~|52|65||~~*100*~~|64|65|
|1100|~~*100*~~|58|70||~~*100*~~|74|70||~~*100*~~|52|65||~~*100*~~|64|65|
|1110|~~*100*~~|58|70||~~*100*~~|74|70||~~*100*~~|52|64||~~*100*~~|63|64|
|1120|~~*100*~~|58|69||~~*100*~~|74|69||~~*100*~~|52|64||~~*100*~~|63|64|
|1130|~~*100*~~|57|69||~~*100*~~|72|69||~~*100*~~|52|64||~~*100*~~|62|64|
|1140|~~*100*~~|57|69||~~*100*~~|72|69||~~*100*~~|51|63||~~*100*~~|62|63|
|1150|~~*100*~~|56|68||~~*100*~~|71|68||~~*100*~~|51|63||~~*100*~~|62|63|
|1160|~~*100*~~|56|68||~~*100*~~|71|68||~~*100*~~|51|63||~~*100*~~|60|63|

### [Water God Dungeon](https://wiki.melvoridle.com/index.php?title=Water_God_Dungeon), Auto Eat III

|❤️|⚔️|🏹|🧙||⚔️ ☠️|🏹 ☠️|🧙 ☠️||⚔️ 💍|🏹 💍|🧙 💍||⚔️ ☠️ 💍|🏹 ☠️ 💍|🧙 ☠️ 💍|
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|390|~~*100*~~|~~*84*~~|~~*86*~~||~~*100*~~|~~*100*~~|~~*86*~~||~~*100*~~|80|~~*84*~~||~~*100*~~|~~*100*~~|~~*84*~~|
|410|~~*100*~~|~~*83*~~|~~*85*~~||~~*100*~~|~~*100*~~|~~*85*~~||~~*100*~~|79|~~*83*~~||~~*100*~~|~~*100*~~|~~*83*~~|
|420|~~*100*~~|~~*82*~~|~~*85*~~||~~*100*~~|~~*100*~~|~~*85*~~||~~*100*~~|79|~~*83*~~||~~*100*~~|~~*100*~~|~~*83*~~|
|430|~~*100*~~|~~*82*~~|~~*84*~~||~~*100*~~|~~*100*~~|~~*84*~~||~~*100*~~|78|~~*83*~~||~~*100*~~|~~*99*~~|~~*83*~~|
|440|~~*100*~~|80|~~*84*~~||~~*100*~~|~~*100*~~|~~*84*~~||~~*100*~~|77|~~*82*~~||~~*100*~~|~~*98*~~|~~*82*~~|
|460|~~*100*~~|79|~~*83*~~||~~*100*~~|~~*100*~~|~~*83*~~||~~*100*~~|76|81||~~*100*~~|~~*96*~~|81|
|480|~~*100*~~|78|~~*83*~~||~~*100*~~|~~*99*~~|~~*83*~~||~~*100*~~|75|80||~~*100*~~|~~*95*~~|80|
|490|~~*100*~~|78|~~*82*~~||~~*100*~~|~~*99*~~|~~*82*~~||~~*100*~~|74|80||~~*100*~~|~~*94*~~|80|
|500|~~*100*~~|77|~~*82*~~||~~*100*~~|~~*98*~~|~~*82*~~||~~*100*~~|74|80||~~*100*~~|~~*94*~~|80|
|510|~~*100*~~|77|~~*82*~~||~~*100*~~|~~*98*~~|~~*82*~~||~~*100*~~|73|79||~~*100*~~|~~*92*~~|79|
|520|~~*100*~~|76|81||~~*100*~~|~~*96*~~|81||~~*100*~~|72|79||~~*100*~~|~~*91*~~|79|
|530|~~*100*~~|75|81||~~*100*~~|~~*95*~~|81||~~*100*~~|72|78||~~*100*~~|~~*91*~~|78|
|540|~~*100*~~|75|80||~~*100*~~|~~*95*~~|80||~~*100*~~|71|78||~~*100*~~|~~*90*~~|78|
|550|~~*100*~~|74|80||~~*100*~~|~~*94*~~|80||~~*100*~~|71|78||~~*100*~~|~~*90*~~|78|
|560|~~*100*~~|74|80||~~*100*~~|~~*94*~~|80||~~*100*~~|70|77||~~*100*~~|~~*88*~~|77|
|570|~~*100*~~|73|79||~~*100*~~|~~*92*~~|79||~~*100*~~|69|77||~~*100*~~|~~*87*~~|77|
|580|~~*100*~~|73|79||~~*100*~~|~~*92*~~|79||~~*100*~~|69|76||~~*100*~~|~~*87*~~|76|
|590|~~*100*~~|72|79||~~*100*~~|~~*91*~~|79||~~*100*~~|68|76||~~*100*~~|~~*86*~~|76|
|600|~~*100*~~|72|78||~~*100*~~|~~*91*~~|78||~~*100*~~|68|75||~~*100*~~|~~*86*~~|75|
|610|~~*100*~~|71|78||~~*100*~~|~~*90*~~|78||~~*100*~~|67|75||~~*100*~~|~~*84*~~|75|
|620|~~*100*~~|71|77||~~*100*~~|~~*90*~~|77||~~*100*~~|66|75||~~*100*~~|~~*83*~~|75|
|630|~~*100*~~|70|77||~~*100*~~|~~*88*~~|77||~~*100*~~|66|74||~~*100*~~|~~*83*~~|74|
|640|~~*100*~~|69|77||~~*100*~~|~~*87*~~|77||~~*100*~~|65|74||~~*100*~~|~~*82*~~|74|
|650|~~*100*~~|69|76||~~*100*~~|~~*87*~~|76||~~*100*~~|65|73||~~*100*~~|~~*82*~~|73|
|660|~~*100*~~|68|76||~~*100*~~|~~*86*~~|76||~~*100*~~|64|73||~~*100*~~|80|73|
|670|~~*100*~~|68|76||~~*100*~~|~~*86*~~|76||~~*100*~~|63|73||~~*100*~~|79|73|
|680|~~*100*~~|67|75||~~*100*~~|~~*84*~~|75||~~*100*~~|63|72||~~*100*~~|79|72|
|690|~~*100*~~|67|75||~~*100*~~|~~*84*~~|75||~~*100*~~|62|72||~~*100*~~|78|72|
|700|~~*100*~~|66|74||~~*100*~~|~~*83*~~|74||~~*100*~~|60|71||~~*100*~~|76|71|
|720|~~*100*~~|65|74||~~*100*~~|~~*82*~~|74||~~*100*~~|59|70||~~*100*~~|75|70|
|730|~~*100*~~|65|73||~~*100*~~|~~*82*~~|73||~~*100*~~|59|70||~~*100*~~|75|70|
|740|~~*100*~~|64|73||~~*100*~~|80|73||~~*100*~~|58|70||~~*100*~~|74|70|
|750|~~*100*~~|63|73||~~*100*~~|79|73||~~*100*~~|57|69||~~*100*~~|72|69|
|760|~~*100*~~|63|72||~~*100*~~|79|72||~~*100*~~|57|69||~~*100*~~|72|69|
|770|~~*100*~~|62|72||~~*100*~~|78|72||~~*100*~~|56|68||~~*100*~~|71|68|
|780|~~*100*~~|62|72||~~*100*~~|78|72||~~*100*~~|55|68||~~*100*~~|70|68|
|790|~~*100*~~|60|71||~~*100*~~|76|71||~~*100*~~|55|68||~~*100*~~|70|68|
|800|~~*100*~~|60|71||~~*100*~~|76|71||~~*100*~~|54|67||~~*100*~~|68|67|
|810|~~*100*~~|59|70||~~*100*~~|75|70||~~*100*~~|54|67||~~*100*~~|68|67|
|820|~~*100*~~|59|70||~~*100*~~|75|70||~~*100*~~|53|66||~~*100*~~|67|66|
|830|~~*100*~~|58|70||~~*100*~~|74|70||~~*100*~~|53|66||~~*100*~~|66|66|
|840|~~*100*~~|58|69||~~*100*~~|74|69||~~*100*~~|52|65||~~*100*~~|66|65|
|850|~~*100*~~|57|69||~~*100*~~|72|69||~~*100*~~|52|65||~~*100*~~|64|65|
|860|~~*100*~~|56|69||~~*100*~~|71|69||~~*100*~~|52|65||~~*100*~~|64|65|
|870|~~*100*~~|56|68||~~*100*~~|71|68||~~*100*~~|52|64||~~*100*~~|63|64|
|880|~~*100*~~|55|68||~~*100*~~|70|68||~~*100*~~|52|64||~~*100*~~|62|64|
|890|~~*100*~~|55|67||~~*100*~~|70|67||~~*100*~~|51|63||~~*100*~~|62|63|
|900|~~*100*~~|54|67||~~*100*~~|68|67||~~*100*~~|51|63||~~*100*~~|60|63|
|920|~~*100*~~|53|66||~~*100*~~|67|66||~~*100*~~|50|62||~~*100*~~|59|62|
|930|~~*100*~~|53|66||~~*100*~~|67|66||~~*100*~~|50|62||~~*100*~~|58|62|
|940|~~*100*~~|53|66||~~*100*~~|66|66||~~*100*~~|49|61||~~*100*~~|58|61|
|950|~~*100*~~|52|65||~~*100*~~|66|65||~~*100*~~|49|61||~~*100*~~|56|61|
|960|~~*100*~~|52|65||~~*100*~~|64|65||~~*100*~~|48|60||~~*100*~~|55|60|
|970|~~*100*~~|52|64||~~*100*~~|63|64||~~*100*~~|48|60||~~*100*~~|55|60|
|980|~~*100*~~|52|64||~~*100*~~|63|64||~~*100*~~|48|60||~~*100*~~|54|60|
|990|~~*100*~~|52|64||~~*100*~~|62|64||~~*100*~~|48|59||~~*100*~~|54|59|
|1000|~~*100*~~|51|63||~~*100*~~|62|63||~~*100*~~|48|59||~~*100*~~|52|59|
|1010|~~*100*~~|51|63||~~*100*~~|60|63||~~*100*~~|47|58||~~*100*~~|51|58|
|1030|~~*100*~~|50|62||~~*100*~~|59|62||~~*100*~~|47|58||~~*100*~~|50|58|
|1040|~~*100*~~|50|62||~~*100*~~|59|62||~~*100*~~|46|57||~~*100*~~|50|57|
|1050|~~*100*~~|50|62||~~*100*~~|58|62||~~*100*~~|46|57||~~*100*~~|48|57|
|1060|~~*100*~~|49|61||~~*100*~~|58|61||~~*100*~~|45|56||~~*100*~~|47|56|
|1070|~~*100*~~|49|61||~~*100*~~|56|61||~~*100*~~|45|56||~~*100*~~|47|56|
|1080|~~*100*~~|48|60||~~*100*~~|55|60||~~*100*~~|44|55||~~*100*~~|46|55|
|1100|~~*100*~~|48|60||~~*100*~~|54|60||~~*100*~~|44|55||~~*100*~~|44|55|
|1110|~~*100*~~|48|59||~~*100*~~|54|59||~~*100*~~|44|54||~~*100*~~|44|54|
|1120|~~*100*~~|48|59||~~*100*~~|52|59||~~*100*~~|44|54||~~*100*~~|44|54|
|1130|~~*100*~~|48|59||~~*100*~~|52|59||~~*100*~~|43|53||~~*100*~~|43|53|
|1140|~~*100*~~|47|58||~~*100*~~|51|58||~~*100*~~|43|53||~~*100*~~|43|53|
|1160|~~*100*~~|46|57||~~*100*~~|50|57||~~*100*~~|42|52||~~*100*~~|42|52|

### [Earth God Dungeon](https://wiki.melvoridle.com/index.php?title=Earth_God_Dungeon), Auto Eat I

|❤️|⚔️|🏹|🧙||⚔️ ☠️|🏹 ☠️|🧙 ☠️||⚔️ 💍|🏹 💍|🧙 💍||⚔️ ☠️ 💍|🏹 ☠️ 💍|🧙 ☠️ 💍|
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|520|~~*100*~~|~~*94*~~|~~*85*~~||~~*100*~~|~~*100*~~|~~*91*~~||~~*100*~~|~~*92*~~|81||~~*100*~~|~~*100*~~|81|
|530|~~*100*~~|~~*94*~~|~~*85*~~||~~*100*~~|~~*100*~~|~~*90*~~||~~*100*~~|~~*92*~~|81||~~*100*~~|~~*100*~~|81|
|540|~~*100*~~|~~*94*~~|~~*84*~~||~~*100*~~|~~*100*~~|~~*88*~~||~~*100*~~|~~*91*~~|80||~~*100*~~|~~*100*~~|80|
|560|~~*100*~~|~~*94*~~|~~*84*~~||~~*100*~~|~~*100*~~|~~*87*~~||~~*100*~~|~~*91*~~|80||~~*100*~~|~~*100*~~|80|
|570|~~*100*~~|~~*93*~~|~~*84*~~||~~*100*~~|~~*100*~~|~~*87*~~||~~*100*~~|~~*90*~~|79||~~*100*~~|~~*100*~~|79|
|580|~~*100*~~|~~*93*~~|~~*83*~~||~~*100*~~|~~*100*~~|~~*86*~~||~~*100*~~|~~*90*~~|79||~~*100*~~|~~*100*~~|79|
|590|~~*100*~~|~~*93*~~|~~*83*~~||~~*100*~~|~~*100*~~|~~*84*~~||~~*100*~~|~~*90*~~|79||~~*100*~~|~~*100*~~|79|
|600|~~*100*~~|~~*93*~~|~~*83*~~||~~*100*~~|~~*100*~~|~~*84*~~||~~*100*~~|~~*90*~~|78||~~*100*~~|~~*100*~~|78|
|610|~~*100*~~|~~*93*~~|~~*82*~~||~~*100*~~|~~*100*~~|~~*83*~~||~~*100*~~|~~*89*~~|78||~~*100*~~|~~*100*~~|78|
|620|~~*100*~~|~~*92*~~|~~*82*~~||~~*100*~~|~~*100*~~|~~*82*~~||~~*100*~~|~~*89*~~|78||~~*100*~~|~~*100*~~|78|
|630|~~*100*~~|~~*92*~~|~~*82*~~||~~*100*~~|~~*100*~~|~~*82*~~||~~*100*~~|~~*89*~~|77||~~*100*~~|~~*100*~~|77|
|650|~~*100*~~|~~*92*~~|81||~~*100*~~|~~*100*~~|81||~~*100*~~|~~*88*~~|77||~~*100*~~|~~*100*~~|77|
|660|~~*100*~~|~~*92*~~|81||~~*100*~~|~~*100*~~|81||~~*100*~~|~~*88*~~|76||~~*100*~~|~~*100*~~|76|
|670|~~*100*~~|~~*91*~~|81||~~*100*~~|~~*100*~~|81||~~*100*~~|~~*88*~~|76||~~*100*~~|~~*100*~~|76|
|680|~~*100*~~|~~*91*~~|80||~~*100*~~|~~*100*~~|80||~~*100*~~|~~*88*~~|75||~~*100*~~|~~*100*~~|75|
|690|~~*100*~~|~~*91*~~|80||~~*100*~~|~~*100*~~|80||~~*100*~~|~~*87*~~|75||~~*100*~~|~~*100*~~|75|
|710|~~*100*~~|~~*90*~~|79||~~*100*~~|~~*100*~~|79||~~*100*~~|~~*87*~~|74||~~*100*~~|~~*100*~~|74|
|720|~~*100*~~|~~*90*~~|79||~~*100*~~|~~*100*~~|79||~~*100*~~|~~*86*~~|74||~~*100*~~|~~*100*~~|74|
|740|~~*100*~~|~~*90*~~|79||~~*100*~~|~~*100*~~|79||~~*100*~~|~~*86*~~|73||~~*100*~~|~~*100*~~|73|
|750|~~*100*~~|~~*90*~~|78||~~*100*~~|~~*100*~~|78||~~*100*~~|~~*86*~~|73||~~*100*~~|~~*100*~~|73|
|760|~~*100*~~|~~*89*~~|78||~~*100*~~|~~*100*~~|78||~~*100*~~|~~*85*~~|72||~~*100*~~|~~*100*~~|72|
|780|~~*100*~~|~~*89*~~|77||~~*100*~~|~~*100*~~|77||~~*100*~~|~~*85*~~|72||~~*100*~~|~~*100*~~|72|
|790|~~*100*~~|~~*89*~~|77||~~*100*~~|~~*100*~~|77||~~*100*~~|~~*85*~~|71||~~*100*~~|~~*100*~~|71|
|800|~~*100*~~|~~*89*~~|77||~~*100*~~|~~*100*~~|77||~~*100*~~|~~*84*~~|71||~~*100*~~|~~*100*~~|71|
|810|~~*100*~~|~~*88*~~|77||~~*100*~~|~~*100*~~|77||~~*100*~~|~~*84*~~|71||~~*100*~~|~~*100*~~|71|
|820|~~*100*~~|~~*88*~~|76||~~*100*~~|~~*100*~~|76||~~*100*~~|~~*84*~~|70||~~*100*~~|~~*100*~~|70|
|840|~~*100*~~|~~*88*~~|76||~~*100*~~|~~*100*~~|76||~~*100*~~|~~*83*~~|69||~~*100*~~|~~*100*~~|69|
|850|~~*100*~~|~~*88*~~|75||~~*100*~~|~~*100*~~|75||~~*100*~~|~~*83*~~|69||~~*100*~~|~~*100*~~|69|
|860|~~*100*~~|~~*87*~~|75||~~*100*~~|~~*100*~~|75||~~*100*~~|~~*83*~~|69||~~*100*~~|~~*100*~~|69|
|870|~~*100*~~|~~*87*~~|75||~~*100*~~|~~*100*~~|75||~~*100*~~|~~*83*~~|68||~~*100*~~|~~*100*~~|68|
|880|~~*100*~~|~~*87*~~|74||~~*100*~~|~~*100*~~|74||~~*100*~~|~~*82*~~|68||~~*100*~~|~~*100*~~|68|
|900|~~*100*~~|~~*86*~~|74||~~*100*~~|~~*100*~~|74||~~*100*~~|~~*82*~~|67||~~*100*~~|~~*100*~~|67|
|920|~~*100*~~|~~*86*~~|73||~~*100*~~|~~*100*~~|73||~~*100*~~|80|67||~~*100*~~|~~*100*~~|67|
|930|~~*100*~~|~~*86*~~|73||~~*100*~~|~~*100*~~|73||~~*100*~~|80|66||~~*100*~~|~~*100*~~|66|
|950|~~*100*~~|~~*85*~~|72||~~*100*~~|~~*100*~~|72||~~*100*~~|79|65||~~*100*~~|~~*100*~~|65|
|980|~~*100*~~|~~*85*~~|72||~~*100*~~|~~*100*~~|72||~~*100*~~|79|64||~~*100*~~|~~*100*~~|64|
|990|~~*100*~~|~~*85*~~|71||~~*100*~~|~~*100*~~|71||~~*100*~~|78|64||~~*100*~~|~~*99*~~|64|
|1000|~~*100*~~|~~*84*~~|71||~~*100*~~|~~*100*~~|71||~~*100*~~|78|64||~~*100*~~|~~*99*~~|64|
|1010|~~*100*~~|~~*84*~~|71||~~*100*~~|~~*100*~~|71||~~*100*~~|78|63||~~*100*~~|~~*99*~~|63|
|1020|~~*100*~~|~~*84*~~|70||~~*100*~~|~~*100*~~|70||~~*100*~~|78|63||~~*100*~~|~~*99*~~|63|
|1030|~~*100*~~|~~*84*~~|70||~~*100*~~|~~*100*~~|70||~~*100*~~|77|63||~~*100*~~|~~*98*~~|63|
|1040|~~*100*~~|~~*83*~~|70||~~*100*~~|~~*100*~~|70||~~*100*~~|77|62||~~*100*~~|~~*98*~~|62|
|1050|~~*100*~~|~~*83*~~|69||~~*100*~~|~~*100*~~|69||~~*100*~~|77|62||~~*100*~~|~~*98*~~|62|
|1060|~~*100*~~|~~*83*~~|69||~~*100*~~|~~*100*~~|69||~~*100*~~|77|61||~~*100*~~|~~*98*~~|61|
|1070|~~*100*~~|~~*83*~~|69||~~*100*~~|~~*100*~~|69||~~*100*~~|76|61||~~*100*~~|~~*96*~~|61|
|1090|~~*100*~~|~~*82*~~|68||~~*100*~~|~~*100*~~|68||~~*100*~~|76|60||~~*100*~~|~~*96*~~|60|
|1100|~~*100*~~|~~*82*~~|68||~~*100*~~|~~*100*~~|68||~~*100*~~|75|60||~~*100*~~|~~*95*~~|60|
|1120|~~*100*~~|~~*82*~~|67||~~*100*~~|~~*100*~~|67||~~*100*~~|75|59||~~*100*~~|~~*95*~~|59|
|1140|~~*100*~~|80|67||~~*100*~~|~~*100*~~|67||~~*100*~~|74|58||~~*100*~~|~~*94*~~|58|
|1160|~~*100*~~|80|66||~~*100*~~|~~*100*~~|66||~~*100*~~|74|58||~~*100*~~|~~*94*~~|58|

### [Earth God Dungeon](https://wiki.melvoridle.com/index.php?title=Earth_God_Dungeon), Auto Eat II

|❤️|⚔️|🏹|🧙||⚔️ ☠️|🏹 ☠️|🧙 ☠️||⚔️ 💍|🏹 💍|🧙 💍||⚔️ ☠️ 💍|🏹 ☠️ 💍|🧙 ☠️ 💍|
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|370|~~*100*~~|~~*94*~~|~~*84*~~||~~*100*~~|~~*100*~~|~~*88*~~||~~*100*~~|~~*92*~~|81||~~*100*~~|~~*100*~~|81|
|380|~~*100*~~|~~*93*~~|~~*84*~~||~~*100*~~|~~*100*~~|~~*87*~~||~~*100*~~|~~*91*~~|81||~~*100*~~|~~*100*~~|81|
|390|~~*100*~~|~~*93*~~|~~*83*~~||~~*100*~~|~~*100*~~|~~*86*~~||~~*100*~~|~~*91*~~|80||~~*100*~~|~~*100*~~|80|
|400|~~*100*~~|~~*93*~~|~~*83*~~||~~*100*~~|~~*100*~~|~~*84*~~||~~*100*~~|~~*91*~~|80||~~*100*~~|~~*100*~~|80|
|410|~~*100*~~|~~*92*~~|~~*82*~~||~~*100*~~|~~*100*~~|~~*83*~~||~~*100*~~|~~*90*~~|79||~~*100*~~|~~*100*~~|79|
|420|~~*100*~~|~~*92*~~|~~*82*~~||~~*100*~~|~~*100*~~|~~*82*~~||~~*100*~~|~~*90*~~|79||~~*100*~~|~~*100*~~|79|
|430|~~*100*~~|~~*92*~~|81||~~*100*~~|~~*100*~~|81||~~*100*~~|~~*90*~~|78||~~*100*~~|~~*100*~~|78|
|440|~~*100*~~|~~*92*~~|81||~~*100*~~|~~*100*~~|81||~~*100*~~|~~*89*~~|78||~~*100*~~|~~*100*~~|78|
|450|~~*100*~~|~~*91*~~|80||~~*100*~~|~~*100*~~|80||~~*100*~~|~~*89*~~|77||~~*100*~~|~~*100*~~|77|
|460|~~*100*~~|~~*91*~~|80||~~*100*~~|~~*100*~~|80||~~*100*~~|~~*88*~~|77||~~*100*~~|~~*100*~~|77|
|470|~~*100*~~|~~*91*~~|80||~~*100*~~|~~*100*~~|80||~~*100*~~|~~*88*~~|76||~~*100*~~|~~*100*~~|76|
|480|~~*100*~~|~~*90*~~|79||~~*100*~~|~~*100*~~|79||~~*100*~~|~~*88*~~|76||~~*100*~~|~~*100*~~|76|
|490|~~*100*~~|~~*90*~~|79||~~*100*~~|~~*100*~~|79||~~*100*~~|~~*87*~~|75||~~*100*~~|~~*100*~~|75|
|500|~~*100*~~|~~*90*~~|78||~~*100*~~|~~*100*~~|78||~~*100*~~|~~*87*~~|75||~~*100*~~|~~*100*~~|75|
|510|~~*100*~~|~~*89*~~|78||~~*100*~~|~~*100*~~|78||~~*100*~~|~~*87*~~|74||~~*100*~~|~~*100*~~|74|
|520|~~*100*~~|~~*89*~~|77||~~*100*~~|~~*100*~~|77||~~*100*~~|~~*86*~~|74||~~*100*~~|~~*100*~~|74|
|530|~~*100*~~|~~*89*~~|77||~~*100*~~|~~*100*~~|77||~~*100*~~|~~*86*~~|73||~~*100*~~|~~*100*~~|73|
|540|~~*100*~~|~~*88*~~|77||~~*100*~~|~~*100*~~|77||~~*100*~~|~~*85*~~|73||~~*100*~~|~~*100*~~|73|
|550|~~*100*~~|~~*88*~~|76||~~*100*~~|~~*100*~~|76||~~*100*~~|~~*85*~~|72||~~*100*~~|~~*100*~~|72|
|570|~~*100*~~|~~*87*~~|75||~~*100*~~|~~*100*~~|75||~~*100*~~|~~*84*~~|71||~~*100*~~|~~*100*~~|71|
|590|~~*100*~~|~~*87*~~|74||~~*100*~~|~~*100*~~|74||~~*100*~~|~~*84*~~|70||~~*100*~~|~~*100*~~|70|
|600|~~*100*~~|~~*86*~~|74||~~*100*~~|~~*100*~~|74||~~*100*~~|~~*83*~~|69||~~*100*~~|~~*100*~~|69|
|610|~~*100*~~|~~*86*~~|73||~~*100*~~|~~*100*~~|73||~~*100*~~|~~*83*~~|69||~~*100*~~|~~*100*~~|69|
|620|~~*100*~~|~~*86*~~|73||~~*100*~~|~~*100*~~|73||~~*100*~~|~~*83*~~|68||~~*100*~~|~~*100*~~|68|
|630|~~*100*~~|~~*85*~~|73||~~*100*~~|~~*100*~~|73||~~*100*~~|~~*82*~~|68||~~*100*~~|~~*100*~~|68|
|640|~~*100*~~|~~*85*~~|72||~~*100*~~|~~*100*~~|72||~~*100*~~|~~*82*~~|67||~~*100*~~|~~*100*~~|67|
|660|~~*100*~~|~~*85*~~|71||~~*100*~~|~~*100*~~|71||~~*100*~~|80|67||~~*100*~~|~~*100*~~|67|
|670|~~*100*~~|~~*84*~~|71||~~*100*~~|~~*100*~~|71||~~*100*~~|80|66||~~*100*~~|~~*100*~~|66|
|680|~~*100*~~|~~*84*~~|70||~~*100*~~|~~*100*~~|70||~~*100*~~|79|65||~~*100*~~|~~*100*~~|65|
|700|~~*100*~~|~~*83*~~|69||~~*100*~~|~~*100*~~|69||~~*100*~~|79|64||~~*100*~~|~~*100*~~|64|
|710|~~*100*~~|~~*83*~~|69||~~*100*~~|~~*100*~~|69||~~*100*~~|78|64||~~*100*~~|~~*99*~~|64|
|720|~~*100*~~|~~*83*~~|69||~~*100*~~|~~*100*~~|69||~~*100*~~|78|63||~~*100*~~|~~*99*~~|63|
|730|~~*100*~~|~~*82*~~|68||~~*100*~~|~~*100*~~|68||~~*100*~~|78|63||~~*100*~~|~~*99*~~|63|
|740|~~*100*~~|~~*82*~~|68||~~*100*~~|~~*100*~~|68||~~*100*~~|77|62||~~*100*~~|~~*98*~~|62|
|750|~~*100*~~|~~*82*~~|67||~~*100*~~|~~*100*~~|67||~~*100*~~|77|62||~~*100*~~|~~*98*~~|62|
|760|~~*100*~~|80|67||~~*100*~~|~~*100*~~|67||~~*100*~~|76|61||~~*100*~~|~~*96*~~|61|
|770|~~*100*~~|80|66||~~*100*~~|~~*100*~~|66||~~*100*~~|76|61||~~*100*~~|~~*96*~~|61|
|780|~~*100*~~|80|66||~~*100*~~|~~*100*~~|66||~~*100*~~|76|60||~~*100*~~|~~*96*~~|60|
|790|~~*100*~~|79|65||~~*100*~~|~~*100*~~|65||~~*100*~~|75|60||~~*100*~~|~~*95*~~|60|
|800|~~*100*~~|79|65||~~*100*~~|~~*100*~~|65||~~*100*~~|75|59||~~*100*~~|~~*95*~~|59|
|820|~~*100*~~|79|64||~~*100*~~|~~*100*~~|64||~~*100*~~|74|58||~~*100*~~|~~*94*~~|58|
|830|~~*100*~~|78|64||~~*100*~~|~~*99*~~|64||~~*100*~~|74|58||~~*100*~~|~~*94*~~|58|
|840|~~*100*~~|78|63||~~*100*~~|~~*99*~~|63||~~*100*~~|73|57||~~*100*~~|~~*92*~~|57|
|860|~~*100*~~|77|62||~~*100*~~|~~*98*~~|62||~~*100*~~|73|56||~~*100*~~|~~*92*~~|56|
|870|~~*100*~~|77|62||~~*100*~~|~~*98*~~|62||~~*100*~~|72|56||~~*100*~~|~~*91*~~|56|
|880|~~*100*~~|77|62||~~*100*~~|~~*98*~~|62||~~*100*~~|72|55||~~*100*~~|~~*91*~~|55|
|890|~~*100*~~|76|61||~~*100*~~|~~*96*~~|61||~~*100*~~|72|55||~~*100*~~|~~*91*~~|55|
|900|~~*100*~~|76|61||~~*100*~~|~~*96*~~|61||~~*100*~~|71|54||~~*100*~~|~~*90*~~|54|
|910|~~*100*~~|76|60||~~*100*~~|~~*96*~~|60||~~*100*~~|71|54||~~*100*~~|~~*90*~~|54|
|920|~~*100*~~|75|60||~~*100*~~|~~*95*~~|60||~~*100*~~|70|53||~~*100*~~|~~*88*~~|53|
|930|~~*100*~~|75|59||~~*100*~~|~~*95*~~|59||~~*100*~~|70|53||~~*100*~~|~~*88*~~|53|
|950|~~*100*~~|74|58||~~*100*~~|~~*94*~~|58||~~*100*~~|69|52||~~*100*~~|~~*87*~~|52|
|980|~~*100*~~|73|57||~~*100*~~|~~*92*~~|57||~~*100*~~|68|52||~~*100*~~|~~*86*~~|52|
|990|~~*100*~~|73|57||~~*100*~~|~~*92*~~|57||~~*98*~~|68|52||~~*100*~~|~~*86*~~|52|
|1000|~~*100*~~|73|56||~~*100*~~|~~*92*~~|56||~~*98*~~|68|52||~~*100*~~|~~*86*~~|52|
|1010|~~*100*~~|72|56||~~*100*~~|~~*91*~~|56||~~*96*~~|67|51||~~*100*~~|~~*84*~~|51|
|1020|~~*100*~~|72|55||~~*100*~~|~~*91*~~|55||~~*96*~~|67|51||~~*100*~~|~~*84*~~|51|
|1030|~~*100*~~|72|55||~~*100*~~|~~*91*~~|55||~~*94*~~|67|51||~~*100*~~|~~*84*~~|51|
|1040|~~*100*~~|72|55||~~*100*~~|~~*91*~~|55||~~*94*~~|66|50||~~*100*~~|~~*83*~~|50|
|1050|~~*100*~~|71|54||~~*100*~~|~~*90*~~|54||~~*92*~~|66|50||~~*100*~~|~~*83*~~|50|
|1060|~~*100*~~|71|54||~~*100*~~|~~*90*~~|54||~~*92*~~|65|49||~~*100*~~|~~*82*~~|49|
|1070|~~*100*~~|71|54||~~*100*~~|~~*90*~~|54||~~*90*~~|65|49||~~*100*~~|~~*82*~~|49|
|1080|~~*100*~~|70|53||~~*100*~~|~~*88*~~|53||~~*90*~~|65|49||~~*100*~~|~~*82*~~|49|
|1090|~~*100*~~|70|53||~~*100*~~|~~*88*~~|53||~~*88*~~|64|48||~~*100*~~|80|48|
|1110|~~*100*~~|69|52||~~*100*~~|~~*87*~~|52||~~*86*~~|64|48||~~*100*~~|80|48|
|1120|~~*100*~~|69|52||~~*100*~~|~~*87*~~|52||~~*86*~~|63|48||~~*100*~~|79|48|
|1130|~~*100*~~|69|52||~~*100*~~|~~*87*~~|52||~~*84*~~|63|48||~~*100*~~|79|48|
|1140|~~*100*~~|68|52||~~*100*~~|~~*86*~~|52||~~*84*~~|62|47||~~*100*~~|78|47|
|1150|~~*100*~~|68|52||~~*100*~~|~~*86*~~|52||~~*82*~~|62|47||~~*100*~~|78|47|
|1160|~~*98*~~|68|52||~~*100*~~|~~*86*~~|52||~~*82*~~|62|47||~~*100*~~|78|47|

### [Earth God Dungeon](https://wiki.melvoridle.com/index.php?title=Earth_God_Dungeon), Auto Eat III

|❤️|⚔️|🏹|🧙||⚔️ ☠️|🏹 ☠️|🧙 ☠️||⚔️ 💍|🏹 💍|🧙 💍||⚔️ ☠️ 💍|🏹 ☠️ 💍|🧙 ☠️ 💍|
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|290|~~*100*~~|~~*93*~~|~~*83*~~||~~*100*~~|~~*100*~~|~~*86*~~||~~*100*~~|~~*92*~~|81||~~*100*~~|~~*100*~~|81|
|300|~~*100*~~|~~*93*~~|~~*83*~~||~~*100*~~|~~*100*~~|~~*84*~~||~~*100*~~|~~*91*~~|80||~~*100*~~|~~*100*~~|80|
|310|~~*100*~~|~~*92*~~|~~*82*~~||~~*100*~~|~~*100*~~|~~*82*~~||~~*100*~~|~~*91*~~|80||~~*100*~~|~~*100*~~|80|
|320|~~*100*~~|~~*92*~~|~~*82*~~||~~*100*~~|~~*100*~~|~~*82*~~||~~*100*~~|~~*90*~~|79||~~*100*~~|~~*100*~~|79|
|330|~~*100*~~|~~*92*~~|81||~~*100*~~|~~*100*~~|81||~~*100*~~|~~*90*~~|79||~~*100*~~|~~*100*~~|79|
|340|~~*100*~~|~~*91*~~|80||~~*100*~~|~~*100*~~|80||~~*100*~~|~~*89*~~|78||~~*100*~~|~~*100*~~|78|
|350|~~*100*~~|~~*91*~~|80||~~*100*~~|~~*100*~~|80||~~*100*~~|~~*89*~~|77||~~*100*~~|~~*100*~~|77|
|360|~~*100*~~|~~*90*~~|79||~~*100*~~|~~*100*~~|79||~~*100*~~|~~*88*~~|77||~~*100*~~|~~*100*~~|77|
|370|~~*100*~~|~~*90*~~|79||~~*100*~~|~~*100*~~|79||~~*100*~~|~~*88*~~|76||~~*100*~~|~~*100*~~|76|
|380|~~*100*~~|~~*89*~~|78||~~*100*~~|~~*100*~~|78||~~*100*~~|~~*87*~~|75||~~*100*~~|~~*100*~~|75|
|390|~~*100*~~|~~*89*~~|77||~~*100*~~|~~*100*~~|77||~~*100*~~|~~*87*~~|75||~~*100*~~|~~*100*~~|75|
|400|~~*100*~~|~~*89*~~|77||~~*100*~~|~~*100*~~|77||~~*100*~~|~~*86*~~|74||~~*100*~~|~~*100*~~|74|
|410|~~*100*~~|~~*88*~~|76||~~*100*~~|~~*100*~~|76||~~*100*~~|~~*86*~~|73||~~*100*~~|~~*100*~~|73|
|420|~~*100*~~|~~*88*~~|76||~~*100*~~|~~*100*~~|76||~~*100*~~|~~*85*~~|73||~~*100*~~|~~*100*~~|73|
|430|~~*100*~~|~~*87*~~|75||~~*100*~~|~~*100*~~|75||~~*100*~~|~~*85*~~|72||~~*100*~~|~~*100*~~|72|
|440|~~*100*~~|~~*87*~~|74||~~*100*~~|~~*100*~~|74||~~*100*~~|~~*85*~~|71||~~*100*~~|~~*100*~~|71|
|450|~~*100*~~|~~*86*~~|74||~~*100*~~|~~*100*~~|74||~~*100*~~|~~*84*~~|71||~~*100*~~|~~*100*~~|71|
|460|~~*100*~~|~~*86*~~|73||~~*100*~~|~~*100*~~|73||~~*100*~~|~~*84*~~|70||~~*100*~~|~~*100*~~|70|
|470|~~*100*~~|~~*86*~~|73||~~*100*~~|~~*100*~~|73||~~*100*~~|~~*83*~~|69||~~*100*~~|~~*100*~~|69|
|480|~~*100*~~|~~*85*~~|72||~~*100*~~|~~*100*~~|72||~~*100*~~|~~*83*~~|69||~~*100*~~|~~*100*~~|69|
|490|~~*100*~~|~~*85*~~|72||~~*100*~~|~~*100*~~|72||~~*100*~~|~~*82*~~|68||~~*100*~~|~~*100*~~|68|
|500|~~*100*~~|~~*84*~~|71||~~*100*~~|~~*100*~~|71||~~*100*~~|~~*82*~~|67||~~*100*~~|~~*100*~~|67|
|510|~~*100*~~|~~*84*~~|70||~~*100*~~|~~*100*~~|70||~~*100*~~|80|67||~~*100*~~|~~*100*~~|67|
|520|~~*100*~~|~~*83*~~|70||~~*100*~~|~~*100*~~|70||~~*100*~~|80|66||~~*100*~~|~~*100*~~|66|
|530|~~*100*~~|~~*83*~~|69||~~*100*~~|~~*100*~~|69||~~*100*~~|79|65||~~*100*~~|~~*100*~~|65|
|550|~~*100*~~|~~*82*~~|68||~~*100*~~|~~*100*~~|68||~~*100*~~|78|64||~~*100*~~|~~*99*~~|64|
|560|~~*100*~~|~~*82*~~|67||~~*100*~~|~~*100*~~|67||~~*100*~~|78|63||~~*100*~~|~~*99*~~|63|
|570|~~*100*~~|80|67||~~*100*~~|~~*100*~~|67||~~*100*~~|77|63||~~*100*~~|~~*98*~~|63|
|580|~~*100*~~|80|66||~~*100*~~|~~*100*~~|66||~~*100*~~|77|62||~~*100*~~|~~*98*~~|62|
|590|~~*100*~~|80|66||~~*100*~~|~~*100*~~|66||~~*100*~~|77|61||~~*100*~~|~~*98*~~|61|
|600|~~*100*~~|79|65||~~*100*~~|~~*100*~~|65||~~*100*~~|76|61||~~*100*~~|~~*96*~~|61|
|610|~~*100*~~|79|64||~~*100*~~|~~*100*~~|64||~~*100*~~|76|60||~~*100*~~|~~*96*~~|60|
|620|~~*100*~~|78|64||~~*100*~~|~~*99*~~|64||~~*100*~~|75|59||~~*100*~~|~~*95*~~|59|
|630|~~*100*~~|78|63||~~*100*~~|~~*99*~~|63||~~*100*~~|75|59||~~*100*~~|~~*95*~~|59|
|640|~~*100*~~|77|63||~~*100*~~|~~*98*~~|63||~~*100*~~|74|58||~~*100*~~|~~*94*~~|58|
|650|~~*100*~~|77|62||~~*100*~~|~~*98*~~|62||~~*100*~~|74|57||~~*100*~~|~~*94*~~|57|
|660|~~*100*~~|77|62||~~*100*~~|~~*98*~~|62||~~*100*~~|73|57||~~*100*~~|~~*92*~~|57|
|670|~~*100*~~|76|61||~~*100*~~|~~*96*~~|61||~~*100*~~|73|56||~~*100*~~|~~*92*~~|56|
|680|~~*100*~~|76|60||~~*100*~~|~~*96*~~|60||~~*100*~~|72|55||~~*100*~~|~~*91*~~|55|
|690|~~*100*~~|75|60||~~*100*~~|~~*95*~~|60||~~*100*~~|72|55||~~*100*~~|~~*91*~~|55|
|700|~~*100*~~|75|59||~~*100*~~|~~*95*~~|59||~~*100*~~|71|54||~~*100*~~|~~*90*~~|54|
|720|~~*100*~~|74|58||~~*100*~~|~~*94*~~|58||~~*100*~~|70|53||~~*100*~~|~~*88*~~|53|
|730|~~*100*~~|74|57||~~*100*~~|~~*94*~~|57||~~*100*~~|70|53||~~*100*~~|~~*88*~~|53|
|740|~~*100*~~|73|57||~~*100*~~|~~*92*~~|57||~~*100*~~|69|52||~~*100*~~|~~*87*~~|52|
|750|~~*100*~~|73|56||~~*100*~~|~~*92*~~|56||~~*100*~~|69|52||~~*100*~~|~~*87*~~|52|
|760|~~*100*~~|72|56||~~*100*~~|~~*91*~~|56||~~*100*~~|68|52||~~*100*~~|~~*86*~~|52|
|770|~~*100*~~|72|55||~~*100*~~|~~*91*~~|55||~~*98*~~|68|52||~~*100*~~|~~*86*~~|52|
|780|~~*100*~~|72|55||~~*100*~~|~~*91*~~|55||~~*98*~~|67|51||~~*100*~~|~~*84*~~|51|
|790|~~*100*~~|71|54||~~*100*~~|~~*90*~~|54||~~*96*~~|67|51||~~*100*~~|~~*84*~~|51|
|800|~~*100*~~|71|54||~~*100*~~|~~*90*~~|54||~~*94*~~|67|51||~~*100*~~|~~*84*~~|51|
|810|~~*100*~~|70|53||~~*100*~~|~~*88*~~|53||~~*94*~~|66|50||~~*100*~~|~~*83*~~|50|
|820|~~*100*~~|70|53||~~*100*~~|~~*88*~~|53||~~*92*~~|66|50||~~*100*~~|~~*83*~~|50|
|830|~~*100*~~|69|52||~~*100*~~|~~*87*~~|52||~~*92*~~|65|49||~~*100*~~|~~*82*~~|49|
|840|~~*100*~~|69|52||~~*100*~~|~~*87*~~|52||~~*90*~~|65|49||~~*100*~~|~~*82*~~|49|
|850|~~*100*~~|69|52||~~*100*~~|~~*87*~~|52||~~*88*~~|64|48||~~*100*~~|80|48|
|860|~~*100*~~|68|52||~~*100*~~|~~*86*~~|52||~~*86*~~|64|48||~~*100*~~|80|48|
|870|~~*98*~~|68|52||~~*100*~~|~~*86*~~|52||~~*86*~~|63|48||~~*100*~~|79|48|
|880|~~*98*~~|67|51||~~*100*~~|~~*84*~~|51||~~*84*~~|63|48||~~*100*~~|79|48|
|890|~~*96*~~|67|51||~~*100*~~|~~*84*~~|51||~~*84*~~|62|47||~~*100*~~|78|47|
|900|~~*94*~~|67|51||~~*100*~~|~~*84*~~|51||~~*82*~~|62|47||~~*100*~~|78|47|
|910|~~*94*~~|66|50||~~*100*~~|~~*83*~~|50||80|60|46||~~*100*~~|76|46|
|920|~~*92*~~|66|50||~~*100*~~|~~*83*~~|50||78|60|46||~~*100*~~|76|46|
|930|~~*92*~~|65|49||~~*100*~~|~~*82*~~|49||78|59|45||~~*100*~~|75|45|
|940|~~*90*~~|65|49||~~*100*~~|~~*82*~~|49||76|59|45||~~*100*~~|75|45|
|950|~~*88*~~|64|48||~~*100*~~|80|48||76|58|44||~~*100*~~|74|44|
|960|~~*88*~~|64|48||~~*100*~~|80|48||74|58|44||~~*100*~~|74|44|
|970|~~*86*~~|64|48||~~*100*~~|80|48||72|57|44||~~*100*~~|72|44|
|980|~~*86*~~|63|48||~~*100*~~|79|48||72|57|44||~~*100*~~|72|44|
|990|~~*84*~~|63|48||~~*100*~~|79|48||70|57|44||~~*100*~~|72|44|
|1000|~~*84*~~|62|47||~~*100*~~|78|47||68|56|43||~~*100*~~|71|43|
|1010|~~*82*~~|62|47||~~*100*~~|78|47||68|56|43||~~*100*~~|71|43|
|1020|80|60|46||~~*100*~~|76|46||66|55|42||~~*100*~~|70|42|
|1030|80|60|46||~~*100*~~|76|46||64|55|42||~~*100*~~|70|42|
|1040|78|60|46||~~*100*~~|76|46||64|54|41||~~*100*~~|68|41|
|1050|78|59|45||~~*100*~~|75|45||62|54|41||~~*100*~~|68|41|
|1060|76|59|45||~~*100*~~|75|45||60|53|40||~~*100*~~|67|40|
|1070|74|58|44||~~*100*~~|74|44||60|53|40||~~*100*~~|67|40|
|1080|74|58|44||~~*100*~~|74|44||58|52|40||~~*100*~~|66|40|
|1090|72|57|44||~~*100*~~|72|44||56|52|40||~~*100*~~|66|40|
|1100|72|57|44||~~*100*~~|72|44||56|51|39||~~*100*~~|64|39|
|1110|70|57|44||~~*100*~~|72|44||54|51|39||~~*100*~~|64|39|
|1120|68|56|43||~~*100*~~|71|43||52|50|38||~~*100*~~|63|38|
|1140|66|55|42||~~*100*~~|70|42||50|49|37||~~*100*~~|62|37|
|1150|66|55|42||~~*100*~~|70|42||48|49|37||~~*96*~~|62|37|
|1160|64|55|42||~~*100*~~|70|42||48|48|36||~~*96*~~|60|36|

### [Fire God Dungeon](https://wiki.melvoridle.com/index.php?title=Fire_God_Dungeon), Auto Eat I

|❤️|⚔️|🏹|🧙||⚔️ ☠️|🏹 ☠️|🧙 ☠️||⚔️ 💍|🏹 💍|🧙 💍||⚔️ ☠️ 💍|🏹 ☠️ 💍|🧙 ☠️ 💍|
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|870|~~*100*~~|~~*86*~~|~~*87*~~||~~*100*~~|~~*100*~~|~~*87*~~||~~*100*~~|80|~~*84*~~||~~*100*~~|~~*100*~~|~~*84*~~|
|890|~~*100*~~|~~*86*~~|~~*87*~~||~~*100*~~|~~*100*~~|~~*87*~~||~~*100*~~|80|~~*83*~~||~~*100*~~|~~*100*~~|~~*83*~~|
|900|~~*100*~~|~~*85*~~|~~*87*~~||~~*100*~~|~~*100*~~|~~*87*~~||~~*100*~~|79|~~*83*~~||~~*100*~~|~~*100*~~|~~*83*~~|
|910|~~*100*~~|~~*85*~~|~~*86*~~||~~*100*~~|~~*100*~~|~~*86*~~||~~*100*~~|79|~~*83*~~||~~*100*~~|~~*100*~~|~~*83*~~|
|940|~~*100*~~|~~*84*~~|~~*86*~~||~~*100*~~|~~*100*~~|~~*86*~~||~~*100*~~|78|~~*82*~~||~~*100*~~|~~*99*~~|~~*82*~~|
|980|~~*100*~~|~~*84*~~|~~*85*~~||~~*100*~~|~~*100*~~|~~*85*~~||~~*100*~~|77|~~*82*~~||~~*100*~~|~~*98*~~|~~*82*~~|
|990|~~*100*~~|~~*83*~~|~~*85*~~||~~*100*~~|~~*100*~~|~~*85*~~||~~*100*~~|77|81||~~*100*~~|~~*98*~~|81|
|1010|~~*100*~~|~~*83*~~|~~*85*~~||~~*100*~~|~~*100*~~|~~*85*~~||~~*100*~~|76|81||~~*100*~~|~~*96*~~|81|
|1030|~~*100*~~|~~*82*~~|~~*85*~~||~~*100*~~|~~*100*~~|~~*85*~~||~~*100*~~|76|81||~~*100*~~|~~*96*~~|81|
|1040|~~*100*~~|~~*82*~~|~~*84*~~||~~*100*~~|~~*100*~~|~~*84*~~||~~*100*~~|76|80||~~*100*~~|~~*96*~~|80|
|1050|~~*100*~~|~~*82*~~|~~*84*~~||~~*100*~~|~~*100*~~|~~*84*~~||~~*100*~~|75|80||~~*100*~~|~~*95*~~|80|
|1080|~~*100*~~|80|~~*84*~~||~~*100*~~|~~*100*~~|~~*84*~~||~~*100*~~|74|80||~~*100*~~|~~*94*~~|80|
|1090|~~*100*~~|80|~~*84*~~||~~*100*~~|~~*100*~~|~~*84*~~||~~*100*~~|74|79||~~*100*~~|~~*94*~~|79|
|1110|~~*100*~~|80|~~*83*~~||~~*100*~~|~~*100*~~|~~*83*~~||~~*100*~~|74|79||~~*100*~~|~~*94*~~|79|
|1120|~~*100*~~|80|~~*83*~~||~~*100*~~|~~*100*~~|~~*83*~~||~~*100*~~|73|79||~~*100*~~|~~*92*~~|79|
|1130|~~*100*~~|79|~~*83*~~||~~*100*~~|~~*100*~~|~~*83*~~||~~*100*~~|73|79||~~*100*~~|~~*92*~~|79|
|1140|~~*100*~~|79|~~*83*~~||~~*100*~~|~~*100*~~|~~*83*~~||~~*100*~~|73|78||~~*100*~~|~~*92*~~|78|
|1150|~~*100*~~|79|~~*83*~~||~~*100*~~|~~*100*~~|~~*83*~~||~~*100*~~|72|78||~~*100*~~|~~*91*~~|78|
|1160|~~*100*~~|79|~~*83*~~||~~*100*~~|~~*100*~~|~~*83*~~||~~*100*~~|72|78||~~*100*~~|~~*91*~~|78|

### [Fire God Dungeon](https://wiki.melvoridle.com/index.php?title=Fire_God_Dungeon), Auto Eat II

|❤️|⚔️|🏹|🧙||⚔️ ☠️|🏹 ☠️|🧙 ☠️||⚔️ 💍|🏹 💍|🧙 💍||⚔️ ☠️ 💍|🏹 ☠️ 💍|🧙 ☠️ 💍|
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|620|~~*100*~~|~~*85*~~|~~*86*~~||~~*100*~~|~~*100*~~|~~*86*~~||~~*100*~~|80|~~*84*~~||~~*100*~~|~~*100*~~|~~*84*~~|
|630|~~*100*~~|~~*84*~~|~~*86*~~||~~*100*~~|~~*100*~~|~~*86*~~||~~*100*~~|80|~~*84*~~||~~*100*~~|~~*100*~~|~~*84*~~|
|640|~~*100*~~|~~*84*~~|~~*86*~~||~~*100*~~|~~*100*~~|~~*86*~~||~~*100*~~|80|~~*83*~~||~~*100*~~|~~*100*~~|~~*83*~~|
|650|~~*100*~~|~~*84*~~|~~*85*~~||~~*100*~~|~~*100*~~|~~*85*~~||~~*100*~~|79|~~*83*~~||~~*100*~~|~~*100*~~|~~*83*~~|
|660|~~*100*~~|~~*83*~~|~~*85*~~||~~*100*~~|~~*100*~~|~~*85*~~||~~*100*~~|79|~~*83*~~||~~*100*~~|~~*100*~~|~~*83*~~|
|670|~~*100*~~|~~*83*~~|~~*85*~~||~~*100*~~|~~*100*~~|~~*85*~~||~~*100*~~|78|~~*82*~~||~~*100*~~|~~*99*~~|~~*82*~~|
|690|~~*100*~~|~~*82*~~|~~*85*~~||~~*100*~~|~~*100*~~|~~*85*~~||~~*100*~~|78|~~*82*~~||~~*100*~~|~~*99*~~|~~*82*~~|
|700|~~*100*~~|~~*82*~~|~~*84*~~||~~*100*~~|~~*100*~~|~~*84*~~||~~*100*~~|77|~~*82*~~||~~*100*~~|~~*98*~~|~~*82*~~|
|710|~~*100*~~|~~*82*~~|~~*84*~~||~~*100*~~|~~*100*~~|~~*84*~~||~~*100*~~|77|81||~~*100*~~|~~*98*~~|81|
|720|~~*100*~~|80|~~*84*~~||~~*100*~~|~~*100*~~|~~*84*~~||~~*100*~~|77|81||~~*100*~~|~~*98*~~|81|
|730|~~*100*~~|80|~~*84*~~||~~*100*~~|~~*100*~~|~~*84*~~||~~*100*~~|76|81||~~*100*~~|~~*96*~~|81|
|740|~~*100*~~|80|~~*83*~~||~~*100*~~|~~*100*~~|~~*83*~~||~~*100*~~|76|80||~~*100*~~|~~*96*~~|80|
|750|~~*100*~~|79|~~*83*~~||~~*100*~~|~~*100*~~|~~*83*~~||~~*100*~~|75|80||~~*100*~~|~~*95*~~|80|
|780|~~*100*~~|78|~~*82*~~||~~*100*~~|~~*99*~~|~~*82*~~||~~*100*~~|74|79||~~*100*~~|~~*94*~~|79|
|800|~~*100*~~|78|~~*82*~~||~~*100*~~|~~*99*~~|~~*82*~~||~~*100*~~|73|79||~~*100*~~|~~*92*~~|79|
|810|~~*100*~~|77|~~*82*~~||~~*100*~~|~~*98*~~|~~*82*~~||~~*100*~~|73|79||~~*100*~~|~~*92*~~|79|
|820|~~*100*~~|77|81||~~*100*~~|~~*98*~~|81||~~*100*~~|72|78||~~*100*~~|~~*91*~~|78|
|840|~~*100*~~|76|81||~~*100*~~|~~*96*~~|81||~~*100*~~|72|78||~~*100*~~|~~*91*~~|78|
|850|~~*100*~~|76|81||~~*100*~~|~~*96*~~|81||~~*100*~~|71|78||~~*100*~~|~~*90*~~|78|
|860|~~*100*~~|76|81||~~*100*~~|~~*96*~~|81||~~*100*~~|71|77||~~*100*~~|~~*90*~~|77|
|870|~~*100*~~|75|80||~~*100*~~|~~*95*~~|80||~~*100*~~|71|77||~~*100*~~|~~*90*~~|77|
|880|~~*100*~~|75|80||~~*100*~~|~~*95*~~|80||~~*100*~~|70|77||~~*100*~~|~~*88*~~|77|
|900|~~*100*~~|74|80||~~*100*~~|~~*94*~~|80||~~*100*~~|69|76||~~*100*~~|~~*87*~~|76|
|910|~~*100*~~|74|79||~~*100*~~|~~*94*~~|79||~~*100*~~|69|76||~~*100*~~|~~*87*~~|76|
|930|~~*100*~~|73|79||~~*100*~~|~~*92*~~|79||~~*100*~~|68|75||~~*100*~~|~~*86*~~|75|
|950|~~*100*~~|73|78||~~*100*~~|~~*92*~~|78||~~*100*~~|68|75||~~*100*~~|~~*86*~~|75|
|960|~~*100*~~|72|78||~~*100*~~|~~*91*~~|78||~~*100*~~|67|75||~~*100*~~|~~*84*~~|75|
|970|~~*100*~~|72|78||~~*100*~~|~~*91*~~|78||~~*100*~~|67|74||~~*100*~~|~~*84*~~|74|
|980|~~*100*~~|72|78||~~*100*~~|~~*91*~~|78||~~*100*~~|66|74||~~*100*~~|~~*83*~~|74|
|990|~~*100*~~|71|78||~~*100*~~|~~*90*~~|78||~~*100*~~|66|74||~~*100*~~|~~*83*~~|74|
|1000|~~*100*~~|71|77||~~*100*~~|~~*90*~~|77||~~*100*~~|66|74||~~*100*~~|~~*83*~~|74|
|1010|~~*100*~~|71|77||~~*100*~~|~~*90*~~|77||~~*100*~~|65|73||~~*100*~~|~~*82*~~|73|
|1020|~~*100*~~|70|77||~~*100*~~|~~*88*~~|77||~~*100*~~|65|73||~~*100*~~|~~*82*~~|73|
|1030|~~*100*~~|70|77||~~*100*~~|~~*88*~~|77||~~*100*~~|64|73||~~*100*~~|80|73|
|1040|~~*100*~~|70|76||~~*100*~~|~~*88*~~|76||~~*100*~~|64|72||~~*100*~~|80|72|
|1050|~~*100*~~|69|76||~~*100*~~|~~*87*~~|76||~~*100*~~|64|72||~~*100*~~|80|72|
|1060|~~*100*~~|69|76||~~*100*~~|~~*87*~~|76||~~*100*~~|63|72||~~*100*~~|79|72|
|1080|~~*100*~~|68|76||~~*100*~~|~~*86*~~|76||~~*100*~~|62|71||~~*100*~~|78|71|
|1090|~~*100*~~|68|75||~~*100*~~|~~*86*~~|75||~~*100*~~|62|71||~~*100*~~|78|71|
|1110|~~*100*~~|67|75||~~*100*~~|~~*84*~~|75||~~*100*~~|60|71||~~*100*~~|76|71|
|1120|~~*100*~~|67|75||~~*100*~~|~~*84*~~|75||~~*100*~~|60|70||~~*100*~~|76|70|
|1130|~~*100*~~|67|74||~~*100*~~|~~*84*~~|74||~~*100*~~|59|70||~~*100*~~|75|70|
|1140|~~*100*~~|66|74||~~*100*~~|~~*83*~~|74||~~*100*~~|59|70||~~*100*~~|75|70|
|1160|~~*100*~~|66|74||~~*100*~~|~~*83*~~|74||~~*100*~~|58|69||~~*100*~~|74|69|

### [Fire God Dungeon](https://wiki.melvoridle.com/index.php?title=Fire_God_Dungeon), Auto Eat III

|❤️|⚔️|🏹|🧙||⚔️ ☠️|🏹 ☠️|🧙 ☠️||⚔️ 💍|🏹 💍|🧙 💍||⚔️ ☠️ 💍|🏹 ☠️ 💍|🧙 ☠️ 💍|
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|480|~~*100*~~|~~*84*~~|~~*86*~~||~~*100*~~|~~*100*~~|~~*86*~~||~~*100*~~|80|~~*84*~~||~~*100*~~|~~*100*~~|~~*84*~~|
|490|~~*100*~~|~~*84*~~|~~*85*~~||~~*100*~~|~~*100*~~|~~*85*~~||~~*100*~~|80|~~*84*~~||~~*100*~~|~~*100*~~|~~*84*~~|
|500|~~*100*~~|~~*83*~~|~~*85*~~||~~*100*~~|~~*100*~~|~~*85*~~||~~*100*~~|79|~~*83*~~||~~*100*~~|~~*100*~~|~~*83*~~|
|520|~~*100*~~|~~*82*~~|~~*84*~~||~~*100*~~|~~*100*~~|~~*84*~~||~~*100*~~|78|~~*82*~~||~~*100*~~|~~*99*~~|~~*82*~~|
|540|~~*100*~~|80|~~*84*~~||~~*100*~~|~~*100*~~|~~*84*~~||~~*100*~~|77|~~*82*~~||~~*100*~~|~~*98*~~|~~*82*~~|
|550|~~*100*~~|80|~~*84*~~||~~*100*~~|~~*100*~~|~~*84*~~||~~*100*~~|77|81||~~*100*~~|~~*98*~~|81|
|560|~~*100*~~|80|~~*83*~~||~~*100*~~|~~*100*~~|~~*83*~~||~~*100*~~|76|81||~~*100*~~|~~*96*~~|81|
|570|~~*100*~~|79|~~*83*~~||~~*100*~~|~~*100*~~|~~*83*~~||~~*100*~~|76|81||~~*100*~~|~~*96*~~|81|
|580|~~*100*~~|79|~~*83*~~||~~*100*~~|~~*100*~~|~~*83*~~||~~*100*~~|75|80||~~*100*~~|~~*95*~~|80|
|590|~~*100*~~|78|~~*82*~~||~~*100*~~|~~*99*~~|~~*82*~~||~~*100*~~|75|80||~~*100*~~|~~*95*~~|80|
|600|~~*100*~~|78|~~*82*~~||~~*100*~~|~~*99*~~|~~*82*~~||~~*100*~~|74|80||~~*100*~~|~~*94*~~|80|
|610|~~*100*~~|77|~~*82*~~||~~*100*~~|~~*98*~~|~~*82*~~||~~*100*~~|74|79||~~*100*~~|~~*94*~~|79|
|620|~~*100*~~|77|81||~~*100*~~|~~*98*~~|81||~~*100*~~|73|79||~~*100*~~|~~*92*~~|79|
|630|~~*100*~~|76|81||~~*100*~~|~~*96*~~|81||~~*100*~~|73|79||~~*100*~~|~~*92*~~|79|
|640|~~*100*~~|76|81||~~*100*~~|~~*96*~~|81||~~*100*~~|72|78||~~*100*~~|~~*91*~~|78|
|650|~~*100*~~|76|80||~~*100*~~|~~*96*~~|80||~~*100*~~|72|78||~~*100*~~|~~*91*~~|78|
|660|~~*100*~~|75|80||~~*100*~~|~~*95*~~|80||~~*100*~~|71|78||~~*100*~~|~~*90*~~|78|
|670|~~*100*~~|75|80||~~*100*~~|~~*95*~~|80||~~*100*~~|71|77||~~*100*~~|~~*90*~~|77|
|680|~~*100*~~|74|79||~~*100*~~|~~*94*~~|79||~~*100*~~|70|77||~~*100*~~|~~*88*~~|77|
|700|~~*100*~~|73|79||~~*100*~~|~~*92*~~|79||~~*100*~~|69|76||~~*100*~~|~~*87*~~|76|
|720|~~*100*~~|72|78||~~*100*~~|~~*91*~~|78||~~*100*~~|68|76||~~*100*~~|~~*86*~~|76|
|730|~~*100*~~|72|78||~~*100*~~|~~*91*~~|78||~~*100*~~|68|75||~~*100*~~|~~*86*~~|75|
|740|~~*100*~~|71|78||~~*100*~~|~~*90*~~|78||~~*100*~~|67|75||~~*100*~~|~~*84*~~|75|
|750|~~*100*~~|71|77||~~*100*~~|~~*90*~~|77||~~*100*~~|67|75||~~*100*~~|~~*84*~~|75|
|760|~~*100*~~|71|77||~~*100*~~|~~*90*~~|77||~~*100*~~|66|74||~~*100*~~|~~*83*~~|74|
|770|~~*100*~~|70|77||~~*100*~~|~~*88*~~|77||~~*100*~~|66|74||~~*100*~~|~~*83*~~|74|
|780|~~*100*~~|70|76||~~*100*~~|~~*88*~~|76||~~*100*~~|65|73||~~*100*~~|~~*82*~~|73|
|790|~~*100*~~|69|76||~~*100*~~|~~*87*~~|76||~~*100*~~|65|73||~~*100*~~|~~*82*~~|73|
|800|~~*100*~~|69|76||~~*100*~~|~~*87*~~|76||~~*100*~~|64|73||~~*100*~~|80|73|
|810|~~*100*~~|68|76||~~*100*~~|~~*86*~~|76||~~*100*~~|64|72||~~*100*~~|80|72|
|820|~~*100*~~|68|75||~~*100*~~|~~*86*~~|75||~~*100*~~|63|72||~~*100*~~|79|72|
|840|~~*100*~~|67|75||~~*100*~~|~~*84*~~|75||~~*100*~~|62|71||~~*100*~~|78|71|
|850|~~*100*~~|67|74||~~*100*~~|~~*84*~~|74||~~*100*~~|62|71||~~*100*~~|78|71|
|860|~~*100*~~|66|74||~~*100*~~|~~*83*~~|74||~~*100*~~|60|71||~~*100*~~|76|71|
|870|~~*100*~~|66|74||~~*100*~~|~~*83*~~|74||~~*100*~~|60|70||~~*100*~~|76|70|
|880|~~*100*~~|65|73||~~*100*~~|~~*82*~~|73||~~*100*~~|59|70||~~*100*~~|75|70|
|900|~~*100*~~|64|73||~~*100*~~|80|73||~~*100*~~|58|69||~~*100*~~|74|69|
|910|~~*100*~~|64|72||~~*100*~~|80|72||~~*100*~~|58|69||~~*100*~~|74|69|
|920|~~*100*~~|64|72||~~*100*~~|80|72||~~*100*~~|57|69||~~*100*~~|72|69|
|930|~~*100*~~|63|72||~~*100*~~|79|72||~~*100*~~|57|68||~~*100*~~|72|68|
|940|~~*100*~~|63|72||~~*100*~~|79|72||~~*100*~~|56|68||~~*100*~~|71|68|
|950|~~*100*~~|62|71||~~*100*~~|78|71||~~*100*~~|56|68||~~*100*~~|71|68|
|960|~~*100*~~|62|71||~~*100*~~|78|71||~~*100*~~|55|67||~~*100*~~|70|67|
|970|~~*100*~~|60|71||~~*100*~~|76|71||~~*100*~~|55|67||~~*100*~~|70|67|
|980|~~*100*~~|60|70||~~*100*~~|76|70||~~*100*~~|54|66||~~*100*~~|68|66|
|990|~~*100*~~|59|70||~~*100*~~|75|70||~~*100*~~|54|66||~~*100*~~|68|66|
|1000|~~*100*~~|59|70||~~*100*~~|75|70||~~*100*~~|53|66||~~*100*~~|67|66|
|1010|~~*100*~~|58|69||~~*100*~~|74|69||~~*100*~~|53|65||~~*100*~~|67|65|
|1020|~~*100*~~|58|69||~~*100*~~|74|69||~~*100*~~|52|65||~~*100*~~|66|65|
|1040|~~*100*~~|57|68||~~*100*~~|72|68||~~*100*~~|52|64||~~*100*~~|64|64|
|1060|~~*100*~~|56|68||~~*100*~~|71|68||~~*100*~~|52|64||~~*100*~~|63|64|
|1070|~~*100*~~|56|67||~~*100*~~|71|67||~~*100*~~|51|63||~~*100*~~|63|63|
|1080|~~*100*~~|55|67||~~*100*~~|70|67||~~*100*~~|51|63||~~*100*~~|62|63|
|1100|~~*100*~~|55|67||~~*100*~~|70|67||~~*100*~~|50|62||~~*100*~~|60|62|
|1110|~~*100*~~|54|66||~~*100*~~|68|66||~~*100*~~|50|62||~~*100*~~|60|62|
|1120|~~*100*~~|54|66||~~*100*~~|68|66||~~*100*~~|50|62||~~*100*~~|59|62|
|1130|~~*100*~~|53|66||~~*100*~~|67|66||~~*100*~~|49|61||~~*100*~~|59|61|
|1140|~~*100*~~|53|65||~~*100*~~|67|65||~~*100*~~|49|61||~~*100*~~|58|61|
|1150|~~*100*~~|52|65||~~*100*~~|66|65||~~*100*~~|49|61||~~*100*~~|58|61|
|1160|~~*100*~~|52|65||~~*100*~~|66|65||~~*100*~~|48|60||~~*100*~~|56|60|

### [Into the Mist (Warmup)](https://wiki.melvoridle.com/index.php?title=Into_the_Mist_(Warmup)), Auto Eat I

|❤️|⚔️|🏹|🧙||⚔️ ☠️|🏹 ☠️|🧙 ☠️||⚔️ 💍|🏹 💍|🧙 💍||⚔️ ☠️ 💍|🏹 ☠️ 💍|🧙 ☠️ 💍|
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|1030|~~*100*~~|~~*100*~~|~~*100*~~||~~*100*~~|~~*100*~~|~~*100*~~||~~*100*~~|~~*100*~~|~~*100*~~||~~*100*~~|~~*100*~~|~~*100*~~|

### [Into the Mist (Warmup)](https://wiki.melvoridle.com/index.php?title=Into_the_Mist_(Warmup)), Auto Eat II

|❤️|⚔️|🏹|🧙||⚔️ ☠️|🏹 ☠️|🧙 ☠️||⚔️ 💍|🏹 💍|🧙 💍||⚔️ ☠️ 💍|🏹 ☠️ 💍|🧙 ☠️ 💍|
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|1030|~~*100*~~|~~*100*~~|~~*100*~~||~~*100*~~|~~*100*~~|~~*100*~~||~~*100*~~|~~*100*~~|~~*100*~~||~~*100*~~|~~*100*~~|~~*100*~~|

### [Into the Mist (Warmup)](https://wiki.melvoridle.com/index.php?title=Into_the_Mist_(Warmup)), Auto Eat III

|❤️|⚔️|🏹|🧙||⚔️ ☠️|🏹 ☠️|🧙 ☠️||⚔️ 💍|🏹 💍|🧙 💍||⚔️ ☠️ 💍|🏹 ☠️ 💍|🧙 ☠️ 💍|
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|960|~~*100*~~|~~*84*~~|~~*86*~~||~~*100*~~|~~*100*~~|~~*98*~~||~~*100*~~|80|~~*83*~~||~~*100*~~|~~*100*~~|~~*94*~~|
|990|~~*100*~~|~~*83*~~|~~*86*~~||~~*100*~~|~~*100*~~|~~*98*~~||~~*100*~~|80|~~*82*~~||~~*100*~~|~~*100*~~|~~*92*~~|
|1000|~~*100*~~|~~*83*~~|~~*85*~~||~~*100*~~|~~*100*~~|~~*96*~~||~~*100*~~|79|~~*82*~~||~~*100*~~|~~*100*~~|~~*92*~~|
|1020|~~*100*~~|~~*83*~~|~~*85*~~||~~*100*~~|~~*100*~~|~~*96*~~||~~*100*~~|79|80||~~*100*~~|~~*100*~~|~~*91*~~|
|1030|~~*100*~~|~~*82*~~|~~*84*~~||~~*100*~~|~~*100*~~|~~*95*~~||~~*100*~~|79|80||~~*100*~~|~~*100*~~|~~*91*~~|
|1040|~~*100*~~|~~*82*~~|~~*84*~~||~~*100*~~|~~*100*~~|~~*95*~~||~~*100*~~|78|80||~~*100*~~|~~*99*~~|~~*91*~~|
|1050|~~*100*~~|~~*82*~~|~~*84*~~||~~*100*~~|~~*100*~~|~~*95*~~||~~*100*~~|78|79||~~*100*~~|~~*99*~~|~~*90*~~|
|1070|~~*100*~~|~~*82*~~|~~*83*~~||~~*100*~~|~~*100*~~|~~*94*~~||~~*100*~~|78|79||~~*100*~~|~~*99*~~|~~*90*~~|
|1080|~~*100*~~|80|~~*83*~~||~~*100*~~|~~*100*~~|~~*94*~~||~~*100*~~|77|78||~~*100*~~|~~*98*~~|~~*88*~~|
|1110|~~*100*~~|80|~~*82*~~||~~*100*~~|~~*100*~~|~~*92*~~||~~*100*~~|77|77||~~*100*~~|~~*98*~~|~~*87*~~|
|1120|~~*100*~~|80|~~*82*~~||~~*100*~~|~~*100*~~|~~*92*~~||~~*100*~~|76|77||~~*100*~~|~~*96*~~|~~*87*~~|
|1130|~~*100*~~|79|~~*82*~~||~~*100*~~|~~*100*~~|~~*92*~~||~~*100*~~|76|77||~~*100*~~|~~*96*~~|~~*87*~~|
|1140|~~*100*~~|79|80||~~*100*~~|~~*100*~~|~~*91*~~||~~*100*~~|76|76||~~*100*~~|~~*96*~~|~~*86*~~|
|1160|~~*100*~~|79|80||~~*100*~~|~~*100*~~|~~*91*~~||~~*100*~~|75|76||~~*100*~~|~~*95*~~|~~*86*~~|

### [Into the Mist (Boss)](https://wiki.melvoridle.com/index.php?title=Into_the_Mist_(Boss)), Auto Eat I

|❤️|⚔️|🏹|🧙||⚔️ ☠️|🏹 ☠️|🧙 ☠️||⚔️ 💍|🏹 💍|🧙 💍||⚔️ ☠️ 💍|🏹 ☠️ 💍|🧙 ☠️ 💍|
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|1030|~~*100*~~|~~*100*~~|~~*100*~~||~~*100*~~|~~*100*~~|~~*100*~~||~~*100*~~|~~*100*~~|~~*100*~~||~~*100*~~|~~*100*~~|~~*100*~~|

### [Into the Mist (Boss)](https://wiki.melvoridle.com/index.php?title=Into_the_Mist_(Boss)), Auto Eat II

|❤️|⚔️|🏹|🧙||⚔️ ☠️|🏹 ☠️|🧙 ☠️||⚔️ 💍|🏹 💍|🧙 💍||⚔️ ☠️ 💍|🏹 ☠️ 💍|🧙 ☠️ 💍|
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|870|~~*100*~~|~~*100*~~|~~*84*~~||~~*100*~~|~~*100*~~|~~*84*~~||~~*100*~~|~~*100*~~|81||~~*100*~~|~~*100*~~|81|
|910|~~*100*~~|~~*100*~~|~~*83*~~||~~*100*~~|~~*100*~~|~~*83*~~||~~*100*~~|~~*100*~~|80||~~*100*~~|~~*100*~~|80|
|960|~~*100*~~|~~*100*~~|~~*82*~~||~~*100*~~|~~*100*~~|~~*82*~~||~~*100*~~|~~*100*~~|79||~~*100*~~|~~*100*~~|79|
|1000|~~*100*~~|~~*100*~~|~~*82*~~||~~*100*~~|~~*100*~~|~~*82*~~||~~*100*~~|~~*100*~~|78||~~*100*~~|~~*100*~~|78|
|1010|~~*100*~~|~~*100*~~|81||~~*100*~~|~~*100*~~|81||~~*100*~~|~~*100*~~|78||~~*100*~~|~~*100*~~|78|
|1060|~~*100*~~|~~*100*~~|80||~~*100*~~|~~*100*~~|80||~~*100*~~|~~*100*~~|77||~~*100*~~|~~*100*~~|77|
|1100|~~*100*~~|~~*100*~~|80||~~*100*~~|~~*100*~~|80||~~*100*~~|~~*100*~~|76||~~*100*~~|~~*100*~~|76|
|1120|~~*100*~~|~~*100*~~|79||~~*100*~~|~~*100*~~|79||~~*100*~~|~~*100*~~|76||~~*100*~~|~~*100*~~|76|
|1150|~~*100*~~|~~*100*~~|79||~~*100*~~|~~*100*~~|79||~~*100*~~|~~*100*~~|75||~~*100*~~|~~*100*~~|75|
|1160|~~*100*~~|~~*100*~~|79||~~*100*~~|~~*100*~~|79||~~*100*~~|~~*100*~~|75||~~*100*~~|~~*100*~~|75|

### [Into the Mist (Boss)](https://wiki.melvoridle.com/index.php?title=Into_the_Mist_(Boss)), Auto Eat III

|❤️|⚔️|🏹|🧙||⚔️ ☠️|🏹 ☠️|🧙 ☠️||⚔️ 💍|🏹 💍|🧙 💍||⚔️ ☠️ 💍|🏹 ☠️ 💍|🧙 ☠️ 💍|
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|680|~~*100*~~|~~*100*~~|~~*83*~~||~~*100*~~|~~*100*~~|~~*83*~~||~~*100*~~|~~*100*~~|81||~~*100*~~|~~*100*~~|81|
|710|~~*100*~~|~~*100*~~|~~*83*~~||~~*100*~~|~~*100*~~|~~*83*~~||~~*100*~~|~~*100*~~|80||~~*100*~~|~~*100*~~|80|
|720|~~*100*~~|~~*100*~~|~~*82*~~||~~*100*~~|~~*100*~~|~~*82*~~||~~*100*~~|~~*100*~~|80||~~*100*~~|~~*100*~~|80|
|750|~~*100*~~|~~*100*~~|~~*82*~~||~~*100*~~|~~*100*~~|~~*82*~~||~~*100*~~|~~*100*~~|79||~~*100*~~|~~*100*~~|79|
|760|~~*100*~~|~~*100*~~|81||~~*100*~~|~~*100*~~|81||~~*100*~~|~~*100*~~|79||~~*100*~~|~~*100*~~|79|
|780|~~*100*~~|~~*100*~~|81||~~*100*~~|~~*100*~~|81||~~*100*~~|~~*100*~~|78||~~*100*~~|~~*100*~~|78|
|800|~~*100*~~|~~*100*~~|80||~~*100*~~|~~*100*~~|80||~~*100*~~|~~*100*~~|78||~~*100*~~|~~*100*~~|78|
|820|~~*100*~~|~~*100*~~|80||~~*100*~~|~~*100*~~|80||~~*100*~~|~~*100*~~|77||~~*100*~~|~~*100*~~|77|
|840|~~*100*~~|~~*100*~~|79||~~*100*~~|~~*100*~~|79||~~*100*~~|~~*100*~~|77||~~*100*~~|~~*100*~~|77|
|860|~~*100*~~|~~*100*~~|79||~~*100*~~|~~*100*~~|79||~~*100*~~|~~*100*~~|76||~~*100*~~|~~*100*~~|76|
|880|~~*100*~~|~~*100*~~|78||~~*100*~~|~~*100*~~|78||~~*100*~~|~~*100*~~|76||~~*100*~~|~~*100*~~|76|
|890|~~*100*~~|~~*100*~~|78||~~*100*~~|~~*100*~~|78||~~*100*~~|~~*100*~~|75||~~*100*~~|~~*100*~~|75|
|920|~~*100*~~|~~*100*~~|77||~~*100*~~|~~*100*~~|77||~~*100*~~|~~*100*~~|75||~~*100*~~|~~*100*~~|75|
|930|~~*100*~~|~~*100*~~|77||~~*100*~~|~~*100*~~|77||~~*100*~~|~~*100*~~|74||~~*100*~~|~~*100*~~|74|
|960|~~*100*~~|~~*100*~~|76||~~*100*~~|~~*100*~~|76||~~*100*~~|~~*100*~~|73||~~*100*~~|~~*100*~~|73|
|1000|~~*100*~~|~~*100*~~|75||~~*100*~~|~~*100*~~|75||~~*100*~~|~~*100*~~|72||~~*100*~~|~~*100*~~|72|
|1020|~~*100*~~|~~*100*~~|75||~~*100*~~|~~*100*~~|75||~~*100*~~|~~*98*~~|72||~~*100*~~|~~*98*~~|72|
|1040|~~*100*~~|~~*100*~~|74||~~*100*~~|~~*100*~~|74||~~*100*~~|~~*96*~~|71||~~*100*~~|~~*96*~~|71|
|1060|~~*100*~~|~~*100*~~|74||~~*100*~~|~~*100*~~|74||~~*100*~~|~~*94*~~|71||~~*100*~~|~~*94*~~|71|
|1070|~~*100*~~|~~*100*~~|74||~~*100*~~|~~*100*~~|74||~~*100*~~|~~*94*~~|70||~~*100*~~|~~*94*~~|70|
|1080|~~*100*~~|~~*100*~~|73||~~*100*~~|~~*100*~~|73||~~*100*~~|~~*92*~~|70||~~*100*~~|~~*92*~~|70|
|1100|~~*100*~~|~~*100*~~|73||~~*100*~~|~~*100*~~|73||~~*100*~~|~~*90*~~|70||~~*100*~~|~~*90*~~|70|
|1110|~~*100*~~|~~*100*~~|73||~~*100*~~|~~*100*~~|73||~~*100*~~|~~*90*~~|69||~~*100*~~|~~*90*~~|69|
|1120|~~*100*~~|~~*100*~~|72||~~*100*~~|~~*100*~~|72||~~*100*~~|~~*88*~~|69||~~*100*~~|~~*88*~~|69|
|1140|~~*100*~~|~~*100*~~|72||~~*100*~~|~~*100*~~|72||~~*100*~~|~~*86*~~|68||~~*100*~~|~~*86*~~|68|
|1150|~~*100*~~|~~*98*~~|72||~~*100*~~|~~*98*~~|72||~~*100*~~|~~*86*~~|68||~~*100*~~|~~*86*~~|68|
|1160|~~*100*~~|~~*98*~~|71||~~*100*~~|~~*98*~~|71||~~*100*~~|~~*84*~~|68||~~*100*~~|~~*84*~~|68|

