{:;:} (unprintable)
===================

Output empty string, eliminates or preserves spaces.

```html
{:;:}
```

Modifiers:

```html
{:^;:}
```

No flags
--------

The following are the same:

```html
<div>{:;:}</div>
<div>{:; :}</div>
<div>{:;     :}</div>
<div>{:;

:}</div>
```

Output:

```html
<div></div>
<div></div>
<div></div>
<div></div>
```

The usual behavior in output is as expected in practically all cases, the following produce the same output:

```html
<div>{:code;Hello:}</div>
<div>{:code; Hello :}</div>
<div>{:code;

    Hello

:}</div>
```

Output:

```html
<div>Hello</div>
<div>Hello</div>
<div>Hello</div>
```

But in special cases we may need to make it so that spaces or carriage returns are not removed. And this is the main use of unprintable Bif:

```html
<pre>
{:code;
    {:^;:}
    Hello
    {:^;:}
:}
</pre>
```

Output:

```html
<pre>
    Hello
</pre>
```

Preserve space:

```html
<div>{:code; Hello :}</div>
<div>{:code; {:;:} Hello :}</div>
<div> {:code; Hello :}</div>
```

Output:

```html
<div>Hello</div>
<div> Hello</div>
<div> Hello</div>
```

Not preserve spaces:

```html
<div>
    {:code;
        Hello
    :}
</div>
<div>{:;
    :}{:code;
        Hello
    :}{:;
:}</div>
```

Output:

```html
<div>
    Hello
</div>
<div>Hello</div>
```
