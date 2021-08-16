using System;

namespace CSharpProject.Modules
{
    public class MultiplyTable
    {
        public static void Exec()
        {
            Do(1024, 1024);
        }

        private static void Do(int xLength, int yLength)
        {
            var table = new int[xLength, yLength];
            for (var x = 0; x < xLength; x++)
            {
                for (var y = 0; y < yLength; y++)
                {
                    table[x, y] = (x + 1) * (y + 1);
                }
            }

            if (table[0, 0] == 0)
            {
                Console.WriteLine("Ops!");
            }
        }
    }
}
