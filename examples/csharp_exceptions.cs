fn main() {
    int code = 0;
    try {
        throw new Exception("boom");
    } catch (Exception ex) {
        print(ex.Message);
        code = 7;
    } finally {
        code = code + 1;
    }
    print(code);
}
