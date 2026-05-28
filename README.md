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
| `LIBRARY` | .library.json | From which path loads cards library
| `ADDRESS` | 0.0.0.0:8126 | Where does server for gathering player hosts

Now cards library stores in `.json` format:

``` json
{
    ...
    "<name (identificator of card)>": {
        "name": "<displaying name (usually equals to identificator)>",
        "nature": "<nature of card (for example: mana, token, main, ...)>"
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
        "back_side_url": "<url of card's back side>" ? "",
    },
    ...
}
```

For communicating with clients used `websocket` connections to different routes:
- `/game_queue/ws` - to go in queue and play when in queue finds required number of players.
- `/lobby/<lobby id i32>/ws` - to play with players in the same lobby
- `/lobby/<lobby id i32>/watcher/ws` - to watch for players in `lobby id` lobby

When lobby starts, no more players could connect to it, even if game has ended.

# Messages

Players can communicate with server per `message`s in `json` format.

``` json
"move_card": {
    "source": <CardPointer>,
    "destination": <CardPointer>,
}
```

``` json
"change_card": {
    "target": <CardPointer>,
    "changes": <CardChange>,
}
```

``` json
"change_card_to_raw": {
    "target": <CardPointer>,
}
```

``` json
"create_card": {
    "destination": <CardPointer>,
    "name": "<name>",
}
```

``` json
"view_pile": <PilePointer> 
```

``` json
"view_card": <CardPointer> 
```

``` json
"turn_end"
```

``` json
"surrender"
```

``` json
"game_info"
```
