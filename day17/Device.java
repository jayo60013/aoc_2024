import java.util.List;

public class Device {
    public long a;
    public long b;
    public long c;
    public List<Integer> program;

    public Device(final long a, final long b, final long c, final List<Integer> program) {
        this.a = a;
        this.b = b;
        this.c = c;
        this.program = List.copyOf(program);
    }
}