/* this should be used to generate the dungeon data */

/* both of these need to be nulled to prevent errors when accessing the page without logging in */
updatePlayerStats = function() {};
startCombat = function() {};

function timeout(ms) {
	return new Promise(resolve => setTimeout(resolve, ms));
}

async function sleep(ms) {
	await timeout(ms);
}

let cachedMaxHits = {};

async function getMaxHit(monsterId) {
	if (cachedMaxHits[monsterId] != null) {
		console.log('\t', monsterId, cachedMaxHits[monsterId]);
		return {
			"name": MONSTERS[monsterId].name,
			"id": monsterId,
			"maxHits": cachedMaxHits[monsterId]
		};
	}
	isDungeon = false;
	isGolbinRaid = false;
	forcedEnemy = monsterId;
	monsterStats[monsterId] = {
		'stats': 0
	}
	/* the max hit calculation requires the enemy to be loaded; therefore, we need to wait for it to load */
	loadNewEnemy();
	await sleep(4000);

	let maxHits = {};
	maxHits[CONSTANTS.attackType.Melee] = [0];
	maxHits[CONSTANTS.attackType.Ranged] = [0];
	maxHits[CONSTANTS.attackType.Magic] = [0];
	let specialAttackChance = 0;
	if (MONSTERS[monsterId].hasSpecialAttack) {
		for (let specialAttackIndex = 0; specialAttackIndex < MONSTERS[monsterId].specialAttackID.length; specialAttackIndex ++) {
			let specialAttack = enemySpecialAttacks[MONSTERS[monsterId].specialAttackID[specialAttackIndex]];
			specialAttackChance += specialAttack.chance;
			let setDOTDamage = 0;
			let setDamage = 0;
			if (specialAttack.setDamage != null) setDamage = specialAttack.setDamage;
			if (specialAttack.setDOTDamage != null) setDOTDamage = specialAttack.setDOTDamage;
			setDamage = Math.max(...[setDamage, setDOTDamage]);
			if (specialAttack.stunDamageMultiplier != null && specialAttack.stunDamageMultiplier > 1) {
				setDamage = setDamage * specialAttack.stunDamageMultiplier;
			} 
			setDamage = setDamage * 10;

			maxHits[combatData.enemy.attackType].push(setDamage);
		}
	}

	/* some enemies only have special attacks, so ignore their (potentially higher) normal damage */
	if (specialAttackChance < 100) maxHits[combatData.enemy.attackType].push(combatData.enemy.maximumStrengthRoll);

	cachedMaxHits[monsterId] = {
		/* max on an empty array gives -Infinity, so correct it to a min of 0 */
		"melee": Math.max(0, Math.max(...maxHits[CONSTANTS.attackType.Melee])),
		"ranged": Math.max(0, Math.max(...maxHits[CONSTANTS.attackType.Ranged])),
		"magic": Math.max(0, Math.max(...maxHits[CONSTANTS.attackType.Magic]))
	};
	return await getMaxHit(monsterId);
}

async function getMaxHitDungeon(dungeonIndex) {
	let dungeon = DUNGEONS[dungeonIndex];
	let dungeonMonsters;
	if (dungeon.name === "Into the Mist") {
		let originalDungeonMonsters = dungeon.monsters;
		dungeonMonsters = [];
		for (let i = 0; i < MONSTERS.length; i++) {
			if (getMonsterCombatLevel(i, true) >= 165 && getMonsterCombatLevel(i, true) <= 677 && i !== 134 && i !== 135 && i !== 136) dungeonMonsters.push(i);
		}
		for (let i = 0; i < originalDungeonMonsters.length; i++) {
			if (i >= 20) dungeonMonsters.push(originalDungeonMonsters[i]);
		}
	}
	else {
		dungeonMonsters = dungeon.monsters;
	}
	let unduplicatedMonsters = [];
	for (let i = 0; i < dungeonMonsters.length; i++) {
		let element = dungeonMonsters[i];
		if (unduplicatedMonsters.includes[element]) continue;
		unduplicatedMonsters.push(element);
	}

	console.log(dungeon.name, unduplicatedMonsters);

	let monsterData = [];
	for (let index = 0; index < unduplicatedMonsters.length; index ++) {
		monsterData.push(await getMaxHit(unduplicatedMonsters[index]));
	}

	return {
		"dungeon": dungeon.name,
		"monsters": monsterData
	}
}

async function getMaxHitDungeons() {
	let dungeonData = [];
	for (let dungeonIndex = 1; dungeonIndex <= DUNGEONS.length; dungeonIndex++) {
		dungeonData.push(await getMaxHitDungeon(DUNGEONS.length - dungeonIndex));
	}
	return dungeonData.reverse();
}

console.log('%|%REPLACE|ME%|%', JSON.stringify(await getMaxHitDungeons()));