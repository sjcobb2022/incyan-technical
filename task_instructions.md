### Challenge - write a function that takes a json payload and outputs a dot / asci version of a graph.

1. The graph must be drawn by writing to a console or the equivalent command in the language that you choose - eg println(), console.log(), echo.
1. The output **must** be a **vertical** bar chart 
1. The output does not need to look *exactly* the same as the below, but this is to provide a guide.
1. The output **may** be in colour, but this is not necessary. 
1. Outline any assumptions made about the json payload using code comments.

The goal of this exercise is to assess coding ability and thought processes in a language agnostic way.

Example payload:
```json
{
    "title": "stock count",
    "xtitle": "asset",
    "ytitle": "count",
    "items": [
        {"chairs": 20},
        {"tables": 5},
        {"stands": 7},
        {"lamps": 8},
        {"cups": 10}
    ]
}
```

Example output:
```shell


                  stock count
                  -----------
count
-----
        *
        *
        *
        *
        *
        *                                   *  
        *                 *        *        *  
        *        *        *        *        *  
        *        *        *        *        *  
        *        *        *        *        *  
        chairs   tables   stands   lamps    cups
                                                   asset
                                                   -----

``` 
