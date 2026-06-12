interface IScored {
    int Score();
}

class BaseCounter {
    public int Seed;

    int GetSeed() {
        return this.Seed;
    }
}

class DerivedCounter : BaseCounter, IScored {
    public int Bonus;

    public DerivedCounter(int seed, int bonus) {
        this.Seed = seed;
        this.Bonus = bonus;
    }

    int Score() {
        int result = 0;
        try {
            result = this.GetSeed() + this.Bonus;
        } catch (Exception ex) {
            result = 0;
        } finally {
            result = result + 1;
        }
        return result;
    }
}

fn main() {
    DerivedCounter counter = new DerivedCounter(4, 5);
    print(counter.Score());
}
