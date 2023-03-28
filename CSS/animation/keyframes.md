## keyframesとは

標準モジュールとしてネイティブなCSSに組み込まれている、  
CSS プロパティの値を時間経過とともにアニメーション化するための仕組み

## どこのドキュメントを親とすべきか

やっぱW3Cだよね
https://www.w3.org/TR/css-animations-1/
https://www.w3.org/TR/css-animations-2/


## First Example

`<div>`要素を（0, 0）から（100px, 100px）まで5秒かけて移動させ、それを9回（合計10回）繰り返すアニメーションを `diagonal-slide`という名前で作成している。

```css
div {
  animation-name: diagonal-slide;
  animation-duration: 5s;
  animation-iteration-count: 10;
}

@keyframes diagonal-slide {

  from {
    left: 0;
    top: 0;
  }

  to {
    left: 100px;
    top: 100px;
  }

}
```


