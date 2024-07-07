from __future__ import annotations

import argparse
import datetime
import socket
import struct
import sys

SERVER_DATA_POST_STRINGS_PATTERN = "hccccccc"
SERVER_DATA_STRUCT_SIZE = struct.calcsize(SERVER_DATA_POST_STRINGS_PATTERN)


game_dict = {
    200: "Base Source Shared",
    201: "Source Engine 64bit (for x86_64/AMD64 CPU/OS, deprecated)",
    202: "Source Init (VAC)",
    203: "Source Shared Securom",
    206: "Base Source Shared Materials",
    207: "Base Source Shared Models",
    208: "Base Source Shared Sounds",
    209: "Source Low Violence",
    212: "Base Source Engine 2",
    216: "Source 2007 Binaries",
    217: "Multiplayer OB Binaries",
    220: "Half-Life 2",
    221: "Half-Life 2 Base Content",
    223: "Half-Life 2 French",
    224: "Half-Life 2 Italian",
    225: "Half-Life 2 German",
    226: "Half-Life 2 Spanish",
    227: "Half-Life 2 Simplified Chinese",
    228: "Half-Life 2 Korean (Teen)",
    229: "Half-Life 2 Korean (Adult)",
    230: "Half-Life 2 Traditional Chinese",
    231: "Half-Life 2 Japanese",
    232: "Half-Life 2 Russian",
    233: "Half-Life 2 Thai",
    234: "Half-Life 2 Portuguese",
    236: "Half-Life 2 Game Dialog",
    240: "Counter-Strike: Source",
    241: "Counter-Strike: Source Base Content",
    242: "Counter-Strike: Source Shared Content",
    243: "Counter-Strike: Source French",
    244: "Counter-Strike: Source Italian",
    245: "Counter-Strike: Source German",
    246: "Counter-Strike: Source Spanish",
    247: "Counter-Strike: Source Simplified Chinese",
    248: "Counter-Strike: Source Korean (Teen)",
    249: "Counter-Strike: Source Korean (Adult)",
    250: "Counter-Strike: Source Traditional Chinese",
    251: "Counter-Strike: Source Japanese",
    252: "Counter-Strike: Source Russian",
    253: "Counter-Strike: Source Thai",
    260: "Counter-Strike: Source Beta",
    280: "Half-Life: Source",
    281: "Half-Life: Source Base Content",
    283: "Half-Life: Source French",
    284: "Half-Life: Source Italian",
    285: "Half-Life: Source German",
    286: "Half-Life: Source Spanish",
    287: "Half-Life: Source Simplified Chinese",
    288: "Half-Life: Source Korean (Teen)",
    289: "Half-Life: Source Korean (Adult)",
    290: "Half-Life: Source Traditional Chinese",
    291: "Half-Life: Source Japanese",
    292: "Half-Life: Source Russian",
    293: "Half-Life: Source Thai",
    300: "Day of Defeat: Source",
    301: "Day of Defeat: Source Base Content",
    305: "Source 2007 Shared Materials",
    306: "Source 2007 Shared Models",
    307: "Source 2007 Shared Sounds",
    308: "Episodic 2007 Shared",
    312: "|all_source_engine_paths|hl2",
    320: "Half-Life 2: Deathmatch",
    321: "Half-Life 2: Deathmatch",
    340: "Half-Life 2: Lost Coast",
    341: "Half-Life 2: Lost Coast Content",
    342: "Half-Life 2: Lost Coast French",
    343: "Half-Life 2: Lost Coast German",
    344: "Half-Life 2: Lost Coast Italian",
    345: "Half-Life 2: Lost Coast Korean (Teen)",
    346: "Half-Life 2: Lost Coast Korean (Adult)",
    347: "Half-Life 2: Lost Coast Russian",
    348: "Half-Life 2: Lost Coast Simplified Chinese",
    349: "Half-Life 2: Lost Coast Spanish",
    350: "Half-Life 2: Lost Coast Traditional Chinese",
    360: "Half-Life Deathmatch: Source",
    363: "Half-Life Deathmatch: Source Client",
    380: "Half-Life 2: Episode One",
    381: "Half-Life 2: Episode One Content",
    213: "Half-Life 2: Episode One Shared",
    400: "Portal",
    401: "Portal Content",
    405: "Portal English",
    420: "Half-Life 2: Episode Two",
    421: "Half-Life 2: Episode Two Content",
    422: "Half-Life 2: Episode Two Materials",
    423: "Half-Life 2: Episode Two Maps",
    428: "Half-Life 2: Episode Two English",
    440: "Team Fortress 2",
    441: "Team Fortress 2 Content",
    442: "Team Fortress 2 Materials",
    443: "Team Fortress 2 Client Content",
    500: "Left 4 Dead",
    501: "Left 4 Dead binaries",
    502: "Left 4 Dead base",
    503: "Left 4 Dead client binary",
    504: "Left 4 Dead sound",
    550: "Left 4 Dead 2",
    590: "Left 4 Dead 2 Demo",
    570: "Dota 2 Beta",
    571: "Dota 2 Beta content",
    572: "Dota 2 Beta client",
    573: "Dota 2 Beta Win32 content",
    620: "Portal 2",
    630: "Alien Swarm",
    640: "Alien Swarm SDK Launcher",
    730: "Counter-Strike: Global Offensive",
    731: "Counter Strike Global Offensive Beta Common Content",
    732: "Counter Strike Global Offensive Beta Win32 Content",
    870: "Left 4 Dead 2 Downloadable content",
    1300: "SiN Episodes",
    1301: "SiN Episodes Materials",
    1302: "SiN Episodes Models",
    1303: "SiN Episodes Sounds",
    1304: "SiN Episodes Core",
    1305: "SiN Episodes: Emergence Content",
    1306: "SiN Episodes: Emergence German",
    1307: "SiN Episodes: Emergence German Preload",
    1315: "SiN Episodes: Emergence Russian",
    1308: "SiN Episodes Arena",
    1316: "SiN Episodes Unabridged",
    1800: "Counter-Strike: Global Offensive",
    2100: "Dark Messiah of Might and Magic",
    2130: "Dark Messiah Might and Magic Multi-Player",
    2400: "The Ship: Murder Party",
    2401: "The Ship",
    2402: "The Ship Common",
    2412: "The Ship Shared",
    2430: "The Ship Tutorial",
    2406: "The Ship Tutorial Content",
    2405: "The Ship Single Player Content",
    2450: "Bloody Good Time",
    2600: "Vampire The Masquerade - Bloodlines",
    4000: "Garry's Mod",
    4001: "Garry's Mod Content",
    4020: "Garry's Mod Dedicated Server",
    17500: "Zombie Panic! Source",
    17510: "Age of Chivalry",
    17520: "Synergy",
    17530: "D.I.P.R.I.P.",
    17550: "Eternal Silence",
    17570: "Pirates, Vikings, & Knights II",
    17580: "Dystopia",
    17700: "Insurgency",
    17710: "Nuclear Dawn",
    17730: "Smashball",
    222880: "Insurgency",
    224260: "No More Room in Hell",
    238430: "Contagion",
}

