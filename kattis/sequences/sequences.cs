using System;

public class Program
{
    static long f1(long x, long p) => (x << 1) % p;

    static long f2(long x, int o, long q, long p) => (x + o * q) % p;
    public static void Main()
    {
        const int mod = 1000000007;
        long r = 0;
        int o = 0;
        long q = 500000004;

        string line = Console.ReadLine();
        for (int i = 0; i < line.Length; i++)
        {
            if (line[i] == '?')
            {
                r = f1(r, mod);
                r = f2(r, o, q, mod);

                q = f1(q, mod);

                o += 1;
            }
            else if (line[i] == '0')
            {
                r = f2(r, o, q, mod);
            }
            else
            {
                o += 2;
            }
        }
        Console.WriteLine(r);
    }
}
