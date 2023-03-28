## URL
https://eslint.org/docs/latest/rules/no-warning-comments#options

## これ何

こういうコメントに対して指摘を投げるやつ

```js
// TODO: do something
// FIXME: this is not a good idea
```

## Options

3つある。
1. `terms`
2. `location`
3. `decoration`

### 1. terms

どういうワードに反応するかを決める。
デフォだと `['todo', 'fixme', 'xxx']`。
因みに大文字/小文字の区別はしない。

これは駄目なパターン。
```js
/*eslint no-warning-comments: "error"*/

/*
FIXME
*/
function callback(err, results) {
  if (err) {
    console.error(err);
    return;
  }
  // TODO
}
```

### 2. location

`terms`で指定したワードが、コメントのどこで使われている時に反応するかを決める。

`'start'`か`'anywhere'`のどちらかの値をとる。

`'start'`...コメントの先頭にある時だけ反応する
`'anywhere'`...コメントのどこに `terms`で定義されたワードが使われていても反応する

例えばこれはNG

```js
/*eslint no-warning-comments: ["error", { "terms": ["todo", "fixme", "any other term"], "location": "anywhere" }]*/

// TODO: this
// todo: this too
// Even this: TODO
/*
 * The same goes for this TODO comment
 * Or a fixme
 * as well as any other term
 */
```

### 3. decoration

`location`が`'start'`の時だけ有効になる。
ちょっと理解しきれていないけど、これはOKらしい

```js
/*eslint no-warning-comments: ["error", { "decoration": ["/", "*"] }]*/

//!TODO preceded by non-decoration character
/**
 *!TODO preceded by non-decoration character in a block comment
 */
```

おそらく、ESLintが`'TODO'`ではなく`'!TODO'`という単語として見るということだろう。