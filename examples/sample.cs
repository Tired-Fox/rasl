using System;
using System.Runtime.CompilerServices;

public class Sample {
    private int count;

    public Sample(int count) {
        this.count = count;
    }

    public int Count() {
        return this.count;
    }

    public void ModifyTest(ref Test test) {
        test.name = "Modify Test";
        test.age = 1000;
    }

    [MethodImplAttribute(MethodImplOptions.InternalCall)]
    public static extern float Avg(float[] values);
    [MethodImplAttribute(MethodImplOptions.InternalCall)]
    public static extern float Sqrt(float value);
}

public struct Test {
    public String name;
    public int age;
}
