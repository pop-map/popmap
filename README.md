# PopMap API description

Publicly publish messages (called *pops*) on the globe (geolocation) and receive responses (called *peps*) from anyone.

## REQUESTS

| request | method | path | body type | status | response type |
|---------|--------|------|------|--------|----------|
| Post a new *pop* | **`POST`** | `/` `pop` | **`post_pop`** | **`CREATED`** | **`uuid`** |
| Get *pops* in an area | **`POST`** | `/` `area` | **`area`** | **`OK`** | **`list_pops`** |
| Get specific *pop* | **`GET`** | `/` `pop` `/` **`uuid`** | | **`OK`** | **`get_pop`** |
| Post a *pep* in a *pop* | **`POST`** | `/` `in` `/` **`uuid`** | **`post_pep`** | **`CREATED`** | **`index`** |
| Get scpecific *pep* | **`GET`** | `/` `in` `/` **`uuid`** `/` **`index`** | | **`OK`** | **`get_pep`** |


## TYPES

---

**`uuid`** `:string (uuid-v4) len(36) with('a-f', '0-9', '-')`

<details><summary>Example (click to expand)</summary>
    
```json
"213d0e25-aac2-4309-a1c8-420cea7cec68"
```
    
</details>

---

**`index`** `:number (ordinal) [0..]`

<details><summary>Example (click to expand)</summary>

```json
12
```

</details>
    
---

**`time`** `:number (second) [..]`

Unix time, i.e seconds since 00:00:00 UTC 1 January 1970

<details><summary>Example (click to expand)</summary>

```json
1668631642
```

</details>

---

**`user_auth`** `:object (telegram-user)`
- **id** `:number [0..]`
- **auth_date** **`:time`**
- **first_name** `:string`
- **last_name** `:string`
- **photo_url** `:string (url)`
- **hash** `:string (sha256) len(64)`

Data provided by ***Telegram*** to authenticate a user.

<details><summary>Example (click to expand)</summary>

```json
{
    "id": 45109345,
    "auth_date": 1668638455,
    "first_name": "David",
    "last_name": "Sirt",
    "photo_url": "https://t.me/i/...",
    "hash": "f6911206ac5e748bf6911206ac5e748bf6911206ac5e748bf6911206ac5e748b"
}
```
    
</details>

---

**`user_info`** `:object`
- **id** `:number [0..]`
- **first_name** `:string`
- **last_name** `:string`
- **photo_url** `:string (url)`

<details><summary>Example (click to expand)</summary>

```json
{
    "id": 45109345,
    "first_name": "David",
    "last_name": "Sirt",
    "photo_url": "https://t.me/i/...",
}
```

</details>

---

**`angle`** `:number (arc second) [..]`

A degree encodeed as an integer value as arc-seconds (360x60x60). This encoding is used to guaranty equality even after serialization and deserialization.

<details><summary>Examples (click to expand)</summary>

```json
950267
```

```json
-534478
```
   
</details>

---

**`location`** `:object`
- **lat** **`:angle`** `[-90x60x60 .. +90x60x60]`
- **lng** **`:angle`** `[-180x60x60+1 .. 180x60x60]`

Geolocation, with latitude and longitude in arc-seconds (degress x60 x60).

<details><summary>Example (click to expand)</summary>

```json
{
    "lat": 950267,
    "lng": -53478
}
```

</details>

---

**`area`** `:object`
- **lat** **`:angle`** `[-90x60x60 .. +90x60x60]`
- **lng** **`:angle`** `[-180x60x60+1 .. 180x60x60]`
- **radius** `:number (meter) [1..]`

Area with center as geolocation and radius from center.

<details><summary>Example (click to expand)</summary>

```json
{
    "lat": 950267,
    "lng": -53478
    "radius": 450
}
```

</details>

---

**`post_pop`** `:object`
- **title** `:string len(0..512)`
- **description** `:string (markdown) len(0..4096)`
- **user**: **`user_auth`**
- **location** **`:location`**
- **expire** **`:time`**

A new *pop* (message).

<details><summary>Example (click to expand)</summary>

```json
{
    "title": "Hello World",
    "description": "The **world** gives itself to the *inocent* eye.",
    "user": {
        "id": 45109345,
        "auth_date": 1668638455,
        "first_name": "David",
        "last_name": "Sirt",
        "photo_url": "https://t.me/i/...",
        "hash": "f6911206ac5e748bf6911206ac5e748bf6911206ac5e748bf6911206ac5e748b"
    }
    "location": {
        "lat": 950267,
        "lng": -53478
    },
    "expire": 1668638702
}
```

</details>

---

**`list_pops`** `:array (list) len(0..)`
- **`uuid`**

List of *pops* uuid.

<details><summary>Examples (click to expand)</summary>

```json
[
    "41f4d087-ecf7-4215-9cc2-69426148c55f",
    "a8e0a97f-56c0-42a9-bf12-127c1d8d2648",
    "9ba90a74-8229-4f21-9b8b-fe09505fa455"
]
```

```json
[
]
```

```json
[
    "9ba90a74-8229-4f21-9b8b-fe09505fa455"
]
```
    
</details>

---

**`get_pop`** `:object`
- **title** `:string len(0..512)`
- **description** `:string (markdown) len(0..4096)`
- **user** **`:user_info`**
- **location** **`:location`**
- **created** **`:time`**
- **expire** **`:time`**
- **peps** `:number (cardinal) [0..]`

A fetched *pop*.

<details><summary>Example (click to expand)</summary>

```json
{
    "title": "Hello World",
    "description": "The **world** gives itself to the *inocent* eye.",
    "user": {
        "id": 45109345,
        "first_name": "David",
        "last_name": "Sirt",
        "photo_url": "https://t.me/i/...",
    }
    "location": {
        "lat": 950267,
        "lng": -53478
    },
    "created": 1668631642,
    "expire": 1668638702,
    "peps": 14
}
```

</details>

---

**`post_pep`** `:object`
- **content** `:string (markdown) len(0..4096)`
- **user** **`user_auth`**

A new *pep* (response to a message).

<details><summary>Example (click to expand)</summary>

```json
{
    "content": "The *inoncent* eye ignores the **world** lays on chaos",
    "user": {
        "id": 6730173,
        "auth_date": 1668638780,
        "first_name": "Aristide",
        "last_name": "Marsaw",
        "photo_url": "https://t.me/i/...",
        "hash": "f6911206ac5e748bf6911206ac5e748bf6911206ac5e748bf6911206ac5e748b"
    }
}
```

</details>

---

**`get_pep`** `:object`
- **content** `:string (markdown) len(0..4096)`
- **user** **`:user_info`**
- **created** **`:time`**

A fetched *pep* (response).

<details><summary>Example (click to expand)</summary>

```json
{
    "content": "The *inoncent* eye ignores the **world** lays on chaos",
    "user": {
        "id": 6730173,
        "first_name": "Aristide",
        "last_name": "Marsaw",
        "photo_url": "https://t.me/i/...",
    }
    "created": 1668631642
}
```

</details>

---
