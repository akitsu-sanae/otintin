# Otintin

Otintinは静的型付けで動的スコープなプログラミング言語です.  
 "動的言語" という言葉が嫌いなのでつくりました.

# Example

NOTE: Otintinは開発途中であり現段階では以下のコードは動きません  

```
func hoge x: int {
    x + y
}

func fuga x: int {
    let y = 1;
    hoge(x)
}

func main x: int {
    /*  hoge(1) => type error: unbound variable 'y' in hoge */
    let y = 2;
    hoge(4); /* => 6 (2 + 4) */
    let y = 1;
    fuga(4) /* => 5 (1 + 4) */
}
```

# Copyright
Copyright (C) 2018 akitsu sanae.  
Distributed under the Boost Software License, Version 1.0. 
(See accompanying file LICENSE or copy at http://www.boost/org/LICENSE_1_0.txt)  



