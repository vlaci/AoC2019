let usage = """Day 3: Crossed Wires
Usage: 03 <input-file>
       03 --help
Options:
    -h, --help  Print this help.
"""

open System
open System.IO

open DocoptNet
open Xunit
open FsUnit.Xunit

let manhattan path =
    0


module Tests =
    [<Fact>]
    let ``manhattan distance`` () =
        (*
          U7,R6,D4,L4 -> 3
        *)
        manhattan "U7,R6,D4,L4" |> should equal 3

[<EntryPoint>]
let main argv =
    let args = Docopt().Apply(usage, argv, exit = true)
    let inputFile = args.["<input-file>"].ToString()
    let inputs = File.ReadAllText(inputFile).Split(',') |> Array.map (fun s -> Int32.Parse(s))


    0
