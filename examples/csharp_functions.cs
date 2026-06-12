int Add(int a, int b) {
    return a + b;
}

string MakeName() {
    return "Ada";
}

void Consume(List<int> xs) {
    xs.Add(42);
    return;
}

fn main() {
    int value = Add(20, 22);
    print(value);

    string name = MakeName();
    print(name);

    List<int> xs = new List<int>();
    Consume(move xs);
}
