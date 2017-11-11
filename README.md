# Otintin

Otintinは静的型付けで動的スコープなプログラミング言語です.  
 "動的言語" という言葉が嫌いなのでつくりました.

# Example

NOTE: Otintinは開発途中であり現段階では以下のコードは動きません  

```
func hoge x: Int {
    x + y
}

func fuga x: Int {
    let y = 1;
    hoge x
}

func main {
    // hoge 1 => type error: unbound variable 'y' in hoge
    let y = 2 in
    hoge@4 // => 6 (2 + 4)
    fuga@4 // => 5 (1 + 4)
}
```

# Copyright
Copyright (C) 2017 akitsu sanae.  
Distributed under the Boost Software License, Version 1.0. 
(See accompanying file LICENSE or copy at http://www.boost/org/LICENSE_1_0.txt)  



