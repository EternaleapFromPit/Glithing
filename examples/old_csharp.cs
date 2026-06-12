using System;
using System.Collections.Generic;
using System.Threading;
using Threads = System.Threading;

struct Point {
    int X;
    int Y;
}

fn main() {
    var x = 10;
    int[] scores = { 7, 8, 9 };
    Point p = new Point { X = x, Y = scores[1] };

    print(p.X);
    print(p.Y);
}
