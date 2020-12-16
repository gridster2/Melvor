# not the best code i've ever written

from copy import deepcopy
from os.path import join, isdir, abspath, dirname
try:
    from .run_javascript import run_javascript
except ImportError:
    from run_javascript import run_javascript

# the order in which to list the dungeons
dungeon_order = [
    'Chicken Coop', 'Undead Graveyard', 'Bandit Base', 'Hall of Wizards', 'Spider Forest', 'Miolite Caves',
    'Deep Sea Ship', 'Frozen Cove', 'Dragons Den', 'Volcanic Cave', 'Infernal Stronghold', 'Air God Dungeon',
    'Water God Dungeon', 'Earth God Dungeon', 'Fire God Dungeon', 'Into the Mist']

combat_triangle = {
    'melee_vs_melee': 1,
    'melee_vs_ranged': 1.25,
    'melee_vs_magic': 0.5,
    'ranged_vs_melee': 0.95,
    'ranged_vs_ranged': 1,
    'ranged_vs_magic': 1.25,
    'magic_vs_melee': 1.25,
    'magic_vs_ranged': 0.85,
    'magic_vs_magic': 1,

    'melee_vs_melee_hc': 1,
    'melee_vs_ranged_hc': 1.25,
    'melee_vs_magic_hc': 0.25,
    'ranged_vs_melee_hc': 0.75,
    'ranged_vs_ranged_hc': 1,
    'ranged_vs_magic_hc': 1.25,
    'magic_vs_melee_hc': 1.25,
    'magic_vs_ranged_hc': 0.75,
    'magic_vs_magic_hc': 1,
}

auto_eat = [
    ('Auto Eat I', 0.2),
    ('Auto Eat II', 0.3),
    ('Auto Eat III', 0.4)
]

wasteful_ring_bonus = 0.05

damage_reduction_minimum = 0
damage_reduction_maximum = 81
hitpoints_maximum = 1030
hitpoints_minimum = 100


