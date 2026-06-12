using System.Threading.Tasks;

async Task<int> LoadNumber() {
    return 42;
}

async Task<string> LoadName() {
    return "Ada";
}

fn main() {
    Task<int> numberTask = LoadNumber();
    int value = await numberTask;
    print(value);

    Task<string> nameTask = LoadName();
    string name = await nameTask;
    print(name);
}
