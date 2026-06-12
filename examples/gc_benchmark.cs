using System;
using System.Collections.Generic;

// 1. Allocate a large live heap to pressure the GC
List<CacheItem> cache = new List<CacheItem>();
int i = 0;
while (i < 1500000) {
    cache.Add(new CacheItem(i));
    i = i + 1;
}

// 2. Perform temporary allocations under live heap pressure
int iteration = 0;
while (iteration < 60000) {
    List<string> list = new List<string>();
    int j = 0;
    while (j < 100) {
        string baseStr = "item_";
        string suffix = "_data";
        string s = baseStr + suffix;
        list.Add(s);
        j = j + 1;
    }
    iteration = iteration + 1;
}

Console.WriteLine(iteration);

class CacheItem {
    public int Value;
    public CacheItem(int v) {
        this.Value = v;
    }
}
