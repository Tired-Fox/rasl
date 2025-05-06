using System.Runtime.CompilerServices;

namespace Example {
    public class Sample {
        private int count;

        public Sample(int count) {
            this.count = count;
        }

        public int Count() {
            return this.count;
        }

        [MethodImplAttribute(MethodImplOptions.InternalCall)]
        public static extern float Avg(float[] values);
        [MethodImplAttribute(MethodImplOptions.InternalCall)]
        public static extern float Sqrt(float value);
    }
}
