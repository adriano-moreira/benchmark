using System;
using System.Collections.Generic;
using System.Diagnostics;
using CSharpProject.Modules;

namespace CSharpProject
{
    class Program
    {
        static void Main(string[] args)
        {
            ExecAction(args.Length == 0 ? "hello_world" : args[0]);
        }

        static void ExecAction(String actionName)
        {
            if (_actions.ContainsKey(actionName))
            {
                ExecWithBenchmark(actionName, _actions[actionName]);
            }
            else
            {
                Console.WriteLine($"action '{actionName}' not found");
            }
        }
        
        static Dictionary<String,Action> _actions = new()
        {
                ["nothing"] = Nothing.Exec, 
                ["hello_world"] = HelloWorld.Exec, 
                ["multiply_table"] = MultiplyTable.Exec, 
        };

        static void ExecWithBenchmark(String actionName, Action action)
        {
            var stopwatch = new Stopwatch();
            stopwatch.Start();
            action();
            stopwatch.Stop();
            Console.WriteLine($"action {actionName} elapse {stopwatch.ElapsedTicks / 1000} microseconds");
            //Console.WriteLine($"action {actionName} elapse {stopwatch.ElapsedMilliseconds} milliseconds");
        }
    }
    
}
