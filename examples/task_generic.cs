using System.Threading.Tasks;

int Compute() {
    return 42;
}

string LoadName() {
    return "Ada";
}

bool IsReady() {
    return true;
}

double LoadRatio() {
    return 1.5;
}

fn main() {
    Task<int> numberTask = Task.Run(Compute);
    int value = numberTask.Result;
    print(value);

    Task<string> nameTask = Task.Run(LoadName);
    string name = nameTask.GetResult();
    print(name);

    Task<bool> readyTask = Task.Run(IsReady);
    bool ready = readyTask.Result;
    print(ready);

    Task<double> ratioTask = Task.Run(LoadRatio);
    double ratio = ratioTask.GetAwaiter().GetResult();
    print(ratio);

    Task<bool> completed = Task.FromResult(true);
    print(completed.IsCompleted);
    print(completed.GetResult());
}