def generate_tables() -> list[str]:
    tables = list()
    dungeon_data = get_dungeon_data()
    for dungeon_index, dungeon in enumerate(dungeon_data):
        for auto_eat_title, auto_eat_threshold in auto_eat:
            # results are expected as a list of ints, in the order:
            #   Hitpoints,
            #   DR (Melee), DR (Ranged), DR (Magic),
            #   DR (Melee) (HC), DR (Ranged) (HC), DR (Magic) (HC),
            #   DR (Melee) (WR), DR (Ranged) (WR), DR (Magic) (WR),
            #   DR (Melee) (HC) (WR), DR (Ranged) (HC) (WR), DR (Magic) (HC) (WR)
            results = []
            # cycle through all possible hitpoint levels
            hitpoints = hitpoints_maximum + 10
            while hitpoints >= hitpoints_minimum + 10:
                hitpoints -= 10
                results_dr = []
                for include_wasteful_ring in [False, True]:
                    for is_hardcore in [False, True]:
                        for player_style in ['melee', 'ranged', 'magic']:
                            # test out damage reduction against each individual enemy in the dungeon -
                            # inefficient, but allows for special cases (like Into the Mist)
                            damage_reduction = 101
                            while damage_reduction >= 0:
                                survived = True
                                for monster in dungeon['monsters']:
                                    for enemy_style in ['melee', 'ranged', 'magic']:
                                        if 'Into the Mist' in dungeon['dungeon']:
                                            if '(Boss)' in dungeon['dungeon']:
                                                # skip the Into the Mist bosses, when player combat style is ineffective
                                                if monster['id'] == 147 and player_style != 'melee':
                                                    continue
                                                if monster['id'] == 148 and player_style != 'ranged':
                                                    continue
                                                if monster['id'] == 149 and player_style != 'magic':
                                                    continue
                                        effective_hp = hitpoints
                                        # "Affliction" caps max hp
                                        if 'Into the Mist' in dungeon['dungeon']:
                                            effective_hp /= 2
                                        modified_auto_eat_threshold = auto_eat_threshold
                                        if include_wasteful_ring:
                                            modified_auto_eat_threshold += wasteful_ring_bonus
                                        player_survivable_hit = int(effective_hp * modified_auto_eat_threshold) + 1

                                        effective_dr = damage_reduction
                                        combat_triangle_key = f'{player_style}_vs_{enemy_style}'
                                        combat_triangle_key += '_hc' if is_hardcore else ''
                                        modifier = combat_triangle[combat_triangle_key]

                                        # Malcs, the Leader of Dragons can decrease DR
                                        if monster['id'] in [146]:
                                            # not implemented in 0.18.0; don't know to put this before or after modifier
                                            pass  # effective_dr -= 10

                                        effective_dr *= modifier

                                        # "mark of death" enemies
                                        if monster['id'] in [147, 148]:
                                            effective_dr /= 2
                                        # the game rounds down, so do we
                                        effective_dr = int(effective_dr)

                                        enemy_hit = monster['maxHits'][enemy_style]
                                        enemy_hit *= (1 - (effective_dr / 100))
                                        enemy_hit = int(enemy_hit)

                                        if enemy_hit >= player_survivable_hit:
                                            survived = False
                                            break

                                    if not survived:
                                        break
                                if survived:
                                    damage_reduction -= 1
                                    continue
                                break
                            max_dr = damage_reduction + 1
                            if max_dr < damage_reduction_minimum:
                                resulting_dr = ''
                            elif max_dr > damage_reduction_maximum:
                                resulting_dr = '~~*' + str(max_dr) + '*~~'
                            else:
                                resulting_dr = str(max_dr)
                            results_dr.append(resulting_dr)
                results_dr = [str(i) for i in results_dr]
                if len([1 for i in results_dr if i == '' or '~' in i]) >= len(results_dr):
                    continue
                results_dr.insert(0, str(hitpoints))
                results.append(results_dr)

            # organize / clean the results, build the table
            # "unbeatable" dungeons, i.e. Fire God Dungeon with Auto Eat I
            if len(results) == 0:
                results = [['1030'] + ['~~*100*~~'] * 12]
            results.reverse()
            # filter out duplicates
            result_check_index = 0
            while result_check_index < len(results) - 2:
                next_index = result_check_index + 1
                while next_index < len(results) and results[result_check_index][1:] == results[next_index][1:]:
                    results.pop(next_index)
                result_check_index += 1

            # small column gaps, for readability
            for i in range(len(results)):
                results[i].insert(10, '')
                results[i].insert(7, '')
                results[i].insert(4, '')

            rows = list()
            rows.append([
                '❤️',
                '⚔️', '🏹', '🧙', '',
                '⚔️ ☠️', '🏹 ☠️', '🧙 ☠️', '',
                '⚔️ 💍', '🏹 💍', '🧙 💍', '',
                '⚔️ ☠️ 💍', '🏹 ☠️ 💍', '🧙 ☠️ 💍']
            )
            rows.append(['---' for _ in rows[0]])
            for result in results:
                rows.append([str(i) for i in result])
            table = '\n'.join(['|' + '|'.join(i) + '|' for i in rows])
            table_title = f'### {make_link(dungeon["dungeon"])}, {auto_eat_title}'
            tables.append('\n\n'.join([table_title, table, '']))
    return tables


def generate_max_hit_table() -> str:
    dungeon_data = get_dungeon_data()
    dungeon_display = list()
    dungeon_display.append(['Dungeon', 'Enemy', 'Max Hit ⚔️', 'Max Hit 🏹', 'Max Hit 🧙'])
    example_line = ['' for _ in dungeon_display[0]]
    dungeon_display.append(['---' for _ in example_line])

    for dungeon in dungeon_data:
        name_line = [i for i in example_line]
        name_line[0] = f"**{make_link(dungeon['dungeon'])}**"
        dungeon_display.append(name_line)
        # sort and remove duplicates
        dungeon_monsters = {m['id']: m for m in dungeon['monsters']}
        dungeon_monsters = sorted([dungeon_monsters[k] for k in dungeon_monsters], key=lambda x: x['id'])
        max_melee = 0
        max_ranged = 0
        max_magic = 0
        for monster in dungeon_monsters:
            monster_line = [i for i in example_line]
            monster_line[1] = make_link(monster['name'])
            monster_line[2] = monster['maxHits']['melee']
            monster_line[3] = monster['maxHits']['ranged']
            monster_line[4] = monster['maxHits']['magic']
            max_melee = max(max_melee, monster['maxHits']['melee'])
            max_ranged = max(max_ranged, monster['maxHits']['ranged'])
            max_magic = max(max_magic, monster['maxHits']['magic'])
            dungeon_display.append(monster_line)
        max_line = [i for i in example_line]
        max_line[1] = '**Maximum**'
        max_line[2] = max_melee
        max_line[3] = max_ranged
        max_line[4] = max_magic
        dungeon_display.append(max_line)
    return '\n'.join(['|'.join([''] + [str(w) for w in line]) for line in dungeon_display] + ['']) + '\n\n***\n\n'


