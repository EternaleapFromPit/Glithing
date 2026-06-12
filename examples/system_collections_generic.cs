using System;
using System.Collections.Generic;
using Collections = System.Collections.Generic;

fn main() {
    System.Collections.Generic.List<int> xs = new System.Collections.Generic.List<int>();
    xs.Add(10);
    xs.Add(20);
    print(xs[1]);

    Collections.Dictionary<string, int> scores = new Collections.Dictionary<string, int>();
    scores.Add("hp", 100);
    print(scores["hp"]);
}
