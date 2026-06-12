int ClampSmall(int value) {
    if (value > 10) {
        return 10;
    } else {
        return value;
    }
}

fn main() {
    int total = 0;

    for (int i = 0; i < 4; i = i + 1) {
        total = total + ClampSmall(i);
    }

    while (total < 20) {
        total = total + 1;
    }

    print(total);
}
