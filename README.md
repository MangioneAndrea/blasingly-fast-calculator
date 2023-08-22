# Blasingly fast calculator

Yes, it's just a calculator. No, it's not as easy as you think

 <img src="/public/screenshot.png" alt="screenshot" width="200"/>


## Principle

The given string is parsed as set of tokens. A token might be a number, an operation, a parenthesis...

Given the properties and meaning of all tokens, it is possible to see if the given set is valid. If so, the calculation can be done.

If you think `what the hell are you talking about...`, maybe the following representation might be helpful

```
     7 + ( 9 - 2 ) * 7 + 3 * 4
                       +
                   /         \
     7 + ( 9 - 2 ) * 7        3 * 4
       +                        *
     /   \                    /  \ 
    7     ( 9 - 2 ) * 7      3    4
                    *
                  /  \
          ( 9 - 2 )    7
              -
            /   \
           9     2
```

Having built this tree, one can just recursively walk it as each operation seen alone is trivial!

## Compile 

Consider that in order to compile this project, you need rust and node installed.

This project is powered by [tauri](https://tauri.app/). Also it's setup with solid.js and npm. In order to compile the exe, after the usual `npm i`, just run `npm run tauri build`


## But why

- Why so complicated? Couldn't you have just done a simple calculator which after each operation yields the result
    - No... That's the point, sometime you write a formula, but using excel is overkill :D
- Why didn't you use js `eval`? ThAt wOUld Be eaSY anD WiHouT RuST
    1. What's the fun in that...
    2. It [should never be used](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/eval#never_use_eval!) anyway
    3. Any operation can be implemented. Its completelly independent from js
        - E.g. you can use`√` instead of `Math.sqrt(..)`, The user needn't be a developer to use the calculator!
        - E.g. you can implement `|-1|` for absolute values whatsoever

