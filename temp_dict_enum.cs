using System.Collections.Generic;

fn main() {
    Dictionary<string, int> values = new Dictionary<string, int>();
    values.Add("a", 1);
    values.Add("b", 2);

    IEnumerator<KeyValuePair<string, int>> enumerator = values.GetEnumerator();
    while (enumerator.MoveNext()) {
        KeyValuePair<string, int> pair = enumerator.Current;
        print(pair.Key);
        print(pair.Value);
    }
}
