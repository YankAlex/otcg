# otcg

Open tactical card game.

This repository include 3 crates:
- engine
- game_server
- lobby_server

To run use `cargo run`. You can specify enviroment variables

| name | default value | description
|-------|---------------|---------------------------------------|
| `GAME` | riftbound     | What game rules use
| `PLAYERS` | 2          | Players count
| `LIBRARY` | .otcglib | From which path loads cards&chips library
| `ADDRESS` | 0.0.0.0:8126 | Where does server for gathering player hosts


Now cards library stores in `.json` format:

```
{
    "cards": {
        ...
        "card name (id)": <RawCard>
        ...
    },
    "chips": {
        ...
        "chip name (id)": <RawChip>
        ...
    },
}
```

For communicating with clients used `websocket` connections to different routes:
- `/game_queue/ws` - to go in queue and play when in queue finds required number of players.
- `/lobby/<lobby id i32>/ws` - to play with players in the same lobby
- `/lobby/<lobby id i32>/watcher/ws` - to watch for players in `lobby id` lobby

When lobby starts, no more players could connect to it, even if game has ended.

# Structs in api

## `RawChip`

```
{
    "name": "<displaying name (usually equals to identificator)>",
    "health": <health i32> ? 0,
    "colors": [..., "<color symbol>", ...] ? [],
    "art_url": "<url of card art>" ? "",
}
```

## `RawCard`

```
{
    "name": "<displaying name (usually equals to identificator)>",
    "nature": "<nature of card (for example: mana, token, main, ...)>",
    "type": "<type of card (for example: unit, chamion, gear, ...)",
    "cost": <cost i32> ? 0,
    "power": <power i32> ? 0,
    "health": <health i32> ? 0,
    "colors": [..., "<color symbol>", ...] ? [],
    "tags": [..., "<tag>",...] ? [],
    "description": "<description of card>" ? "",
    "rarity": "<rarity>" ? "",
    "art_url": "<url of card art>" ? "",
    "card_picture_url": "<url of card picture>" ? "",
    "back_side_url": "<url of card's back side>" ? ""
}
```

## `RawBoard`

```
{
    height: <usize>,
    width: <usize>,
    img_url: "<url of image on board>"
}
```

## `Visibility`

```
"secret" | "private" | "public"
```

Visibile to noone | visible to owner | visible to everyone.

## `PileType`

```
{
    "battlefield": <battleground number i32 0..n>
} | {
    "name": "<pile name>"
}

(in riftbound rules) "name": "hand" | "main_deck" | "mana_deck" | "trash_deck" | "base" | "mana_pool" | "special_zone" | "heroes" | "spell_queue" | 
```

## `PilePointer`

```
{
    "player": <pile owner i32>,
    "type": <PileType>
}
```

## `CardPointer`

```
{
    "index": <card index i32 (0.. - indexed from the start, -1 - space in the end, ..-2 - indexed from the end)>,
    "pile": <PilePointer>
}
```

## `BoardPointer`

```
{
    "name": "<board name>"
}

(in unmatched rules) "name": "board"
```

## `ChipPointer`

```
{
    "board": <BoardPointer>
    "index": <card index i32 (as it is in CardPointer)>
}

(in unmatched rules) "name": "board"
```

## `CardChange`

```
{
    "power": <? change power to i32>,
    "health": <? change health to i32>,
    "cost": <? change cost to i32>,
    "color_cost": <? change color_cost to i32>,
    "description": <? change description to [..., "<color_symbol>", ...]>,
    "tags": <? change tags [..., "<tag>", ...]>,
    "visibility": <? change visibility to Visibility>,
    "comments": "<? change comments to>",
    "tapped": <? change tapped to bool>
}
```

## `ChipChange`

```
{
    "health": <? change health to i32>,
    "visibility": <? change visibility to Visibility>,
    "coordinates": <Coordinates>,
}
```

## `CardView`

```
{
    "raw": <? RawCard>,
    "type": ? "<type>",
    "rarity": ? "<rarity>",
    "power": <? i32>,
    "health": <? i32>,
    "cost": <? i32>,
    "color_cost": ? [..., "<color_symbol>", ...],
    "description": ? "<description>",
    "tags": ? [..., "<tag>", ...],
    "colors": ? [..., "color_symbol", ...],
    "visibility": <Visibility>,
    "tapped": bool,
    "owner": <Player>,
    "art_url": ? "<art url>",
    "card_picture_url": ? "<card picture url>",
    "comments": "<comments>",
    "nature": "<nature>",
    "visible_to_me": <is it card visivle to player bool>,
    "back_side_url": "<card back side url>"
}
```

It represents how player can see a card.

## `Coordinates`

```
{
    "x": <f32>,
    "y": <f32>
}
```

## `ChipView`

```
{
    "raw": <? RawChip>,
    "health": <? i32>,
    "owner": <Player>,
    "coordinates": <Coordinates>
}
```

It represents how player can see a chip.

## `PileView`

