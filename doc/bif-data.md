{:data; ... :}
==============

Set local data from json file.

```html
{:data; local-data.json :}
```
The scope of the data is the file where it is loaded and its children.

To access the variables, the prefix "local::" must be used:

```html
{:;local::varname:}
```
Modifiers
---------

```html
{:+data; ... :}
{:^data; ... :}
```
The "not" modifier prevents the file from being reload if it has already been parsed.

```html
{:!data; file.json :}
```

Flags
-----

```html
{:data; {:flg; noparse :} >> ... :}
```

### Flag: noparse

The "noparse" flag prevents the file to be datad from being parsed.

```html
{:data; {:flg; noparse :} >> file.json :}
```
Examples
--------

Assumes local-data.json:

```json
{
    "data": {
        "hello": "Hello!"
    }
}
```

Then:

```html
{:;local::hello:}
```

Output:

```html
Hello!
```