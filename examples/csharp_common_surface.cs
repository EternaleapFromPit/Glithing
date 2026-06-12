using static System.Math;

namespace Demo {
    enum Status {
        Unknown,
        Active = 2,
    }

    class Model {
        public Status State;
        public bool Enabled;
        public double Score;

        public Model(Status state) {
            this.State = state;
            this.Enabled = true;
            this.Score = 12.5;
        }

        int StateCode => this.State;
    }

    static int ReadConst() {
        const int value = 3;
        return value;
    }

    int ReadStatus(Status status) {
        int code = 0;
        switch (status) {
            case Status.Active:
                code = 1;
                break;
            default:
                code = 9;
                break;
        }
        return code;
    }

    fn main() {
        Model model = new Model(Status.Active);
        string? title = "ready";
        DateTime now = "2026-05-31";
        Guid id = "00000000-0000-0000-0000-000000000000";
        byte b = 1;
        short s = 2;
        uint u = 3;
        decimal money = 4.5;
        print(model.StateCode);
        print(model.Enabled);
        print(model.Score);
        print(ReadConst());
        print(ReadStatus(model.State));
    }
}