```
{
    "only_raw_cards": <are cards in this pile raw bool>,
    "default_visibility": <default visivility of card Visibility>,
    "cards": [..., <CardView> ,...]
}
```

It represents how player can see a pile.

## `BoardView`

```
{
    "raw": <RawBoard>,
    "img_url": <current img of board, initial you can get from "raw" propertie>,
    "chips": [..., <ChipView> ,...]
}
```

It represents how player can see a board.

# Messages

Players can communicate with server per `message`s in `json` format.

## Move card

```
{
    "move_card": {
        "source": <CardPointer>,
        "destination": <CardPointer>
    }
}
```

Player moves card from `source` to `destination`.

## Change card

```
{
    "change_card": {
        "target": <CardPointer>,
        "changes": <CardChange>
    }
}
```

Player changes card at `target` by `changes`.

## Change chip

```
{
    "change_chip": {
        "target": <ChipPointer>,
        "changes": <ChipChange>
    }
}
```

Player changes chip at `target` by `changes`.

## Change card to raw

```
{
    "change_card_to_raw": {
        "target": <CardPointer>
    }
}
```

Player returns card at `target` to initial state, without any changes.

## Create card

```
{
    "create_card": {
        "destination": <CardPointer>,
        "name": "<name>"
    }
}
```

Player creates a card at `destination` by `name`.

## Create chip

```
{
    "create_chip": {
        "destination": <ChipPointer>,
        "name": "<name>",
        "coordinates": <Coordinates>
    }
}
```

Player creates a chip at `destination` on `coordinates` by `name`.

## View pile

```
{
    "view_pile": <PilePointer> 
}
```

Player views `view_pile` pile.

## View board

```
{
    "view_board": <BoardPointer> 
}
```

Player views `view_pile` board.

## View card

```
{
    "view_card": <CardPointer> 
}
```

Player views `view_card` card.

## View chip

```
{
    "view_chip": <ChipPointer> 
}
```

Player views `view_chip` chip.

## Turn end

```
"turn_end"
```

Player ends turn.

## Surrender

```
"surrender"
```

Player surrneds. _Now it is'nt handling_.

## Game info

```
"game_info"
```

Player wants to get information about game state.

## Background

>[!WARNING] Background is special message, which needs to send only after *Background request*.

```
{
    "piles": {
        ...
        "name": [..., "<card name>", ...],
        ...
    }
}

(in riftbounds rules:)

{
    "piles": {
        "main_deck": [..., "<card name>", ...],
        "mana_deck": [..., "<card name>", ...],
        "special_zone": [..., "<card name>", ...],
        "heroes": [..., "<card name>", ...],
        "base": [..., "<card name>", ...],
    }
}
```

Player sends his begin position of cards.

# Actions

Server communicates with players by `action`s in `json` format.

## Card moved

```
{
    "card_moved": {
        "source": <CardPointer>,
        "destination": <CardPointer>
    }
}
```

Some player moved card from `source` to `destination`.

## Card changed

```
{
    "card_changed": {
        "target": <CardPointer>,
        "new_card": <CardView>
    }
}
```

Some player changed card at `target` to `new_card`.

## Chip changed

```
{
    "chip_changed": {
        "target": <ChipPointer>,
        "new_chip": <ChipView>
    }
}
```

Some player changed chip at `target` to `new_chip`.

## Card created

```
{
    "card_created": {
        "destination": <CardPointer>,
        "card": <CardView>
    }
}
```

Some player created a `card` card at `destination`.

## Chip created

```
{
    "chip_created": {
        "destination": <ChipPointer>,
        "chip": <ChipView>
    }
}
```

Some player created a `card` card at `destination`.

## View pile

```
{
    "view_pile": {
        "target": <PilePointer>,
        "pile": <PileView> 
    }
}
```

Player viewed `pile` at `target`.

## View board

```
{
    "view_board": {
        "target": <BoardPointer>,
        "board": <BoardView> 
    }
}
```

Player viewed `board` at `target`.

## View card

```
{
    "view_card": {
        "target": <CardPointer>,
        "card": <CardView>
    }
}
```

Player viewed `card` at `target`.

## View chip

```
{
    "view_chip": {
        "target": <ChipPointer>,
        "card": <ChipView>
    }
}
```

Player viewed `chip` at `target`.

## Next turn

```
{
    "next_turn": <Player>
}
```

Next turn of `next_turn` player.

## Game info

```
{
    "game_info": {
        "your_number": <number of player-receiver i32>,
        "players_count": <players count usize>,
        "battlefields_count": <battlefields count usize>
    }
}
```

Information about current game.

## BackgroundRequest

```
"background_request"
```

Server requests `Background` from player.

# Communication

```
Client                         Server

           +[Connection]+
      +[Waiting for game start]+

   <-----(BackgroundRequest)-----< 
    
   >--------(Background)--------->

    +[Waiting for other players]+

   <---------(GameInfo)----------< 

   <<======(Communication)======>>
```