game_type_dict = {b"d": "Dedicated", b"l": "non-dedicated", b"p": "SourceTV"}

environment_dict = {
    b"l": "linux",
    b"w": "windows",
    b"m": "mac (deprecated)",
    b"o": "mac",
}


def server_game_is_the_ship(game_id):
    return game_id in range(2400, 2405 + 1)


def format_the_ship_data(the_ship_modes, game_mode, game_map, game_duration):
    print(f"""
game_mode: {the_ship_modes.get(game_mode, "Unknown")}
game_map: {int.from_bytes(game_map, byteorder='big')}
game_duration: {int.from_bytes(game_duration, byteorder='big')}""")


def get_the_ship_data(server_info_response, end_of_game_name):
    the_ship_modes = {
        "\00": "Hunt",
        "\01": "Elimination",
        "\02": "Duel",
        "\03": "Deathmatch",
        "\04": "VIP Team",
        "\05": "Team Elimination",
    }
    the_ship_struct_str = "ccc"
    the_ship_struct_size = struct.calcsize(the_ship_struct_str)
    the_ship_data = server_info_response[
        SERVER_DATA_STRUCT_SIZE + end_of_game_name : SERVER_DATA_STRUCT_SIZE
        + end_of_game_name
        + the_ship_struct_size
    ]
    game_mode, game_map, game_duration = struct.unpack(
        the_ship_struct_str, the_ship_data
    )
    return the_ship_modes, game_mode, game_map, game_duration


def get_string_from_data(data, start):
    end = data.find(b"\x00", start)
    return data[start:end].decode("utf-8"), end + 1


def main():
    ip, port = sys.argv[1:3]

    parser = argparse.ArgumentParser(description="Query Source Engine servers")

    parser.add_argument("ip", type=str, help="IP address of the server")
    parser.add_argument("port", type=int, help="Port of the server")
    parser.add_argument(
        "query", choices=["info", "players"], help="Query type", type=str.lower
    )

    args = parser.parse_args()

    query_bytes = {
        "info": b"\x54",
        "players": b"\x55",
    }

    s = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    s.connect((ip, int(port)))
    s.send(b"\xff\xff\xff\xff" + query_bytes[args.query] + b"Source Engine Query\x00")
    response = s.recv(1024)

    byte_list = [hex(b) for b in response]

    command = byte_list[4]

    if server_sent_challenge(command):
        if args.query == "info":
            send_challenge_response(s, response, "\x49")
            process_server_info_command(s)
        elif args.query == "players":
            send_players_challenge_response(s, response)
            process_server_players_command(s)

    s.close()


def send_players_challenge_response(s, response):
    challenge_response = b"\xff\xff\xff\xff\x55" + response[5:]
    s.send(challenge_response)