def rebuild_document() -> None:
    docs_dir = join(abspath(dirname(__file__)), '..', 'Documents')
    if not isdir(docs_dir):
        raise FileNotFoundError(f'{docs_dir} does not exist')

    max_hit_table = generate_max_hit_table()
    dr_tables = generate_tables()
    with open(join(docs_dir, 'Damage Reduction Tables Intro.md'), 'r', encoding='utf-16') as header_file:
        header_text = header_file.read()

    with open(join(docs_dir, 'Damage Reduction Tables Mini Intro Insert.md'), 'r', encoding='utf-16') as intro_file:
        mini_insert = intro_file.read()

    with open(join(docs_dir, 'Damage Reduction Tables Mini End Insert.md'), 'r', encoding='utf-16') as outro_file:
        mini_outro = outro_file.read()

    full_output = '\n\n'.join([
        header_text.replace('%%insertminiintro%%', ''),
        max_hit_table,
        '***',
        '\n\n'.join(dr_tables)
    ])
    while '\n\n\n' in full_output:
        full_output = full_output.replace('\n\n\n', '\n\n')
    while '  ' in full_output:
        full_output = full_output.replace('  ', ' ')

    with open(join(docs_dir, 'Damage Reduction Tables for Dungeons.md'), 'w', encoding='utf-16') as output_file:
        output_file.write(full_output)

    mini_tables = []
    for dungeon in dungeon_order[dungeon_order.index('Volcanic Cave'):]:
        for table in dr_tables:
            header_line = table.split('\n')[0]
            if dungeon in header_line and 'Auto Eat III' in header_line:
                mini_tables.append(table)

    mini_output = '\n\n'.join([
        header_text.replace('%%insertminiintro%%', mini_insert),
        '***',
        '\n\n'.join(mini_tables),
        '***',
        mini_outro
    ])
    while '\n\n\n' in mini_output:
        mini_output = mini_output.replace('\n\n\n', '\n\n')
    while '  ' in mini_output:
        mini_output = mini_output.replace('  ', ' ')

    with open(join(docs_dir, 'Damage Reduction Tables for Dungeons (Mini).md'), 'w', encoding='utf-16') as mini_file:
        mini_file.write(mini_output)


def get_dungeon_data() -> list:
    dungeon_data = run_javascript('getMaxHitDungeons.js', True)
    # sort the dungeons according to the order they appear
    sorted_dungeon_data = []
    # crazy inefficient, but works fine with such a small amount
    for dungeon_name in dungeon_order:
        for dungeon in dungeon_data:
            if dungeon['dungeon'] == dungeon_name:
                sorted_dungeon_data.append(dungeon)
                break
    if len(sorted_dungeon_data) != len(dungeon_data):
        raise Exception(f'Mismatch in Dungeon Order ({len(sorted_dungeon_data)}) & Dungeon Data ({len(dungeon_data)})!')

    # treat Into the Mist as two separate dungeons, one of the standard enemies, and one as the boss
    sorted_dungeon_data.append(deepcopy(sorted_dungeon_data[-1]))
    sorted_dungeon_data[-2]['dungeon'] += ' (Warmup)'
    sorted_dungeon_data[-1]['dungeon'] += ' (Boss)'
    boss_ids = [147, 148, 149]
    sorted_dungeon_data[-2]['monsters'] = [m for m in sorted_dungeon_data[-2]['monsters'] if m['id'] not in boss_ids]
    sorted_dungeon_data[-1]['monsters'] = [m for m in sorted_dungeon_data[-1]['monsters'] if m['id'] in boss_ids]

    return sorted_dungeon_data


def make_link(title) -> str:
    return f"[{title}](https://wiki.melvoridle.com/index.php?title={title.replace(' ', '_')})"


if __name__ == '__main__':
    rebuild_document()
