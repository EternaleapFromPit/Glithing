using System.Collections.Generic;
using System.Threading.Tasks;

fn main() {
    List<string> names = new List<string>();
    names.Add("Ada");
    print(names[0]);
    print(names.Contains("Ada"));
    print(names.Count);

    List<long> ids = new List<long>();
    ids.Add(42);
    print(ids[0]);
    foreach (long id in ids) {
        print(id);
    }

    Dictionary<string, string> map = new Dictionary<string, string>();
    map.Add("lang", "glitching");
    print(map["lang"]);
    print(map.ContainsKey("lang"));
    print(map.Remove("lang"));

    List<bool> flags = new List<bool>();
    flags.Add(true);
    print(flags[0]);
    print(flags.Contains(true));
    flags.Clear();
    print(flags.Count);

    List<double> ratios = new List<double>();
    ratios.Add(1.5);
    print(ratios[0]);

    Dictionary<string, long> idMap = new Dictionary<string, long>();
    idMap.Add("id", 42);
    print(idMap["id"]);

    Dictionary<string, double> metrics = new Dictionary<string, double>();
    metrics.Add("ratio", 3.14);
    print(metrics["ratio"]);
    print(metrics.Remove("ratio"));

    IEnumerable<string> nameView = names;
    foreach (string name in nameView) {
        print(name);
    }

    Task<int> number = Task.FromResult(42);
    print(number.Result);
}