def process_server_players_command(s):
    server_players_response = s.recv(4096 * 100)
    header = server_players_response[:5]
    assert header == b"\xff\xff\xff\xff\x44", header
    (player_count,) = struct.unpack("h", server_players_response[5:7])
    print(f"player_count: {player_count}")
    current_byte_index = 6
    for i in range(player_count):
        last_byte_index = current_byte_index
        (index,) = struct.unpack(
            "b", server_players_response[current_byte_index : current_byte_index + 1]
        )
        current_byte_index += 1
        name, current_byte_index = get_string_from_data(
            server_players_response, current_byte_index
        )

        score = struct.unpack(
            "i", server_players_response[current_byte_index : current_byte_index + 4]
        )[0]
        current_byte_index += 4
        duration = datetime.timedelta(
            seconds=struct.unpack(
                "f",
                server_players_response[current_byte_index : current_byte_index + 4],
            )[0]
        )
        current_byte_index += 4

        print(f"{name:<25} - {score:<4} - {duration}")


def process_server_info_command(s):
    server_info_response = s.recv(4096)
    # get first two bytes
    server_info_response_id = server_info_response[:4]
    assert server_info_response_id == b"\xff\xff\xff\xff", server_info_response_id
    server_info_response_header = server_info_response[4:5]
    assert server_info_response_header == b"\x49", server_info_response_header
    start_of_strings = 5

    server_name, end_of_name = get_string_from_data(
        server_info_response, start_of_strings
    )
    map_string, end_of_map_string = get_string_from_data(
        server_info_response, end_of_name
    )
    folder_name, end_of_folder_name = get_string_from_data(
        server_info_response, end_of_map_string
    )
    game_name, end_of_game_name = get_string_from_data(
        server_info_response, end_of_folder_name
    )
    print(f"server_name: {str(server_name)}")
    print(f"map_string: {str(map_string)}")
    print(f"folder_name: {str(folder_name)}")
    print(f"game_name: {str(game_name)}")
    (
        game_id,
        players,
        max_players,
        bots,
        server_type,
        environment,
        visibility,
        vac,
    ) = struct.unpack(
        SERVER_DATA_POST_STRINGS_PATTERN,
        server_info_response[
            end_of_game_name : SERVER_DATA_STRUCT_SIZE + end_of_game_name
        ],
    )

    print(f"""
game_id: {game_id} ({game_dict.get(game_id, "Unknown")})
players: {int.from_bytes(players, byteorder='big')}
max_players: {int.from_bytes(max_players, byteorder='big')}
bots: {int.from_bytes(bots, byteorder='big')}
server_type: {game_type_dict.get(server_type, "Unknown")}
environment: {environment_dict.get(environment, "Unknown")}
visibility: {'Public' if visibility == 0 else 'Private'}
vac: {'secured' if vac == 1 else 'Unsecured'}""")

    current_byte_index = SERVER_DATA_STRUCT_SIZE + end_of_game_name
    if server_game_is_the_ship(game_id):
        the_ship_modes, game_mode, game_map, duration = get_the_ship_data(
            server_info_response, end_of_game_name
        )
        format_the_ship_data(the_ship_modes, game_mode, game_map, duration)
        current_byte_index += struct.calcsize("ccc")

    version_string, end_of_version_string = get_string_from_data(
        server_info_response, current_byte_index
    )
    print(f"version_string: {version_string}")

    extra_data_flag_struct = "c"
    extra_data_flag_struct_size = struct.calcsize(extra_data_flag_struct)
    extra_data_flag_bytes = server_info_response[
        end_of_version_string : end_of_version_string + extra_data_flag_struct_size
    ]
    extra_data_flag = int.from_bytes(
        struct.unpack(extra_data_flag_struct, extra_data_flag_bytes)[0],
        byteorder="big",
    )

    current_byte_index = end_of_version_string + extra_data_flag_struct_size

    if extra_data_flag & b"\x80"[0]:
        port = struct.unpack(
            "h", server_info_response[current_byte_index : current_byte_index + 2]
        )[0]
        print(f"port: {port}")
        current_byte_index += 2
    if extra_data_flag & b"\x10"[0]:
        server_id = struct.unpack(
            "q", server_info_response[current_byte_index : current_byte_index + 8]
        )[0]
        print(f"server_id: {server_id}")
        current_byte_index += 8
    if extra_data_flag & b"\x40"[0]:
        port = struct.unpack(
            "h",
            server_info_response[current_byte_index : current_byte_index + 2],
        )[0]
        print(f"port: {port}")
        current_byte_index += 2
        name, current_byte_index = get_string_from_data(
            server_info_response, current_byte_index
        )
        print(f"name: {name}")
    if extra_data_flag & b"\x20"[0]:
        keys, current_byte_index = get_string_from_data(
            server_info_response, current_byte_index
        )
    if extra_data_flag & b"\x01"[0]:
        game_id = struct.unpack(
            "l",
            server_info_response[current_byte_index : current_byte_index + 8],
        )[0]
        print(f"game_id: {game_id} ({game_dict.get(game_id, 'Unknown')})")


def server_sent_challenge(command):
    return command == "0x41"


def send_challenge_response(s, response, query_byte):
    challenge_response = b"\xff\xff\xff\xff\x54Source Engine Query\x00" + response[5:]
    s.send(challenge_response)


if __name__ == "__main__":
    main()
